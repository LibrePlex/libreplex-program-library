import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LibreplexCreator } from "../../../../target/types/libreplex_creator";
import { LibreplexMetadata } from "../../../../target/types/libreplex_metadata";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_SLOT_HASHES_PUBKEY, SystemProgram } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";

import {createMint} from "@solana/spl-token"
import { Transaction } from "@solana/web3.js";

describe("libreplex creator", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LibreplexCreator as Program<LibreplexCreator>;
  const metadataProgram = anchor.workspace.LibreplexMetadata as Program<LibreplexMetadata>;

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
          recipient: program.provider.publicKey,
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
      mintAuthority: program.provider.publicKey,
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
    await program.provider.sendAndConfirm(new Transaction().add(SystemProgram.createAccount({
      fromPubkey: program.provider.publicKey,
      lamports: LAMPORTS_PER_SOL,
      newAccountPubkey: payer.publicKey,
      programId: SystemProgram.programId,
      space: 0,
    })), [payer])
  
    const mint = Keypair.generate()

    await createMint(program.provider.connection, payer, program.provider.publicKey,  program.provider.publicKey, 0, mint)


    const metadata = PublicKey.findProgramAddressSync([Buffer.from("metadata"), 
      mint.publicKey.toBuffer()], metadataProgram.programId)[0]
    const metadataExtension = PublicKey.findProgramAddressSync([Buffer.from("metadata_extension"), metadata.toBuffer()], metadataProgram.programId)[0]

    await program.methods.mint().accounts({
      attributeConfig: null,
      buyer: program.provider.publicKey,
      creator,
      group,
      groupPermissions: creatorGroupPermissions,
      libreplexMetadataProgram: metadataProgram.programId,
      mint: mint.publicKey,
      mintAuthority: program.provider.publicKey,
      minterNumbers: null,
      metadata,
      recentSlothashes: SYSVAR_SLOT_HASHES_PUBKEY,
      systemProgram: SystemProgram.programId,
      metadataExtension,
    }).signers([mint]).rpc({
      skipPreflight: true,
    })
  });


});
