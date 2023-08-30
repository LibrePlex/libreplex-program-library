import {LibreplexCreator} from "@libreplex/idls/lib/types/libreplex_creator"
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata"
import {LibreplexNft} from "@libreplex/idls/lib/types/libreplex_nft"
import {LibreplexCreatorControls} from "@libreplex/idls/lib/types/libreplex_creator_controls"
import { Program, AccountClient , IdlAccounts, IdlTypes, Provider} from "@coral-xyz/anchor"
import { LIBREPLEX_METADATA_PROGRAM_ID, LIBREPLEX_NFT_PROGRAM_ID } from "./constants"

export async function loadMetadataProgram(provider: Provider) {
    return new Program<LibreplexMetadata>((await import("@libreplex/idls/lib/cjs/libreplex_metadata")).IDL, 
        LIBREPLEX_METADATA_PROGRAM_ID, provider)
}

export async function loadNftProgram(provider: Provider) {
    return new Program<LibreplexNft>((await import("@libreplex/idls/lib/cjs/libreplex_nft")).IDL, 
        LIBREPLEX_NFT_PROGRAM_ID, provider)
}
