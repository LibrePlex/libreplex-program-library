import { Keypair, PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";
import { Phase } from "./setupCreator";
import {controlToAnchor} from "./creatorControls"
import {LibreplexCreatorControls} from "@libreplex/idls/lib/types/libreplex_creator_controls"
import { Program, AccountClient , IdlAccounts, IdlTypes} from "@coral-xyz/anchor"

export type UpdateCreatorInput = {
    program: Program<LibreplexCreatorControls>,
    creatorController: PublicKey,
    phases: Phase[],
}


export async function updateCreator(input: UpdateCreatorInput) {
    const {program, phases, creatorController} = input;

    const me = program.provider.publicKey
 
    if (!me) {
        throw new Error("Missing provider. Are you sure your wallet is connected?")
    }



    const anchorPhases: IdlTypes<LibreplexCreatorControls>["Phase"][] = [];

    for (const ph of phases) {
        const controls: IdlTypes<LibreplexCreatorControls>["ControlType"][] = []

        for (const control of ph.control) {
            controls.push(controlToAnchor(control))
        }

        anchorPhases.push({
            controls,
            end: ph.end ? ph.end.getTime()/1000 : null,
            label: ph.label,
            start: ph.start.getTime()/1000,
        })
    }

    return program.methods.update({
        phases: anchorPhases,
    }).accounts({
        systemProgram: SystemProgram.programId,
        payer: me,
        updateAuthority: me,
        creatorController,
    });
}