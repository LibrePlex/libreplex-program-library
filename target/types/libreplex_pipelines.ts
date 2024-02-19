export type LibreplexPipelines = {
  "version": "0.0.0",
  "name": "libreplex_pipelines",
  "instructions": [
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "pipeline"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialisePipeline"
                },
                "path": "input.ticker"
              }
            ]
          }
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexLiquidityProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialisePipeline"
          }
        }
      ]
    },
    {
      "name": "claimSplAsLiquidityProvider",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrow",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liq_provider_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "recipient"
              }
            ]
          }
        },
        {
          "name": "recipientTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipient",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createSwap",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pipelineSwapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "non_fungible_mint_incoming"
              }
            ]
          }
        },
        {
          "name": "monoswapSwapMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMintIncoming",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerNonfungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "monoswapNonfungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowHolder",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "fungible_mint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMonoswapProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexPipelinesProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "FilterInput"
          }
        }
      ]
    },
    {
      "name": "addLiquidity",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment_config"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ],
            "programId": {
              "kind": "account",
              "type": "publicKey",
              "path": "libreplex_fair_launch_program"
            }
          }
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ],
            "programId": {
              "kind": "account",
              "type": "publicKey",
              "path": "libreplex_fair_launch_program"
            }
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrow",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liq_provider_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "liquidity_provider"
              }
            ]
          }
        },
        {
          "name": "liquidityProvider",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidityNonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexLiquidityProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "pipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "fairLaunchDeployment",
            "type": "publicKey"
          },
          {
            "name": "liquidity",
            "type": "publicKey"
          },
          {
            "name": "auth",
            "type": "publicKey"
          },
          {
            "name": "processedItemCount",
            "type": "u64"
          },
          {
            "name": "creationTime",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "filter",
            "type": {
              "defined": "Filter"
            }
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "fungibleChunkCount",
            "type": "u64"
          },
          {
            "name": "fungibleAmountNet",
            "type": "u64"
          },
          {
            "name": "fungibleAmountTotal",
            "type": "u64"
          },
          {
            "name": "createdSwapCount",
            "type": "u64"
          },
          {
            "name": "authProgramId",
            "type": "publicKey"
          },
          {
            "name": "splSwapAmountPrimary",
            "type": "u64"
          },
          {
            "name": "splSwapAmountSecondary",
            "type": "u64"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "pipelineSwapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pipeline",
            "type": "publicKey"
          },
          {
            "name": "incomingMint",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialisePipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "limitPerMint",
            "type": "u64"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "deploymentTemplate",
            "type": "string"
          },
          {
            "name": "mintTemplate",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "filter",
            "type": {
              "defined": "Filter"
            }
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "liquiditySeed",
            "type": "publicKey"
          },
          {
            "name": "liquidityProviderAmountInLamports",
            "type": "u64"
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "FilterInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Hashlist",
            "fields": [
              {
                "name": "proof",
                "type": {
                  "vec": {
                    "array": [
                      "u8",
                      32
                    ]
                  }
                }
              }
            ]
          },
          {
            "name": "Other"
          }
        ]
      }
    },
    {
      "name": "Filter",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MCC",
            "fields": [
              {
                "name": "collectionId",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "FirstCreatorId",
            "fields": [
              {
                "name": "firstCreatorId",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Hashlist",
            "fields": [
              {
                "name": "root",
                "type": {
                  "array": [
                    "u8",
                    32
                  ]
                }
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "CannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "BadAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "BadAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "MultiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "NotSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "Inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "InscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    },
    {
      "code": 6009,
      "name": "DataHashMismatch",
      "msg": "Metadata data missmatch"
    }
  ]
};

export const IDL: LibreplexPipelines = {
  "version": "0.0.0",
  "name": "libreplex_pipelines",
  "instructions": [
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "pipeline"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialisePipeline"
                },
                "path": "input.ticker"
              }
            ]
          }
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexLiquidityProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialisePipeline"
          }
        }
      ]
    },
    {
      "name": "claimSplAsLiquidityProvider",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrow",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liq_provider_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "recipient"
              }
            ]
          }
        },
        {
          "name": "recipientTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipient",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createSwap",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pipelineSwapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "non_fungible_mint_incoming"
              }
            ]
          }
        },
        {
          "name": "monoswapSwapMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMintIncoming",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerNonfungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "monoswapNonfungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowHolder",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "fungible_mint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMonoswapProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexPipelinesProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "FilterInput"
          }
        }
      ]
    },
    {
      "name": "addLiquidity",
      "accounts": [
        {
          "name": "pipeline",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment_config"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ],
            "programId": {
              "kind": "account",
              "type": "publicKey",
              "path": "libreplex_fair_launch_program"
            }
          }
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ],
            "programId": {
              "kind": "account",
              "type": "publicKey",
              "path": "libreplex_fair_launch_program"
            }
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "auth",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityProviderEscrow",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liq_provider_escrow"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Pipeline",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "liquidity_provider"
              }
            ]
          }
        },
        {
          "name": "liquidityProvider",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidityNonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexLiquidityProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "pipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "fairLaunchDeployment",
            "type": "publicKey"
          },
          {
            "name": "liquidity",
            "type": "publicKey"
          },
          {
            "name": "auth",
            "type": "publicKey"
          },
          {
            "name": "processedItemCount",
            "type": "u64"
          },
          {
            "name": "creationTime",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "filter",
            "type": {
              "defined": "Filter"
            }
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "fungibleChunkCount",
            "type": "u64"
          },
          {
            "name": "fungibleAmountNet",
            "type": "u64"
          },
          {
            "name": "fungibleAmountTotal",
            "type": "u64"
          },
          {
            "name": "createdSwapCount",
            "type": "u64"
          },
          {
            "name": "authProgramId",
            "type": "publicKey"
          },
          {
            "name": "splSwapAmountPrimary",
            "type": "u64"
          },
          {
            "name": "splSwapAmountSecondary",
            "type": "u64"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "pipelineSwapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pipeline",
            "type": "publicKey"
          },
          {
            "name": "incomingMint",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialisePipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "limitPerMint",
            "type": "u64"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "deploymentTemplate",
            "type": "string"
          },
          {
            "name": "mintTemplate",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "filter",
            "type": {
              "defined": "Filter"
            }
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "liquiditySeed",
            "type": "publicKey"
          },
          {
            "name": "liquidityProviderAmountInLamports",
            "type": "u64"
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "FilterInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Hashlist",
            "fields": [
              {
                "name": "proof",
                "type": {
                  "vec": {
                    "array": [
                      "u8",
                      32
                    ]
                  }
                }
              }
            ]
          },
          {
            "name": "Other"
          }
        ]
      }
    },
    {
      "name": "Filter",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MCC",
            "fields": [
              {
                "name": "collectionId",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "FirstCreatorId",
            "fields": [
              {
                "name": "firstCreatorId",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Hashlist",
            "fields": [
              {
                "name": "root",
                "type": {
                  "array": [
                    "u8",
                    32
                  ]
                }
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "CannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "BadAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "BadAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "MultiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "NotSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "Inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "InscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    },
    {
      "code": 6009,
      "name": "DataHashMismatch",
      "msg": "Metadata data missmatch"
    }
  ]
};
