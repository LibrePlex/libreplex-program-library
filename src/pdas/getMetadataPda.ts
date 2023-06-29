import { PublicKey } from "@solana/web3.js";

import { METADATA } from "./constants";
import { PROGRAM_ID_METADATA } from "../program/getProgramInstance";



export const getMetadataPda = (mint: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(METADATA), mint.toBuffer()],
    new PublicKey(PROGRAM_ID_METADATA)
  );
};
