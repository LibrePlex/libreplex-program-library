export type LibreplexLiquidity = {
  "version": "0.0.1",
  "name": "libreplex_liquidity",
  "instructions": [
    {
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "liqudity",
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
                "path": "non_fungible_mint"
              }
            ]
          }
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
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "authority"
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
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
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
          "name": "inscriptionsProgram",
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
      "name": "initialise",
      "accounts": [
        {
          "name": "authority",
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
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liquidity"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.seed"
              }
            ]
          }
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
    }
  ],
  "accounts": [
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
            "name": "padding",
            "type": {
              "array": [
                "u8",
                100
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
          }
        ]
      }
    }
  ]
};

export const IDL: LibreplexLiquidity = {
  "version": "0.0.1",
  "name": "libreplex_liquidity",
  "instructions": [
    {
      "name": "swapToFungible",
      "accounts": [
        {
          "name": "liqudity",
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
                "path": "non_fungible_mint"
              }
            ]
          }
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
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidity",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment",
            "authority"
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
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
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
          "name": "inscriptionsProgram",
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
      "name": "initialise",
      "accounts": [
        {
          "name": "authority",
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
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "liquidity"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.seed"
              }
            ]
          }
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
    }
  ],
  "accounts": [
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
            "name": "padding",
            "type": {
              "array": [
                "u8",
                100
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
          }
        ]
      }
    }
  ]
};
