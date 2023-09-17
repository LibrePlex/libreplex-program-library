import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { AnchorProvider, getProvider } from "@coral-xyz/anchor";
import {mintSingle, setupCollection, setUserPermissionsForGroup, UserPermission, updateCollectionAuthority} from "@libreplex/sdk"
import { Keypair } from "@solana/web3.js";


describe("mint", () => {
    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    it("has minted", async () => {
        const mintCtx =  (await mintSingle({
            provider,
            mintData: {
                assetUrl: {
                    type: "jsonUrl",
                    value: "COOL.com"
                },
                name: "COOL",
                symbol: "COOL",
            }
        }))

        await mintCtx.method.rpc()        
    })

    it ("has minted to a collection", async () => {
        const me = provider.publicKey

        const grpCtx = await setupCollection({
            connector: {
                type: "provider",
                provider,
            },
            groupAuthority: me,
            input: {
                description: "A very cool group",
                name: "COOLIO",
                symbol: "GRP",
                url: "COOL.com",
                royalties: {
                    bps: 0,
                    shares: [{
                        recipient: me,
                        share: 100,
                    }],
                },
                permittedSigners: [],
                onChainAttributes: [],
            },
        })
        
        await grpCtx.method.rpc()

        const collection = grpCtx.collection

        const mintCtx =  (await mintSingle({
            provider,
            mintData: {
                assetUrl: {
                    type: "jsonUrl",
                    value: "COOL.com"
                },
                name: "COOL",
                symbol: "COOL",
            },
            mintToCollection: {
                collection,
                checkValidGroup: true,
            }
        }))

        await mintCtx.method.rpc()     
    })

    it ("has minted to a collection where I am not the authority", async () => {
        const me = provider.publicKey

        const grpCtx = await setupCollection({
            connector: {
                type: "provider",
                provider,
            },
            groupAuthority: me,
            input: {
                description: "A very cool group",
                name: "COOLIO",
                symbol: "GRP",
                url: "COOL.com",
                royalties: {
                    bps: 0,
                    shares: [],
                },
                permittedSigners: [],
                onChainAttributes: [],
            },
        })

        await grpCtx.method.rpc();

        const collection = grpCtx.collection;

        await (await setUserPermissionsForGroup({
            connector: {
                type: "provider",
                provider,
            },
            collection,
            user: me,
            groupUpdateAuthority: me,
            permissions: [UserPermission.AddToGroup]
        })).rpc()

        await (await updateCollectionAuthority({
            collection,
            new_authority: Keypair.generate().publicKey,
            connector: {
                type: "provider",
                provider,
            },
        })).rpc()

        const mintCtx =  (await mintSingle({
            provider,
            mintData: {
                assetUrl: {
                    type: "jsonUrl",
                    value: "COOL.com"
                },
                name: "COOL",
                symbol: "COOL",
            },
            mintToCollection: {
                collection,
                checkValidGroup: true,
            }
        }))

        await mintCtx.method.rpc()     
    })

  
})