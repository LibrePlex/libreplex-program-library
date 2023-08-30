import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { AnchorProvider, getProvider } from "@coral-xyz/anchor";
import {mintSingle, setupGroup, setUserPermissionsForGroup, UserPermission, updateGroupAuthority} from "@libreplex/sdk"
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

    it ("has minted to a group", async () => {
        const me = provider.publicKey

        const grpCtx = await setupGroup({
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

        const group = grpCtx.group

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
            mintToGroup: {
                group,
                checkValidGroup: true,
            }
        }))

        await mintCtx.method.rpc()     
    })

    it ("has minted to a group where I am not the authority", async () => {
        const me = provider.publicKey

        const grpCtx = await setupGroup({
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

        const group = grpCtx.group;

        await (await setUserPermissionsForGroup({
            connector: {
                type: "provider",
                provider,
            },
            group,
            user: me,
            groupUpdateAuthority: me,
            permissions: [UserPermission.AddToGroup]
        })).rpc()

        await (await updateGroupAuthority({
            group,
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
            mintToGroup: {
                group,
                checkValidGroup: true,
            }
        }))

        await mintCtx.method.rpc()     
    })

  
})