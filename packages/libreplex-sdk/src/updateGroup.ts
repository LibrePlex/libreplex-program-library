import { PublicKey } from "@solana/web3.js";
import { Connector } from "./createGroup";
import { loadMetadataProgram } from "./programs";



export async function updateGroupAuthority(
    {
      connector,
      group,
      new_authority
    }: {
      connector: Connector,
      group: PublicKey,
      new_authority: PublicKey,
    }
  ) {
    const metadataProgram = connector.type === "program" ? connector.metadataProgram : await loadMetadataProgram(connector.provider)

    const me = metadataProgram.provider.publicKey;

    if (!me) {
        throw new Error("Provider not setup. Perhaps your wallet is not connected");
    }

    return metadataProgram.methods.updateGroupAuthority(new_authority).accounts({
        group,
        updateAuthority: me,
    })
  }
