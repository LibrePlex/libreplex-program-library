import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { LibreplexCreator } from "../../../../target/types/libreplex_creator";
import { LibreplexMetadata } from "../../../../target/types/libreplex_metadata";
import { LibreplexNft } from "../../../../target/types/libreplex_nft";
import { ConfirmOptions, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_SLOT_HASHES_PUBKEY, Signer, SystemProgram, TransactionInstruction, sendAndConfirmTransaction } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";
import { struct, u8 } from "@solana/buffer-layout";
import { publicKey } from "@solana/buffer-layout-utils";
import {
  MINT_SIZE, TOKEN_2022_PROGRAM_ID, createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToInstruction
} from "@solana/spl-token"
import { Transaction } from "@solana/web3.js";

type GroupDescriptor = {
  name: string,
  symbol: string,
  description: string,
}

type RoyaltyConfig = {
  bps: number,
  shares: {
    recipient: PublicKey,
    share: number,
  }[]
}

type SetupGroupInput = {
  name: string,
  symbol: string,
  url: string,
  royalties: RoyaltyConfig
  description: string,

  /**
  * The set of possible attributes for metadatas in your collection.
 */
  onChainAttributes?: AttributeType[]

  /**
   * The set of all addresses that are allowed to sign your collection.
   */
  permittedSigners?: PublicKey[]
}

enum TemplateConfig {
  None,
}

const LIBREPLEX_METADATA_PROGRAM_ID = new PublicKey("LibrQsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p")

function getGroupAddress(groupSeed: PublicKey, program = LIBREPLEX_METADATA_PROGRAM_ID) {
  return PublicKey.findProgramAddressSync([Buffer.from("group"), groupSeed.toBuffer()], LIBREPLEX_METADATA_PROGRAM_ID)[0]

}

type AttributeType = {
  name: string,
  possibleValues: (string | BN | number)[]
}

async function setupGroup(
  {
    metadataProgram,
    input,
    groupAuthority,
    groupSeedKp = Keypair.generate()
  }: {
    metadataProgram: Program<LibreplexMetadata>,
    input: SetupGroupInput,
    groupAuthority: PublicKey,
    groupSeedKp?: Keypair
  }
) {
  return metadataProgram.methods.createGroup({
    permittedSigners: input.permittedSigners || [],
    attributeTypes: input.onChainAttributes?.map(v => {
      return {
        permittedValues: v.possibleValues,
        continuedFromIndex: null,
        continuedAtIndex: null,
        deleted: false,
        name: v.name,
      }
    }) || [],
    description: input.description,
    templateConfiguration: {
      none: {},
    },
    name: input.name,
    symbol: input.symbol,
    url: input.url,
    royalties: input.royalties
  }).accounts({
    authority: groupAuthority,
    seed: groupSeedKp.publicKey,
    systemProgram: SystemProgram.programId,
    group: getGroupAddress(groupSeedKp.publicKey),
  });
}



