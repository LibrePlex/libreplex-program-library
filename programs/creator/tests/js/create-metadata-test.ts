import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Libreplex } from "../../../../target/types/libreplex";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";

describe("libreplex", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Libreplex as Program<Libreplex>;
  const authority = anchor.getProvider().publicKey
  const collectionSeed = Keypair.generate();
  const collection = PublicKey.findProgramAddressSync([Buffer.from("collection"), collectionSeed.publicKey.toBuffer()], program.programId)[0]
  const userPermissions = PublicKey.findProgramAddressSync([Buffer.from("permissions"), collection.toBuffer(), authority.toBuffer()], program.programId)[0]

  it("has created a collection and metadata", async () => {
    const createCollectionEventPromise = new Promise<any>((resolve, reject) => {
      program.addEventListener("CreateCollectionEvent", (event, slot, sig) => {
        resolve(event)
      })
    })

    const permissionEventPromise = new Promise<any>((resolve, reject) => {
      program.addEventListener("PermissionEvent", (event, slot, sig) => {
        resolve(event)
      })
    })

    const createMetadataEventPromise = new Promise<any>((resolve, reject) => {
      program.addEventListener("CreateMetadataEvent", (event, slot, sig) => {
        resolve(event)
      })
    })

    const collectionName = "COOL COLLECTION"

    const tx = await program.methods.createCollection({
      collectionUrl: "COOL.com",
      name: collectionName,
      symbol: "COOL",
      nftCollectionData: null,
    }).accounts({
      authority,
      seed: collectionSeed.publicKey,
      collection,
      systemProgram: SystemProgram.programId,
      userPermissions
    }).rpc();

    console.log("Your transaction signature", tx);
    const createCollectionEvent = await createCollectionEventPromise
    const permissionEvent = await permissionEventPromise

    expect(createCollectionEvent).to.deep.equal({
      creator: authority,
      name: collectionName,
      id: collection,
    })

    expect(permissionEvent).to.deep.equal({
      collection,
      user: authority,
      eventType: {
        update: {},
      }
    })


    console.log("Here")
    const mint = Keypair.generate()
    const metadata = PublicKey.findProgramAddressSync([Buffer.from("metadata"), mint.publicKey.toBuffer()], program.programId)[0]
    const metadataName = "COOLMETA"

    await program.methods.createMetadata({
      metadataUrl: "COOLURL.com",
      name: metadataName,
      nftMetadata: null,
    }).accounts({
      mint: mint.publicKey,
      collection,
      metadata,
      systemProgram: SystemProgram.programId,
      signer: authority, 
      signerCollectionPermissions: userPermissions,
    }).signers([mint]).rpc()

    const createMetadataEvent = await createMetadataEventPromise

    expect(createMetadataEvent).to.deep.equal({
      id: metadata,
      collection,
      mint: mint.publicKey,
      name: metadataName,
    })
  });
});
