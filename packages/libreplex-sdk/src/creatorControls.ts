import {BN, IdlAccounts, IdlTypes} from "@coral-xyz/anchor"
import { PublicKey } from "@solana/web3.js"
import {LibreplexCreatorControls} from "@libreplex/idls/lib/types/libreplex_creator_controls"

export type CreatorControl = SolPaymentControl | 
    MintLimitControl |
    SplPaymentControl |
    AllowListControl |
    CustomProgramControl

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


export type SplPaymentControl = {
    name: "SplPayment",
    amount: BN,
    mint: PublicKey,
    recepient: PublicKey,
    tokenProgram: PublicKey,
}

export type AllowListControl = {
    label: string,
    name: "AllowList",
    merkleRoot: number[],
}


type CustomProgramAccountMetaKeySeed = {
    type: "bytes",
    value: Buffer
} | {
    type: "mintPlaceHolder",
} | {
    type: "receiverPlaceHolder",
} | {
    type: "payerPlaceHolder",
}

type CustomProgramAccountMetaKey = {
    type: "key",
    value: PublicKey,
} | {
    type: "seedDerivation",
    programId: PublicKey,
    seeds: CustomProgramAccountMetaKeySeed[],
}

export type CustomProgramControl = {
    label: string,
    name: "CustomProgram",
    programId: PublicKey,
    instructionData: Buffer,
    remainingAccountsMetas: {
        isSigner: boolean,
        isWritable: boolean,
        key: CustomProgramAccountMetaKey,
    }[],
}

export function anchorToControl(c: IdlAccounts<LibreplexCreatorControls>["creatorController"]["phases"][0]["controls"][0]): CreatorControl {
    if (c.allowList) {
        return {
            name: "AllowList",
            label: c.allowList[0].label,
            merkleRoot: c.allowList[0].root
        }
    }

    if (c.customProgram) {
        return {
            name: "CustomProgram",
            instructionData: c.customProgram[0].instructionData,
            label: c.customProgram[0].label,
            programId: c.customProgram[0].programId,
            remainingAccountsMetas: c.customProgram[0].remainingAccountMetas.map(m => {
                let anchorKey: IdlTypes<LibreplexCreatorControls>["CustomProgramAcountMetaKey"] = m.key as any;

                let key: CustomProgramAccountMetaKey;
                if (anchorKey.pubkey) {
                    key = {
                        type: "key",
                        value: anchorKey.pubkey[0]
                    }
                }
                else if (anchorKey.derivedFromSeeds) {
                    const anchorSeeds = anchorKey.derivedFromSeeds[0].seeds;

                    const seeds: CustomProgramAccountMetaKeySeed[] = []
                    for (let anchorSeed of anchorSeeds) {
                        if (anchorSeed.bytes) {
                            seeds.push({
                                type: "bytes",
                                value: anchorSeed.bytes[0]
                            })
                        }
                        else if (anchorSeed.mintPlaceHolder) {
                            seeds.push({
                                type: "mintPlaceHolder",
                            })
                        }
                        else if (anchorSeed.payerPlaceHolder) {
                            seeds.push({
                                type: "payerPlaceHolder",
                            })
                        }
                        else if (anchorSeed.receiverPlaceHolder) {
                            seeds.push({
                                type: "receiverPlaceHolder",
                            })
                        }
                        else {
                            throw new Error("Unsupported custom program anchor seed.")
                        }
                    }

                    key = {
                        type: "seedDerivation",
                        seeds,
                        programId: anchorKey.derivedFromSeeds[0].programId,
                    }
                }
                else {
                    throw new Error("Unsupported custom program anchor key.")
                }
                
             
                return {
                    isSigner: m.isSigner,
                    isWritable: m.isWritable,
                    key,
                }   
            }),
        }
    }

    if (c.mintLimit) {
        return {
            name: "MintLimitControl",
            amount: c.mintLimit[0].limit,
            extraSeeds: c.mintLimit[0].accountKey,
            scopedToBuyer: c.mintLimit[0].scopedToBuyer,
        }
    }

    if (c.payment) {
        const control = c.payment[0];

        return {
            name: "SolPayment",
            price: control.amount,
            receiver: control.recepient
        }
    }

    if (c.splPayment) {
        const control = c.splPayment[0]

        return {
            name: "SplPayment",
            amount: control.amount,
            mint: control.mint,
            recepient: control.recepient,
            tokenProgram: control.tokenProgram
        }
    }

    throw new Error("Tried to convert invalid anchor control")
}

export function controlToAnchor(c: CreatorControl): IdlTypes<LibreplexCreatorControls>["ControlType"] {
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

            const remainingAccountMetas: IdlTypes<LibreplexCreatorControls>["CustomProgramAccountMeta"][] = []    

            for (let meta of c.remainingAccountsMetas) {
                let key: IdlTypes<LibreplexCreatorControls>["CustomProgramAcountMetaKey"];

                switch  (meta.key.type) {
                    case "key":
                        key = {
                            pubkey: {
                                "0": meta.key.value,
                            }
                        }
                        break;
                    case "seedDerivation":
                        const seeds: IdlTypes<LibreplexCreatorControls>["Seed"][] = []

                        for (const seed of meta.key.seeds) {
                            switch (seed.type) {
                                case "bytes":
                                    seeds.push({
                                        bytes: {
                                            "0": seed.value
                                        }
                                    })
                                    break;
                                case "mintPlaceHolder":
                                    seeds.push({
                                        mintPlaceHolder: {}
                                    })
                                    break;
                                case "receiverPlaceHolder":
                                    seeds.push({
                                        receiverPlaceHolder: {}
                                    })
                                    break;
                                case "payerPlaceHolder":
                                    seeds.push({
                                        payerPlaceHolder: {}
                                    })
                                    break;

                                default:
                                    throw new Error(`Invalid seed ${seed}`)
                            }
                        }

                        key = {
                            derivedFromSeeds: {
                                "0": {
                                    programId: meta.key.programId,
                                    seeds,
                                }
                            }
                        }
                        break;
                    default: 
                        throw new Error(`Invalid custom program account meta key type. ${meta.key}`)
                }

                remainingAccountMetas.push({
                    isSigner: meta.isSigner,
                    isWritable: meta.isWritable,
                    key,
                })
            }

            return {
                customProgram: {
                    0: {
                       ...c,
                       remainingAccountMetas
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