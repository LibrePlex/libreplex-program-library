
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";
import { LibreplexMetadata } from "../../../../target/types/libreplex_metadata";

describe("libreplex", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LibreplexMetadata as Program<LibreplexMetadata>;
  const authority = anchor.getProvider().publicKey
  const collectionSeed = Keypair.generate();
  const group = PublicKey.findProgramAddressSync([Buffer.from("group"), collectionSeed.publicKey.toBuffer()], program.programId)[0]
  const userPermissions = PublicKey.findProgramAddressSync([Buffer.from("permissions"), group.toBuffer(), authority.toBuffer()], program.programId)[0]

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

    const tx = await program.methods.createGroup({
      url: "COOL.com",
      name: collectionName,
      symbol: "COOL",
      description: "",
      metadataRenderMode: undefined,
      royalties: undefined,
      attributeTypes: [],
      permittedSigners: []
    }).accounts({
      authority,
      seed: collectionSeed.publicKey,
      group,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("Your transaction signature", tx);
    const createCollectionEvent = await createCollectionEventPromise
    const permissionEvent = await permissionEventPromise

    expect(createCollectionEvent).to.deep.equal({
      creator: authority,
      name: collectionName,
      id: group,
    })

    expect(permissionEvent).to.deep.equal({
      group,
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
      name: metadataName,
      symbol: "",
      asset: undefined,
      description: "",
      updateAuthority: Keypair.generate().publicKey
    }).accounts({
      mint: mint.publicKey,
      metadata,
      systemProgram: SystemProgram.programId,
      signer: authority, 
    }).signers([mint]).rpc()

    const createMetadataEvent = await createMetadataEventPromise

    expect(createMetadataEvent).to.deep.equal({
      id: metadata,
      group,
      mint: mint.publicKey,
      name: metadataName,
    })
  });
});
