/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_nft.json`.
 */
export type LibreplexNft = {
  "address": "9SXDHUdtfvBGT3H2uPCNEkxmWREoqdeS1qdBudLDD6KX",
  "metadata": {
    "name": "libreplexNft",
    "version": "0.10.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/LibrePlex/metadata"
  },
  "instructions": [
    {
      "name": "toggleFreeze",
      "discriminator": [
        136,
        3,
        54,
        90,
        252,
        134,
        158,
        172
      ],
      "accounts": [
        {
          "name": "delegate",
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "wrappedMint",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "token_account.mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "toggleFreezeInput"
            }
          }
        }
      ]
    },
    {
      "name": "wrap",
      "discriminator": [
        178,
        40,
        10,
        189,
        228,
        129,
        186,
        140
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "wrappedMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "wrappedMint",
      "discriminator": [
        154,
        132,
        105,
        241,
        240,
        244,
        36,
        193
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidMintAuthority",
      "msg": "invalidMintAuthority"
    },
    {
      "code": 6001,
      "name": "mintCannotRepresentNft",
      "msg": "mintCannotRepresentNft"
    },
    {
      "code": 6002,
      "name": "invalidMint",
      "msg": "invalidMint"
    },
    {
      "code": 6003,
      "name": "invalidTokenAccount",
      "msg": "invalidTokenAccount"
    }
  ],
  "types": [
    {
      "name": "toggleFreezeInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "freeze"
          },
          {
            "name": "unfreeze"
          }
        ]
      }
    },
    {
      "name": "wrappedMint",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ]
};
