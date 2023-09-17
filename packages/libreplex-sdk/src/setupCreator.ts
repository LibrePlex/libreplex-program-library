import { BN, Program } from "@coral-xyz/anchor"
import {LibreplexCreator} from "@libreplex/idls/lib/types/libreplex_creator"
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata"
import {LibreplexCreatorControls} from "@libreplex/idls/lib/types/libreplex_creator_controls"

import { Keypair, PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";
import {getCreatorAddress, getCreatorControllerAddress} from "./pda"
import {UserPermission, setUserPermissionsForGroup} from "./groupPermissions"
import { CreatorControl, controlToAnchor } from "./creatorControls";
import { LIBREPLEX_CREATOR_PROGRAM_ID } from "./constants";

export type SetupCreatorData = {
    description: string,
    baseName: string,
    ordered: boolean,
    supply: number,
    symbol: string,
    baseUrl: {
        type: "json-prefix",
        url: string,
    } | {
        type: "chain-renderer",
        programId: PublicKey,
        description?: string,
    },
}

export type SetupCreatorInput = {
    mintAuthority: PublicKey,
    program: Program<LibreplexCreator>,
    metadataProgram: Program<LibreplexMetadata>,
    collection: PublicKey,
    creatorData: SetupCreatorData,
}



export type Phase = {
    start: Date,
    end: Date | null,
    label: string,
    control: CreatorControl[]
}

export async function setupCreatorWithCustomSalePhases(
    input: SetupCreatorInput, 
    creatorControllerProgram: Program<LibreplexCreatorControls>,
    salePhases: Phase[], 
    checkGroupIsValid = true) {
        const me = creatorControllerProgram.provider.publicKey
        const setupCreatorCtx = await setupCreator(input, checkGroupIsValid)

        const creatorControllerSeed = Keypair.generate()

        const anchorPhases = salePhases.map(p => {
            const anchorControls = p.control.map(controlToAnchor)

            return {
                start: new BN(Math.floor(p.start.getTime() / 1000)),
                end: p.end != null ? new BN(Math.floor(p.end.getTime() / 1000)) : null,
                label: p.label,
                controls: anchorControls,
            }
        });

        const creatorController = getCreatorControllerAddress(creatorControllerSeed.publicKey)

        const controllerCtx = await creatorControllerProgram.methods.initialize({
            seed: creatorControllerSeed.publicKey,
            phases: anchorPhases
        }).accounts({
            creator: setupCreatorCtx.creator,
            creatorController,
            libreplexCreatorProgram: LIBREPLEX_CREATOR_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            updateAuthority: me,
            payer: me,
        }).prepare()

        const method = setupCreatorCtx.method.postInstructions([controllerCtx.instruction])

        return {
            method,
            creatorController,
            creator: setupCreatorCtx.creator,
            minterNumbers: setupCreatorCtx.minterNumbers,
        }
}


export async function setupCreator(input: SetupCreatorInput, checkGroupIsValid = true) {
    const {program, collection, creatorData, mintAuthority, metadataProgram} = input;
    const {description, baseName, ordered, supply, symbol, baseUrl} = creatorData;

    const me = program.provider.publicKey

    if (checkGroupIsValid) {
        const groupAccount = await metadataProgram.account.collection.fetchNullable(collection)

        if (!groupAccount) {
            throw new Error("Provided group does not exist")
        }


        if (groupAccount.updateAuthority.toString() !== me?.toString()) {
            throw new Error(`You do not have authority over the provided group. 
                ${groupAccount.updateAuthority.toString()} ${me?.toString()}`)
        }
    }

    const creatorSeed = Keypair.generate()
    const creator = getCreatorAddress(creatorSeed.publicKey);

    const preIx: TransactionInstruction[] = []
    const signers: Keypair[] = []
    let minterNumbers: PublicKey | null = null

    if (!ordered) {
        const minterNumbersKp = Keypair.generate()
        const minterNumbersSize = 8 + 32 + 4 * supply
        const rent = await program.provider.connection.getMinimumBalanceForRentExemption(minterNumbersSize, "confirmed")

        const creatorMinterNumbersIx = SystemProgram.createAccount({
            fromPubkey: program.provider.publicKey as PublicKey,
            lamports: rent,
            newAccountPubkey: minterNumbersKp.publicKey,
            programId: program.programId,
            space: minterNumbersSize,
        })

        minterNumbers = minterNumbersKp.publicKey

        preIx.push(creatorMinterNumbersIx)
        signers.push(minterNumbersKp)
    }


    let createCreatorMethod = await program.methods.createCreator({
        attributeMappings: null,
        collection,
        description,
        isOrdered: ordered,
        maxMints: supply,
        mintAuthority,
        name: baseName,
        seed: creatorSeed.publicKey,
        symbol: symbol,
        assetUrl: baseUrl.type === "json-prefix" ? {
            jsonPrefix: {
              url: baseUrl.url,
            }
          } : {
            chainRenderer: {
                programId: baseUrl.programId
            }
          }
      }).accounts({
        creator,
        minterNumbers,
        signer: program.provider.publicKey,
        systemProgram: SystemProgram.programId,
      }).preInstructions(preIx).signers(signers)

      const delegateToGroupMethod = await (await setUserPermissionsForGroup({
        collection,
        groupUpdateAuthority: me as PublicKey,
        user: creator,
        connector: {
            type: "program",
            metadataProgram,
        },
        permissions: [UserPermission.AddToGroup]
      })).prepare()
      
      createCreatorMethod = createCreatorMethod.postInstructions([delegateToGroupMethod.instruction])



      return {
        method: createCreatorMethod,
        creator,
        minterNumbers
      }
}