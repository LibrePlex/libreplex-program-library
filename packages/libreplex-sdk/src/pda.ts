import { PublicKey } from "@solana/web3.js";
import { LIBREPLEX_METADATA_PROGRAM_ID, LIBREPLEX_CREATOR_PROGRAM_ID, LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID, LIBREPLEX_NFT_PROGRAM_ID } from "./constants";



export function getCollectionAddress(collectionSeed: PublicKey, program = LIBREPLEX_METADATA_PROGRAM_ID) {
    return PublicKey.findProgramAddressSync([Buffer.from("collection"), collectionSeed.toBuffer()], LIBREPLEX_METADATA_PROGRAM_ID)[0]
  
  }
  
export function getCreatorAddress(seed: PublicKey, program = LIBREPLEX_CREATOR_PROGRAM_ID) {
    return PublicKey.findProgramAddressSync([Buffer.from("creator"), seed.toBuffer()], program)[0]

}

export function getCreatorControllerAddress(seed: PublicKey, programId: PublicKey = LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID) {
    return PublicKey.findProgramAddressSync([seed.toBuffer()], programId)[0]
}

export function getMetadataAddress(mint: PublicKey, program = LIBREPLEX_METADATA_PROGRAM_ID) {
    return PublicKey.findProgramAddressSync([Buffer.from("metadata"),
        mint.toBuffer()], program)[0]
}

export function getMintWrapperAddress(mint: PublicKey, program = LIBREPLEX_NFT_PROGRAM_ID) {
    return PublicKey.findProgramAddressSync([mint.toBuffer()], program)[0]
} 