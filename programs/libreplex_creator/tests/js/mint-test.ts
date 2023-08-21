import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LibreplexCreator } from "../../../../target/types/libreplex_creator";
import { LibreplexMetadata } from "../../../../target/types/libreplex_metadata";
import { LibreplexNft } from "../../../../target/types/libreplex_nft";
import { ConfirmOptions, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_SLOT_HASHES_PUBKEY, Signer, SystemProgram, TransactionInstruction, sendAndConfirmTransaction } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";
import {struct, u8} from "@solana/buffer-layout";
import {publicKey} from "@solana/buffer-layout-utils";
import {MINT_SIZE, TOKEN_2022_PROGRAM_ID, createInitializeMint2Instruction, getMinimumBalanceForRentExemptMint} from "@solana/spl-token"
import { Transaction } from "@solana/web3.js";

type GroupDescriptor = {
  name: string,
  symbol: string,
  description: string,
}

describe("libreplex creator", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LibreplexCreator as Program<LibreplexCreator>;
  const metadataProgram = anchor.workspace.LibreplexMetadata as Program<LibreplexMetadata>;
  const nftProgram = anchor.workspace.LibreplexNft as Program<LibreplexNft>

  console.log(Object.keys(anchor.workspace))

  const authority = anchor.getProvider().publicKey

  const mint = Keypair.generate()
  

  it("has minted", async () => {
    const groupSeed = Keypair.generate();
    const group = PublicKey.findProgramAddressSync([Buffer.from("group"), groupSeed.publicKey.toBuffer()], metadataProgram.programId)[0]
    
    const creatorSeed = Keypair.generate()
    const creator = PublicKey.findProgramAddressSync([Buffer.from("creator"), creatorSeed.publicKey.toBuffer()], program.programId)[0]
    const creatorGroupPermissions = PublicKey.findProgramAddressSync([Buffer.from("permissions"), creator.toBuffer(), group.toBuffer()], metadataProgram.programId)[0]


    console.log("Creating group")
    await metadataProgram.methods.createGroup({
      permittedSigners: [],
      attributeTypes: [],
      description: "A very cool group",
      templateConfiguration: {
        none: {},
      },
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
    }).accounts({
      authority: program.provider.publicKey,
      seed: groupSeed.publicKey,
      systemProgram: SystemProgram.programId,
      group,
    }).rpc({
      skipPreflight: true,
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

    const payer = Keypair.generate()
    await program.provider.sendAndConfirm?.(new Transaction().add(SystemProgram.createAccount({
      fromPubkey: program.provider.publicKey as PublicKey,
      lamports: LAMPORTS_PER_SOL,
      newAccountPubkey: payer.publicKey,
      programId: SystemProgram.programId,
      space: 0,
    })), [payer])
  
    const mint = Keypair.generate()
    const metadata = PublicKey.findProgramAddressSync([Buffer.from("metadata"), 
      mint.publicKey.toBuffer()], metadataProgram.programId)[0]

    await createMint(program.provider.connection, payer, program.provider.publicKey as PublicKey,  program.provider.publicKey as PublicKey, 0, mint, metadata, undefined, TOKEN_2022_PROGRAM_ID)


    const metadataExtension = PublicKey.findProgramAddressSync([Buffer.from("metadata_extension"), metadata.toBuffer()], metadataProgram.programId)[0]

    await program.methods.mint().accounts({
      libreplexNftProgram: nftProgram.programId,

      attributeConfig: null,
  
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


async function createMint(
  connection: Connection,
  payer: Signer,
  mintAuthority: PublicKey,
  freezeAuthority: PublicKey | null,
  decimals: number,
  keypair = Keypair.generate(),
  metadata: PublicKey,
  confirmOptions?: ConfirmOptions,
  programId = TOKEN_2022_PROGRAM_ID
): Promise<PublicKey> {
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
            pubkey: keypair.publicKey
          }
        ],
        programId,
        data: initMetadataPointerIxSpan,
      }
    )
  })();

  const transaction = new Transaction().add(
      SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: keypair.publicKey,
          space: MetadataPointerMintSize,
          lamports,
          programId,
      }),
      initMetadataPointerExtensionIx,
      createInitializeMint2Instruction(keypair.publicKey, decimals, mintAuthority, freezeAuthority, programId)
  );

  await sendAndConfirmTransaction(connection, transaction, [payer, keypair], confirmOptions);

  return keypair.publicKey;
}
