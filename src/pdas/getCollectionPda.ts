
import { PublicKey } from "@solana/web3.js";

import { GROUP } from "./constants";
import { PROGRAM_ID_METADATA } from "../program/getProgramInstance";


export const getGroupPda = (seed: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(GROUP), seed.toBuffer()],
    new PublicKey(PROGRAM_ID_METADATA)
  );
};
