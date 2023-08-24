import { Program, BN } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata"
import { getGroupAddress } from "./pda";

type AttributeType = {
  name: string,
  possibleValues: (string | BN | number)[]
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



export async function setupGroup(
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
    const group = getGroupAddress(groupSeedKp.publicKey)

    return {
      method: await metadataProgram.methods.createGroup({
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
  