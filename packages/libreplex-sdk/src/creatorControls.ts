import {BN} from "@coral-xyz/anchor"
import { PublicKey } from "@solana/web3.js"

export type CreatorControl = SolPaymentControl | 
    MintLimitControl |
    SplPayment |
    AllowList |
    CustomProgram

export type SolPaymentControl = {
    name: "SolPayment",
    price: BN,
    receiver: PublicKey,
}

export type MintLimitControl = {
    name: "MintLimitControl",

    amount: number,

    // Is this a global limit or does each buy have their own limit
    scopedToBuyer: boolean,

    extraSeeds: PublicKey[]
}


export type SplPayment = {
    name: "SplPayment",
    amount: BN,
    mint: PublicKey,
    recepient: PublicKey,
    tokenProgram: PublicKey,
}

export type AllowList = {
    label: string,
    name: "AllowList",
    merkleRoot: number[],
}

export type CustomProgram = {
    label: string,
    name: "CustomProgram",
    programId: PublicKey,
    instructionData: Buffer,
    remainingAccountsToUse: number,
}

export function controlToAnchor(c: CreatorControl) {
    switch (c.name) {
        case "SolPayment":
            return {
                payment: {
                    0: {
                        amount: c.price,
                        recepient: c.receiver,
                    }
                }
            }

        case "AllowList":
            return {
                allowList: {
                    "0": {
                        root: c.merkleRoot,
                        label: c.label,
                    }
                }
            }

        case "CustomProgram":
            return {
                customProgram: {
                    0: {
                        programId: c.programId,
                        remainingAccountsToUse: c.remainingAccountsToUse,
                        instructionData: c.instructionData,
                        label: c.label,
                    }
                }
            }

        case "MintLimitControl":
            return {
                mintLimit: {
                    "0": {
                        limit: c.amount,
                        scopedToBuyer: c.scopedToBuyer,
                        accountKey: c.extraSeeds,
                    }
                }
            }

        case "SplPayment":
            return {
                splPayment: {
                    "0": c
                }
            }

        default:
            throw new Error("Invalid control")
    };
}