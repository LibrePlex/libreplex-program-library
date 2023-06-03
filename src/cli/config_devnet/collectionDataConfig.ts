import { PublicKey } from '@solana/web3.js';
import { CollectionDataInput } from '../../metadata/metadata.client'

export const collectionDataInput: CollectionDataInput = {
    name: "THE GENESIS ZERO",
    symbol: "GEN-ZERO",
    collectionUrl: "https://thegenesis/",
    nftCollectionData: {
        royaltyBps: 500,
        royaltyShares: [
            {
                royaltyAddress: new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh"), // creator1
                royaltyShare: 10000, // in basis points (i.e. 100%)
            }
        ],
        permittedSigners: [
            new PublicKey("nD9NvcnQhxvv5sor6s5KUz2BnczRjygAR3V54w9fuXB"),
            new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh")
        ]
    }
}
