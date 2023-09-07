# Quickstart


Install the [SDK](https://www.npmjs.com/package/@libreplex/sdk)


```
import {mintSingle} from "@libreplex/sdk"
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env()

const {method, mint} =  (await mintSingle({
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

await method.rpc()  

```

## Creating a collection/group.



```
import {mintSingle} from "@libreplex/sdk"
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env()

const {method, group} = await setupGroup({
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

await method.rpc()  

```

## Minting to a group.

```
import {mintSingle} from "@libreplex/sdk"
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env()


const group = "....Some Public Key..."

const {method, mint} =  (await mintSingle({
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
        checkValidGroup: false,
    }
}))

await method.rpc()  

```
