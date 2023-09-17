import { Program, BN, Provider } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata"
import { getCollectionAddress } from "./pda";
import { loadMetadataProgram } from "./programs";

type AttributeType = {
  name: string,
  possibleValues: (string | BN | number)[]
}

export type RoyaltyConfig = {
  bps: number,
  shares: {
    recipient: PublicKey,
    share: number,
  }[]
}

type SetupCollectionInput = {
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


export type Connector = {
  type: "provider",
  provider: Provider,
} | {
  type: "program",
  metadataProgram: Program<LibreplexMetadata>,
}


export async function setupCollection(
    groupInfo: {
      connector: Connector,
      input: SetupCollectionInput,
      collectionAuthority: PublicKey,
      groupSeedKp?: Keypair
    }
  ) {
    const {
      connector,
      input,
      collectionAuthority,
      groupSeedKp = Keypair.generate()
    } = groupInfo
    const collection = getCollectionAddress(groupSeedKp.publicKey)

    const metadataProgram = connector.type === "program" ? connector.metadataProgram : await loadMetadataProgram(connector.provider)

    return {
      method: metadataProgram.methods.createCollection({
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
        name: input.name,
        symbol: input.symbol,
        url: input.url,
        royalties: input.royalties
      }).accounts({
        authority: collectionAuthority,
        seed: groupSeedKp.publicKey,
        systemProgram: SystemProgram.programId,
        collection,
      }),

      collection,
    }
  }
  