describe("libreplex creator", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LibreplexCreator as Program<LibreplexCreator>;
  const metadataProgram = anchor.workspace.LibreplexMetadata as Program<LibreplexMetadata>;
  const nftProgram = anchor.workspace.LibreplexNft as Program<LibreplexNft>

  console.log(Object.keys(anchor.workspace))

  const authority = anchor.getProvider().publicKey

  it("has minted", async () => {
    const groupSeed = Keypair.generate();
    const group = PublicKey.findProgramAddressSync([Buffer.from("group"), groupSeed.publicKey.toBuffer()], metadataProgram.programId)[0]

    const creatorSeed = Keypair.generate()
    const creator = PublicKey.findProgramAddressSync([Buffer.from("creator"), creatorSeed.publicKey.toBuffer()], program.programId)[0]
    const creatorGroupPermissions = PublicKey.findProgramAddressSync([Buffer.from("permissions"), creator.toBuffer(), group.toBuffer()], metadataProgram.programId)[0]

    console.log("Creating group")

    const grpSetupMethod = await setupGroup({
      metadataProgram,
      groupSeedKp: groupSeed,
      groupAuthority: program.provider.publicKey as PublicKey,
      input: {
        description: "A very cool group",
        name: "COOL GROUP",
        symbol: "COOL",
        url: "COOL.com/group",
        royalties: {
          bps: 100,
          shares: [{
            recipient: program.provider.publicKey as PublicKey,
            share: 100,
          }],
        }
      }
    })
    await grpSetupMethod.rpc({
      skipPreflight: false,
    });

    console.log("Group created")

    await metadataProgram.methods.delegateGroupPermissions({
      permissions: [{
        addToGroup: {}
      }],
    }).accounts({
      group,
      delegatedUser: creator,
      systemProgram: SystemProgram.programId,
      updateAuthority: program.provider.publicKey,
      userPermissions: creatorGroupPermissions,
    }).rpc()

    console.log("Group permissions delegated")

    await program.methods.createCreator({
      attributeMappings: null,
      collection: group,
      description: "The coolest metadatas",
      isOrdered: true,
      maxMints: 2000,
      mintAuthority: program.provider.publicKey as PublicKey,
      name: "COOL #",
      seed: creatorSeed.publicKey,
      symbol: "COOL",
      assetUrl: {
        jsonPrefix: {
          url: "COOL.com/",
        }
      }
    }).accounts({
      creator,
      minterNumbers: null,
      signer: program.provider.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc()

    console.log("Creator initialised")


    const mint = Keypair.generate()
    const metadata = PublicKey.findProgramAddressSync([Buffer.from("metadata"),
    mint.publicKey.toBuffer()], metadataProgram.programId)[0]

    console.log("Creating mint");
    const { transaction, keypair } = await setupMint(program.provider.connection, program.provider.publicKey as PublicKey, program.provider.publicKey as PublicKey, program.provider.publicKey as PublicKey, program.provider.publicKey as PublicKey, 0, mint, metadata, TOKEN_2022_PROGRAM_ID)

    await program.provider.sendAndConfirm?.(transaction, [keypair])

    console.log("Mint created");
    await program.methods.mint().accounts({
      libreplexNftProgram: nftProgram.programId,
      mintWrapper: PublicKey.findProgramAddressSync([mint.publicKey.toBuffer()], nftProgram.programId)[0],
      attributeConfig: null,
      receiverTokenAccount: getAssociatedTokenAddressSync(mint.publicKey, program.provider.publicKey as PublicKey, undefined, TOKEN_2022_PROGRAM_ID),
      creator,
      group,
      groupPermissions: creatorGroupPermissions,
      libreplexMetadataProgram: metadataProgram.programId,
      mint: mint.publicKey,
      creatorAuthority: program.provider.publicKey,
      payer: program.provider.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      mintAuthority: program.provider.publicKey,
      receiver: program.provider.publicKey,

      minterNumbers: null,
      metadata,
      recentSlothashes: SYSVAR_SLOT_HASHES_PUBKEY,
      systemProgram: SystemProgram.programId,
    }).signers([mint]).rpc({
      skipPreflight: true,
    })
  });


});

const MetadataPointerMintSize = 234;

interface InitializeMetadataPointerIx {
  instruction: 39
  metadataPointerInitIx: 0,
  authority: PublicKey,
  metadataAddress: PublicKey,
}

const initializeMetadataPointerInstructionData = struct<InitializeMetadataPointerIx>([
  u8('instruction') as any,
  u8('metadataPointerInitIx'),
  publicKey("authority"),
  publicKey("metadataAddress")
]);


async function setupMint(
  connection: Connection,
  payer: PublicKey,
  receiver: PublicKey,
  mintAuthority: PublicKey,
  freezeAuthority: PublicKey | null,
  decimals: number,
  mintKeypair = Keypair.generate(),
  metadata: PublicKey,
  programId = TOKEN_2022_PROGRAM_ID
) {
  const lamports = await connection.getMinimumBalanceForRentExemption(MetadataPointerMintSize);

  const initMetadataPointerExtensionIx = (() => {
    const initMetadataPointerIxSpan = Buffer.alloc(initializeMetadataPointerInstructionData.span)

    initializeMetadataPointerInstructionData.encode({
      instruction: 39,
      authority: PublicKey.default,
      metadataPointerInitIx: 0,
      metadataAddress: metadata,
    }, initMetadataPointerIxSpan)

    return new TransactionInstruction(
      {
        keys: [
          {
            isSigner: false,
            isWritable: true,
            pubkey: mintKeypair.publicKey
          }
        ],
        programId,
        data: initMetadataPointerIxSpan,
      }
    )
  })();

  const assocTokenAccount = getAssociatedTokenAddressSync(mintKeypair.publicKey, receiver, undefined, TOKEN_2022_PROGRAM_ID);
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: mintKeypair.publicKey,
      space: MetadataPointerMintSize,
      lamports,
      programId,
    }),
    initMetadataPointerExtensionIx,
    createInitializeMint2Instruction(mintKeypair.publicKey, decimals, mintAuthority, freezeAuthority, programId),
    createAssociatedTokenAccountInstruction(payer,
      assocTokenAccount, receiver, mintKeypair.publicKey, TOKEN_2022_PROGRAM_ID),
    createMintToInstruction(mintKeypair.publicKey, assocTokenAccount, mintAuthority, 1, undefined, TOKEN_2022_PROGRAM_ID),
  );

  return {
    transaction,
    keypair: mintKeypair,
  };
}
