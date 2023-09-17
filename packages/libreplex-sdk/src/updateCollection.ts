import { PublicKey } from "@solana/web3.js";
import { Connector } from "./createCollection";
import { loadMetadataProgram } from "./programs";



export async function updateCollectionAuthority(
    {
      connector,
      collection,
      new_authority
    }: {
      connector: Connector,
      collection: PublicKey,
      new_authority: PublicKey,
    }
  ) {
    const metadataProgram = connector.type === "program" ? connector.metadataProgram : await loadMetadataProgram(connector.provider)

    const me = metadataProgram.provider.publicKey;

    if (!me) {
        throw new Error("Provider not setup. Perhaps your wallet is not connected");
    }

    return metadataProgram.methods.updateCollectionAuthority(new_authority).accounts({
       collection,
        updateAuthority: me,
    })
  }
