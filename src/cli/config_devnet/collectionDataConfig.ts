import { PublicKey } from '@solana/web3.js';
import { CollectionDataInput } from '../../metadata/metadata.client';

type CollectionDataConfig = {
    collectionDataInput: CollectionDataInput,
}

// export const collectionDataConfig: CollectionDataConfig =
//     {
//         collectionDataInput : {
//             name: "THE GENESIS ZERO",
//             symbol: "GEN-ZERO",
//             collectionUrl: "https://thegenesis/",
//             nftCollectionData: {
//                 royaltyBps: 500,
//                 royaltyShares: [
//                     {
//                         royaltyAddress: new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh"), // creator1
//                         royaltyShare: 10000, // in basis points (i.e. 100%)
//                     }
//                 ],
//                 permittedSigners: [
//                     new PublicKey("nD9NvcnQhxvv5sor6s5KUz2BnczRjygAR3V54w9fuXB"),
//                     new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh")
//                 ]
//             }
//         }
//     }

export const collectionDataConfig: CollectionDataConfig =
    {
        collectionDataInput : {
            name: "THE GENESIS ZERO",
            symbol: "GEN-ZERO",
            collectionUrl: "https://thegenesis/",
            nftCollectionData: {
                royaltyBps: 420,
                royaltyShares: [
                    {
                        royaltyAddress: new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh"), // creator1
                        royaltyShare: 6969, // in basis points (i.e. 100%)
                    },
                    {
                        royaltyAddress: new PublicKey("FsouSwjRg68Z5jP8K9XmezeV1t5SNdxjQZBpXvTYxkMF"), // creator2
                        royaltyShare: 1642, // in basis points (i.e. 100%)
                    },
                    {
                        royaltyAddress: new PublicKey("26BHW3eXotV4AgMfMiq8zYGe7WmPus1uLfdktuPgPrVK"), // creator3
                        royaltyShare: 969, // in basis points (i.e. 100%)
                    },
                    {
                        royaltyAddress: new PublicKey("C2ZkiWsG6A3QgqSUBaZFZy3VJVT1coj1JgTuPUgQPdnm"), // creator4
                        royaltyShare: 420, // in basis points (i.e. 100%)
                    }
                ],
                permittedSigners: [
                    new PublicKey("nD9NvcnQhxvv5sor6s5KUz2BnczRjygAR3V54w9fuXB"),
                    new PublicKey("B5dB1agyQvzVmWugmcYj24RUibEhRB45YtPzsUc3bSCh"),
                    new PublicKey("FsouSwjRg68Z5jP8K9XmezeV1t5SNdxjQZBpXvTYxkMF"),
                    new PublicKey("26BHW3eXotV4AgMfMiq8zYGe7WmPus1uLfdktuPgPrVK"),
                    new PublicKey("C2ZkiWsG6A3QgqSUBaZFZy3VJVT1coj1JgTuPUgQPdnm"),
                ]
            }
        }
    }
