import { Program, BN, Provider } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata"
import { getGroupAddress } from "./pda";
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


export type Connector = {
  type: "provider",
  provider: Provider,
} | {
  type: "program",
  metadataProgram: Program<LibreplexMetadata>,
}


export async function setupGroup(
    groupInfo: {
      connector: Connector,
      input: SetupGroupInput,
      groupAuthority: PublicKey,
      groupSeedKp?: Keypair
    }
  ) {
    const {
      connector,
      input,
      groupAuthority,
      groupSeedKp = Keypair.generate()
    } = groupInfo
    const group = getGroupAddress(groupSeedKp.publicKey)

    const metadataProgram = connector.type === "program" ? connector.metadataProgram : await loadMetadataProgram(connector.provider)

    return {
      method: metadataProgram.methods.createGroup({
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
        group,
      }),

      group,
    }
  }
  