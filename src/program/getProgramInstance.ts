import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";

export const PROGRAM_ID_METADATA =
  "LibrQsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p";
export const PROGRAM_ID_INSCRIPTIONS =
  "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp";

export const PROGRAM_ID_LISTINGS =
  "ListjawGEdhxuAErSyYwcTEGWQswFoi6FScnGG1RKSB"

import { IDL as IDLMetadata, LibreplexMetadata } from "../../target/types/libreplex_metadata";
import { IDL as IDLOrdinals, Inscriptions } from "../../target/types/inscriptions";

type ArrayElement<ArrayType extends readonly unknown[]> =
  ArrayType extends readonly (infer ElementType)[] ? ElementType : never;

export type LibreplexWithOrdinals = {
  version: string,
  name: string,
  instructions: (
    | ArrayElement<LibreplexMetadata["instructions"]>
    | ArrayElement<Inscriptions["instructions"]>
  )[];
  accounts: (
    | ArrayElement<LibreplexMetadata["accounts"]>
    | ArrayElement<Inscriptions["accounts"]>
  )[];
  types: (
    | ArrayElement<LibreplexMetadata["types"]>
    | ArrayElement<Inscriptions["types"]>
  )[];
};
export function getProgramInstance(
  connection: Connection,
  wallet: anchor.Wallet
) {
  if (!wallet.publicKey) return;
  const provider = new anchor.AnchorProvider(
    connection,
    wallet,
    anchor.AnchorProvider.defaultOptions()
  );
  // Read the generated IDL.
  const idl: LibreplexWithOrdinals = {
    ...IDLMetadata,
    ...IDLOrdinals,
    instructions: [...IDLMetadata.instructions, ...IDLOrdinals.instructions],
    accounts: [...IDLMetadata.accounts, ...IDLOrdinals.accounts],
    types: [...IDLMetadata.types, ...IDLOrdinals.types],
  };
  // Address of the deployed program.
  const programId = PROGRAM_ID_METADATA;
  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId, provider);
  return program;
}
