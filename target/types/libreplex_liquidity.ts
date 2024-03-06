export type LibreplexLiquidity = {
  "version": "0.1.0",
  "name": "libreplex_liquidity",
  "instructions": [
    {
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
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
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
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
          "name": "fairLaunchProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mint",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "treasury"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledHashlistMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
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
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledNonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "isMut": true,
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
          "name": "fairLaunch",
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
      "name": "join",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "treasury"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledHashlistMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
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
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledNonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "isMut": true,
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
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "JoinInput"
          }
        }
      ]
    },
    {
      "name": "createLookupTable",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lookupTable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "recentSlot",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
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
            "defined": "InitialiseInput"
          }
        }
      ]
    },
    {
      "name": "initialiseV2",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
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
            "defined": "InitialiseInputV2"
          }
        }
      ]
    },
    {
      "name": "bootstrapPool",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerWrappedSolAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgramLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ]
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
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMintMetadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "relinquishCosigner",
      "accounts": [
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fixDeploymentType",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerWrappedSolAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgramLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ]
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
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMintMetadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "prepareNativeEscrow",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowWrappedSolAccount",
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
      "name": "mintSpl",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
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
          "name": "liquidityFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountReceiver",
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
          "name": "fairLaunch",
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
            "name": "deploymentType",
            "type": "u8"
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
      "name": "liquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
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
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
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
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "InitialiseInputV2",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "requiredDoubleMints",
            "type": "u32"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "JoinInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pooledMultiplierNumerator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "pooledMultiplierDenominator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "userMultiplierNumerator",
            "type": "u16"
          },
          {
            "name": "userMultiplierDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "Liquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "Bootstrap",
      "fields": [
        {
          "name": "liquidity",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "LiquidityCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "liquidity",
          "type": {
            "defined": "Liquidity"
          },
          "index": false
        }
      ]
    },
    {
      "name": "Mint",
      "fields": [
        {
          "name": "liquidity",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "totalMints",
          "type": "u64",
          "index": false
        }
      ]
    }
  ]
};

export const IDL: LibreplexLiquidity = {
  "version": "0.1.0",
  "name": "libreplex_liquidity",
  "instructions": [
    {
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
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
          "name": "nonFungibleTargetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
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
          "name": "fairLaunchProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mint",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "treasury"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledHashlistMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
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
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledNonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "isMut": true,
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
          "name": "fairLaunch",
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
      "name": "join",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "treasury"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledHashlistMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
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
          "name": "nonFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pooledNonFungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "isMut": true,
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
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "JoinInput"
          }
        }
      ]
    },
    {
      "name": "createLookupTable",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lookupTable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "recentSlot",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
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
            "defined": "InitialiseInput"
          }
        }
      ]
    },
    {
      "name": "initialiseV2",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
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
            "defined": "InitialiseInputV2"
          }
        }
      ]
    },
    {
      "name": "bootstrapPool",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerWrappedSolAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgramLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ]
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
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMintMetadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "relinquishCosigner",
      "accounts": [
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fixDeploymentType",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payerWrappedSolAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payerLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgramLpTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ]
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
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedSolTokenFee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMintMetadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "prepareNativeEscrow",
      "accounts": [
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "wrappedSolMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowWrappedSolAccount",
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
      "name": "mintSpl",
      "accounts": [
        {
          "name": "receiver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
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
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
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
          "name": "liquidityFungibleTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleTokenAccountReceiver",
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
          "name": "fairLaunch",
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
            "name": "deploymentType",
            "type": "u8"
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
      "name": "liquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
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
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
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
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "InitialiseInputV2",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "requiredDoubleMints",
            "type": "u32"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "JoinInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pooledMultiplierNumerator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "pooledMultiplierDenominator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "userMultiplierNumerator",
            "type": "u16"
          },
          {
            "name": "userMultiplierDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "Liquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "Bootstrap",
      "fields": [
        {
          "name": "liquidity",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "LiquidityCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "liquidity",
          "type": {
            "defined": "Liquidity"
          },
          "index": false
        }
      ]
    },
    {
      "name": "Mint",
      "fields": [
        {
          "name": "liquidity",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "totalMints",
          "type": "u64",
          "index": false
        }
      ]
    }
  ]
};
