import { PublicKey } from '@solana/web3.js';
import { MetadataInput } from '../../metadata/metadata.client'

export interface MetadataInputCli {
    collectionDataPubkey: PublicKey,
    mint: string,
    metadataInput: MetadataInput,
}

export const metadataInputCli: MetadataInputCli = {
    collectionDataPubkey: new PublicKey("nD9NvcnQhxvv5sor6s5KUz2BnczRjygAR3V54w9fuXB"),
    mint: "/home/charalambos/.config/solana/devnet-libreplex/mint1.json",
    metadataInput: {
        name: "GENESIS 0000",
        symbol: "GZ 0000",
        metadataUrl: "https://thegenesis/0000/",
        nftMetadata: {
            attributes: [
                {
                    traitType: "Background",
                    attribute: "Frozen Hell",
                },
                {
                    traitType: "Clothing",
                    attribute: "Emerald Tuxedo",
                },
                {
                    traitType: "Hair",
                    attribute: "Wavy",
                },
                {
                    traitType: "Mouth",
                    attribute: "Smirk",
                }
            ],
            signers : [
                new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh")
            ]
        }
    }
}
