/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_editions_controls.json`.
 */
export type LibreplexEditionsControls = {
  "address": "EdCo6pePXJX3PuEPRLSE59gKXp4KDwWjATEXRpztvu9X",
  "metadata": {
    "name": "libreplexEditionsControls",
    "version": "0.2.1",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/Libreplex/libreplex-program-library"
  },
  "instructions": [
    {
      "name": "addPhase",
      "discriminator": [
        245,
        220,
        147,
        40,
        30,
        207,
        36,
        127
      ],
      "accounts": [
        {
          "name": "editionsControls",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "libreplexEditionsProgram",
          "address": "Eddy2qpzTgTfUvNJ271Wgfw3RF2LsbmRJdNTSnZajmU3"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialisePhaseInput"
            }
          }
        }
      ]
    },
    {
      "name": "initialiseEditionsControls",
      "discriminator": [
        69,
        176,
        133,
        29,
        20,
        49,
        120,
        202
      ],
      "accounts": [
        {
          "name": "editionsControls",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110,
                  115,
                  95,
                  99,
                  111,
                  110,
                  116,
                  114,
                  111,
                  108,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "editionsDeployment"
              }
            ]
          }
        },
        {
          "name": "editionsDeployment",
          "writable": true
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator"
        },
        {
          "name": "groupMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "group",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "groupExtensionProgram",
          "address": "TGRPp2mDGxSyH3We9hH8pwcmhajtszPAvWjVdVgsPa5"
        },
        {
          "name": "libreplexEditionsProgram",
          "address": "Eddy2qpzTgTfUvNJ271Wgfw3RF2LsbmRJdNTSnZajmU3"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialiseControlInput"
            }
          }
        }
      ]
    },
    {
      "name": "mintWithControls",
      "discriminator": [
        167,
        57,
        252,
        220,
        69,
        92,
        231,
        61
      ],
      "accounts": [
        {
          "name": "editionsDeployment",
          "writable": true
        },
        {
          "name": "editionsControls",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110,
                  115,
                  95,
                  99,
                  111,
                  110,
                  116,
                  114,
                  111,
                  108,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "editionsDeployment"
              }
            ]
          }
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "minter",
          "writable": true
        },
        {
          "name": "minterStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  116,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "editionsDeployment"
              },
              {
                "kind": "account",
                "path": "minter"
              }
            ]
          }
        },
        {
          "name": "minterStatsPhase",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  116,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115,
                  95,
                  112,
                  104,
                  97,
                  115,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "editionsDeployment"
              },
              {
                "kind": "account",
                "path": "minter"
              },
              {
                "kind": "arg",
                "path": "mint_input.phase_index"
              }
            ]
          }
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "member",
          "writable": true,
          "signer": true
        },
        {
          "name": "group",
          "writable": true
        },
        {
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "groupExtensionProgram",
          "address": "TGRPp2mDGxSyH3We9hH8pwcmhajtszPAvWjVdVgsPa5"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "libreplexEditionsProgram",
          "address": "Eddy2qpzTgTfUvNJ271Wgfw3RF2LsbmRJdNTSnZajmU3"
        }
      ],
      "args": [
        {
          "name": "mintInput",
          "type": {
            "defined": {
              "name": "mintInput"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "editionsControls",
      "discriminator": [
        124,
        32,
        239,
        85,
        118,
        231,
        152,
        156
      ]
    },
    {
      "name": "editionsDeployment",
      "discriminator": [
        101,
        54,
        68,
        216,
        168,
        131,
        242,
        157
      ]
    },
    {
      "name": "minterStats",
      "discriminator": [
        138,
        239,
        240,
        226,
        199,
        53,
        170,
        179
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "tickerTooLong",
      "msg": "Ticker too long"
    },
    {
      "code": 6001,
      "name": "mintTemplateTooLong",
      "msg": "Mint template too long"
    },
    {
      "code": 6002,
      "name": "deploymentTemplateTooLong",
      "msg": "Deployment template too long"
    },
    {
      "code": 6003,
      "name": "rootTypeTooLong",
      "msg": "Root type too long"
    },
    {
      "code": 6004,
      "name": "mintedOut",
      "msg": "Minted out"
    },
    {
      "code": 6005,
      "name": "legacyMigrationsAreMintedOut",
      "msg": "Legacy migrations are minted out"
    },
    {
      "code": 6006,
      "name": "missingGlobalTreeDelegate",
      "msg": "Global tree delegate is missing"
    },
    {
      "code": 6007,
      "name": "incorrectMintType",
      "msg": "Incorrect mint type"
    },
    {
      "code": 6008,
      "name": "invalidMetadata",
      "msg": "Invalid Metadata"
    },
    {
      "code": 6009,
      "name": "creatorFeeTooHigh",
      "msg": "Creator fee too high"
    }
  ],
  "types": [
    {
      "name": "editionsControls",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "editionsDeployment",
            "type": "pubkey"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                200
              ]
            }
          },
          {
            "name": "phases",
            "type": {
              "vec": {
                "defined": {
                  "name": "phase"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "editionsDeployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "numberOfTokensIssued",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "groupMint",
            "type": "pubkey"
          },
          {
            "name": "group",
            "type": "pubkey"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "nameIsTemplate",
            "type": "bool"
          },
          {
            "name": "urlIsTemplate",
            "type": "bool"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                98
              ]
            }
          }
        ]
      }
    },
    {
      "name": "initialiseControlInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "initialisePhaseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "pubkey"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "maxMintsTotal",
            "type": "u64"
          },
          {
            "name": "endTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "mintInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "phaseIndex",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "minterStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "mintCount",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                50
              ]
            }
          }
        ]
      }
    },
    {
      "name": "phase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "pubkey"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "maxMintsTotal",
            "type": "u64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "currentMints",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                200
              ]
            }
          }
        ]
      }
    }
  ]
};
