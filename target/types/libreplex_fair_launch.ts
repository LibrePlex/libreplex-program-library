export type LibreplexFairLaunch = {
  "version": "0.1.0",
  "name": "libreplex_fair_launch",
  "instructions": [
    {
      "name": "deployLegacy",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
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
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
            "defined": "InitialiseInput"
          }
        }
      ]
    },
    {
      "name": "mintLegacy",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriber",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMasteredition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
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
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintCompressed",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ghostRootSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriber",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountCompressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "noopProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "merkleTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treeAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalTreeDelegate",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "global_tree_delegate"
              }
            ]
          }
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintCompressedInput"
          }
        }
      ]
    },
    {
      "name": "deployMigrated",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
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
    },
    {
      "name": "migrateToHashlist",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "migrationMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "migration_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "migrationCounter",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "migration_counter"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
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
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "fungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "this always exists so we can specify the account type explicitly"
          ]
        },
        {
          "name": "fungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "this always exists (otherwise we couldn't swap), so we can specify the account",
            "type explicitly"
          ]
        },
        {
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
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
    },
    {
      "name": "swapToNonfungible",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
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
      "name": "deployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "limitPerMint",
            "type": "u64"
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
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "useInscriptions",
            "type": "bool"
          },
          {
            "name": "mintedOut",
            "type": "bool"
          },
          {
            "name": "requireCreatorCosign",
            "type": "bool"
          },
          {
            "name": "migratedFromLegacy",
            "type": "bool"
          },
          {
            "name": "escrowNonFungibleCount",
            "type": "u64"
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
            "name": "fungibleMint",
            "type": "publicKey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "hashlist",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "issues",
            "type": {
              "vec": {
                "defined": "MintAndOrder"
              }
            }
          }
        ]
      }
    },
    {
      "name": "migrationMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "hashlistMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "migrationCounter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "migrationCount",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseInput",
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
          }
        ]
      }
    },
    {
      "name": "MintCompressedInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treeDelegateType",
            "type": {
              "defined": "TreeDelegateType"
            }
          }
        ]
      }
    },
    {
      "name": "MintAndOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "order",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TreeDelegateType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Glogbal"
          },
          {
            "name": "Deployment"
          }
        ]
      }
    },
    {
      "name": "DeploymentStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Initialised"
          },
          {
            "name": "Deployed"
          },
          {
            "name": "MintedOut"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "HashlistEvent",
      "fields": [
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "deployment",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "NewDeploymentEvent",
      "fields": [
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "limitPerMint",
          "type": "u64",
          "index": false
        },
        {
          "name": "maxNumberOfTokens",
          "type": "u64",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MintEvent",
      "fields": [
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "tokensMinted",
          "type": "u64",
          "index": false
        },
        {
          "name": "maxNumberOfTokens",
          "type": "u64",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "TickerTooLong",
      "msg": "Ticker too long"
    },
    {
      "code": 6001,
      "name": "MintTemplateTooLong",
      "msg": "Mint template too long"
    },
    {
      "code": 6002,
      "name": "DeploymentTemplateTooLong",
      "msg": "Deployment template too long"
    },
    {
      "code": 6003,
      "name": "RootTypeTooLong",
      "msg": "Root type too long"
    },
    {
      "code": 6004,
      "name": "MintedOut",
      "msg": "Minted out"
    },
    {
      "code": 6005,
      "name": "LegacyMigrationsAreMintedOut",
      "msg": "Legacy migrations are minted out"
    },
    {
      "code": 6006,
      "name": "MissingGlobalTreeDelegate",
      "msg": "Global tree delegate is missing"
    }
  ]
};

export const IDL: LibreplexFairLaunch = {
  "version": "0.1.0",
  "name": "libreplex_fair_launch",
  "instructions": [
    {
      "name": "deployLegacy",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
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
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
            "defined": "InitialiseInput"
          }
        }
      ]
    },
    {
      "name": "mintLegacy",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriber",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMasteredition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
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
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintCompressed",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ghostRootSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriber",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountCompressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "noopProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "merkleTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treeAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalTreeDelegate",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "global_tree_delegate"
              }
            ]
          }
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintCompressedInput"
          }
        }
      ]
    },
    {
      "name": "deployMigrated",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
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
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
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
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
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
    },
    {
      "name": "migrateToHashlist",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "migrationMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "migration_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "migrationCounter",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "migration_counter"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
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
            ]
          }
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountEscrow",
          "isMut": true,
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
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "fungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "this always exists so we can specify the account type explicitly"
          ]
        },
        {
          "name": "fungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "this always exists (otherwise we couldn't swap), so we can specify the account",
            "type explicitly"
          ]
        },
        {
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
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
    },
    {
      "name": "swapToNonfungible",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "deployment"
              },
              {
                "kind": "account",
                "type": "string",
                "account": "Deployment",
                "path": "deployment.ticker"
              }
            ]
          }
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "hashlist_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Deployment",
                "path": "deployment"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "non_fungible_mint"
              }
            ]
          }
        },
        {
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
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
      "name": "deployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "limitPerMint",
            "type": "u64"
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
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "useInscriptions",
            "type": "bool"
          },
          {
            "name": "mintedOut",
            "type": "bool"
          },
          {
            "name": "requireCreatorCosign",
            "type": "bool"
          },
          {
            "name": "migratedFromLegacy",
            "type": "bool"
          },
          {
            "name": "escrowNonFungibleCount",
            "type": "u64"
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
            "name": "fungibleMint",
            "type": "publicKey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "hashlist",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "issues",
            "type": {
              "vec": {
                "defined": "MintAndOrder"
              }
            }
          }
        ]
      }
    },
    {
      "name": "migrationMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "hashlistMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "migrationCounter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "migrationCount",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseInput",
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
          }
        ]
      }
    },
    {
      "name": "MintCompressedInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treeDelegateType",
            "type": {
              "defined": "TreeDelegateType"
            }
          }
        ]
      }
    },
    {
      "name": "MintAndOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "order",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TreeDelegateType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Glogbal"
          },
          {
            "name": "Deployment"
          }
        ]
      }
    },
    {
      "name": "DeploymentStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Initialised"
          },
          {
            "name": "Deployed"
          },
          {
            "name": "MintedOut"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "HashlistEvent",
      "fields": [
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "deployment",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "NewDeploymentEvent",
      "fields": [
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "limitPerMint",
          "type": "u64",
          "index": false
        },
        {
          "name": "maxNumberOfTokens",
          "type": "u64",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MintEvent",
      "fields": [
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "tokensMinted",
          "type": "u64",
          "index": false
        },
        {
          "name": "maxNumberOfTokens",
          "type": "u64",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "TickerTooLong",
      "msg": "Ticker too long"
    },
    {
      "code": 6001,
      "name": "MintTemplateTooLong",
      "msg": "Mint template too long"
    },
    {
      "code": 6002,
      "name": "DeploymentTemplateTooLong",
      "msg": "Deployment template too long"
    },
    {
      "code": 6003,
      "name": "RootTypeTooLong",
      "msg": "Root type too long"
    },
    {
      "code": 6004,
      "name": "MintedOut",
      "msg": "Minted out"
    },
    {
      "code": 6005,
      "name": "LegacyMigrationsAreMintedOut",
      "msg": "Legacy migrations are minted out"
    },
    {
      "code": 6006,
      "name": "MissingGlobalTreeDelegate",
      "msg": "Global tree delegate is missing"
    }
  ]
};
