export type LibreplexFairLaunch = {
  "version": "0.3.1",
  "name": "libreplex_fair_launch",
  "instructions": [
    {
      "name": "initialiseV3",
      "accounts": [
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
          "name": "creator",
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
            "defined": "InitialiseInputV3"
          }
        }
      ]
    },
    {
      "name": "initialiseRaw",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
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
            "defined": "InitialiseRawInput"
          }
        }
      ]
    },
    {
      "name": "deployRaw",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
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
      "args": []
    },
    {
      "name": "deployToken22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
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
          "name": "tokenProgram2022",
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
          "name": "rent",
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
      "name": "toggleFreeze",
      "accounts": [
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "delegate",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deployhybrid",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
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
          "name": "rent",
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
      "name": "deployHybridUnchecked",
      "accounts": [
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
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
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
        }
      ],
      "args": []
    },
    {
      "name": "relinquishCosigner",
      "accounts": [
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
          "name": "cosigner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateSymbol22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
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
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
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
          "name": "fungibleMint",
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
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nonFungibleTokenAccountOwner",
          "isMut": false,
          "isSigner": true
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
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "joinraw",
      "accounts": [
        {
          "name": "deployment",
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "nonFungibleMint",
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
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "updateSplMetadata",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
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
          "name": "newUri",
          "type": "string"
        }
      ]
    },
    {
      "name": "switchDeploymentType",
      "accounts": [
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "deploymentType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "reduceMintCount",
      "accounts": [
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
          "name": "creator",
          "isMut": false,
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
            "defined": "ReduceMintCountInput"
          }
        }
      ]
    },
    {
      "name": "burnExcessSpl",
      "accounts": [
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
          "name": "signer",
          "isMut": false,
          "isSigner": true
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
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintToken22",
      "accounts": [
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
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
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
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "swapToFungible22",
      "accounts": [
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
          "name": "signer",
          "isMut": false,
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
          "isSigner": false
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
          "name": "fungibleTargetTokenAccountOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceAccountOwner",
          "isMut": false,
          "isSigner": true
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
        }
      ],
      "args": []
    },
    {
      "name": "swapToNonfungible22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
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
        }
      ],
      "args": []
    },
    {
      "name": "mintLegacy",
      "accounts": [
        {
          "name": "deployment",
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
      "name": "claimTransferFeeAuthAsCreator",
      "accounts": [
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
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
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
          "name": "sources",
          "type": {
            "vec": "publicKey"
          }
        }
      ]
    },
    {
      "name": "migrateToHashlist",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "migrationMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "migrationCounter",
          "isMut": true,
          "isSigner": false
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
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": false,
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
          "isSigner": false
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
          "isSigner": false
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
          "isSigner": false
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
          "isSigner": false
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
      "name": "deploymentConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintLamports",
            "type": "u64"
          },
          {
            "name": "transferFeeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "option": {
                "defined": "MultiplierLimits"
              }
            }
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "transferFeeTargetWallet",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "totalSplEquivalentMinted",
            "type": "u64"
          },
          {
            "name": "splExcessInEscrow",
            "type": "u64"
          },
          {
            "name": "allowBurn",
            "docs": [
              "used for variable-rate swaps"
            ],
            "type": "bool"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          },
          {
            "name": "uncheckedFungible",
            "type": "bool"
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
    },
    {
      "name": "migrationMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "deploymentV2",
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
            "name": "fungibleDecimals",
            "type": "u8"
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
            "name": "fungibleMint",
            "type": "publicKey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "proxyProgramId",
            "type": "publicKey"
          },
          {
            "name": "cosignerMint",
            "type": "publicKey"
          },
          {
            "name": "cosignerSwapToNft",
            "type": "publicKey"
          },
          {
            "name": "cosignerSwapToSpl",
            "type": "publicKey"
          },
          {
            "name": "fungibleType",
            "type": {
              "defined": "FungibleType"
            }
          },
          {
            "name": "nonFungibleType",
            "type": {
              "defined": "NonFungibleType"
            }
          },
          {
            "name": "deployed",
            "type": "bool"
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
    },
    {
      "name": "inscriptionSummary",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "inscriptionCountTotal",
            "type": "u64"
          },
          {
            "name": "inscriptionCountImmutables",
            "type": "u64"
          },
          {
            "name": "lastInscription",
            "type": "publicKey"
          },
          {
            "name": "lastInscriber",
            "type": "publicKey"
          },
          {
            "name": "lastInscriptionCreateTime",
            "type": "i64"
          },
          {
            "name": "extension",
            "type": {
              "defined": "SummaryExtension"
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionV3",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "inscriptionData",
            "type": "publicKey"
          },
          {
            "name": "order",
            "type": "u64"
          },
          {
            "name": "size",
            "type": "u32"
          },
          {
            "name": "contentType",
            "type": "string"
          },
          {
            "name": "encoding",
            "type": "string"
          },
          {
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseRawInput",
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
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "proxyProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerMint",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerSwapToSpl",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerSwapToNft",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "fungibleType",
            "type": {
              "defined": "FungibleType"
            }
          },
          {
            "name": "nonFungibleType",
            "type": {
              "defined": "NonFungibleType"
            }
          }
        ]
      }
    },
    {
      "name": "InitialiseInputV3",
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
            "name": "creatorCosignProgramId",
            "type": {
              "option": "publicKey"
            }
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
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintInLamports",
            "type": "u64"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "defined": "MultiplierLimits"
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "TransferFeeInputConfig"
              }
            }
          }
        ]
      }
    },
    {
      "name": "TransferFeeInputConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "withdrawAuthority",
            "type": "publicKey"
          },
          {
            "name": "targetWallet",
            "type": "publicKey"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "ReduceMintCountInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "MintInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "multiplierNumerator",
            "type": "u16"
          },
          {
            "name": "multiplierDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "DeploymentConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintLamports",
            "type": "u64"
          },
          {
            "name": "transferFeeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "option": {
                "defined": "MultiplierLimits"
              }
            }
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "transferFeeTargetWallet",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "totalSplEquivalentMinted",
            "type": "u64"
          },
          {
            "name": "splExcessInEscrow",
            "type": "u64"
          },
          {
            "name": "allowBurn",
            "docs": [
              "used for variable-rate swaps"
            ],
            "type": "bool"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          },
          {
            "name": "uncheckedFungible",
            "type": "bool"
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
      "name": "MultiplierLimits",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumerator",
            "type": "u16"
          },
          {
            "name": "minDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "FungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "TokenKeg"
          },
          {
            "name": "Token2022"
          }
        ]
      }
    },
    {
      "name": "NonFungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "TokenKeg"
          },
          {
            "name": "Token2022"
          },
          {
            "name": "Nifty"
          }
        ]
      }
    },
    {
      "name": "SummaryExtension",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeploymentActive",
      "fields": [
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "fungibleMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
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
      "name": "NewDeploymentV2",
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
        },
        {
          "name": "offChainUrl",
          "type": "string",
          "index": false
        },
        {
          "name": "requireCoSign",
          "type": "bool",
          "index": false
        },
        {
          "name": "deploymentTemplate",
          "type": "string",
          "index": false
        },
        {
          "name": "mintTemplate",
          "type": "string",
          "index": false
        },
        {
          "name": "usesInscriptions",
          "type": "bool",
          "index": false
        },
        {
          "name": "decimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "deploymentType",
          "type": "u8",
          "index": false
        },
        {
          "name": "config",
          "type": {
            "option": {
              "defined": "DeploymentConfig"
            }
          },
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
      "name": "OffchainUrlTooLong",
      "msg": "Offchain URL too long"
    },
    {
      "code": 6003,
      "name": "DeploymentTemplateTooLong",
      "msg": "Deployment template too long"
    },
    {
      "code": 6004,
      "name": "RootTypeTooLong",
      "msg": "Root type too long"
    },
    {
      "code": 6005,
      "name": "MintedOut",
      "msg": "Minted out"
    },
    {
      "code": 6006,
      "name": "LegacyMigrationsAreMintedOut",
      "msg": "Legacy migrations are minted out"
    },
    {
      "code": 6007,
      "name": "MissingGlobalTreeDelegate",
      "msg": "Global tree delegate is missing"
    },
    {
      "code": 6008,
      "name": "IncorrectMintType",
      "msg": "Incorrect mint type"
    },
    {
      "code": 6009,
      "name": "InvalidMetadata",
      "msg": "Invalid Metadata"
    },
    {
      "code": 6010,
      "name": "CreatorFeeTooHigh",
      "msg": "Creator fee too high"
    },
    {
      "code": 6011,
      "name": "MultiplierMissMatch",
      "msg": "Custom multiplier mints require co signer"
    },
    {
      "code": 6012,
      "name": "IncorrectMintCosigner",
      "msg": "Incorrect cosigner for mint"
    },
    {
      "code": 6013,
      "name": "IncorrectSwapToSplCosigner",
      "msg": "Incorrect cosigner for swap to spl"
    },
    {
      "code": 6014,
      "name": "IncorrectSwapToNftCosigner",
      "msg": "Incorrect cosigner for swap to NFT"
    }
  ]
};

export const IDL: LibreplexFairLaunch = {
  "version": "0.3.1",
  "name": "libreplex_fair_launch",
  "instructions": [
    {
      "name": "initialiseV3",
      "accounts": [
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
          "name": "creator",
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
            "defined": "InitialiseInputV3"
          }
        }
      ]
    },
    {
      "name": "initialiseRaw",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
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
            "defined": "InitialiseRawInput"
          }
        }
      ]
    },
    {
      "name": "deployRaw",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
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
      "args": []
    },
    {
      "name": "deployToken22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
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
          "name": "tokenProgram2022",
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
          "name": "rent",
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
      "name": "toggleFreeze",
      "accounts": [
        {
          "name": "deployment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "delegate",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deployhybrid",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
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
          "name": "rent",
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
      "name": "deployHybridUnchecked",
      "accounts": [
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
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fungibleEscrowTokenAccount",
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
        }
      ],
      "args": []
    },
    {
      "name": "relinquishCosigner",
      "accounts": [
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
          "name": "cosigner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateSymbol22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
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
          "name": "deployment",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "fungible_mint"
          ]
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
          "name": "fungibleMint",
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
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nonFungibleTokenAccountOwner",
          "isMut": false,
          "isSigner": true
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
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "joinraw",
      "accounts": [
        {
          "name": "deployment",
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "nonFungibleMint",
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
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "updateSplMetadata",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
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
          "name": "newUri",
          "type": "string"
        }
      ]
    },
    {
      "name": "switchDeploymentType",
      "accounts": [
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "deploymentType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "reduceMintCount",
      "accounts": [
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
          "name": "creator",
          "isMut": false,
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
            "defined": "ReduceMintCountInput"
          }
        }
      ]
    },
    {
      "name": "burnExcessSpl",
      "accounts": [
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
          "name": "signer",
          "isMut": false,
          "isSigner": true
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
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintToken22",
      "accounts": [
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
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
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
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "swapToFungible22",
      "accounts": [
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
          "name": "signer",
          "isMut": false,
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
          "isSigner": false
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
          "name": "fungibleTargetTokenAccountOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleSourceAccountOwner",
          "isMut": false,
          "isSigner": true
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
        }
      ],
      "args": []
    },
    {
      "name": "swapToNonfungible22",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": false,
          "isSigner": false
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
        }
      ],
      "args": []
    },
    {
      "name": "mintLegacy",
      "accounts": [
        {
          "name": "deployment",
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
      "name": "claimTransferFeeAuthAsCreator",
      "accounts": [
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
          "name": "fungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram22",
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
          "name": "sources",
          "type": {
            "vec": "publicKey"
          }
        }
      ]
    },
    {
      "name": "migrateToHashlist",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "migrationMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "migrationCounter",
          "isMut": true,
          "isSigner": false
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
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": false,
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
          "isSigner": false
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
          "isSigner": false
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
          "isSigner": false
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
          "isSigner": false
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
      "name": "deploymentConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintLamports",
            "type": "u64"
          },
          {
            "name": "transferFeeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "option": {
                "defined": "MultiplierLimits"
              }
            }
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "transferFeeTargetWallet",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "totalSplEquivalentMinted",
            "type": "u64"
          },
          {
            "name": "splExcessInEscrow",
            "type": "u64"
          },
          {
            "name": "allowBurn",
            "docs": [
              "used for variable-rate swaps"
            ],
            "type": "bool"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          },
          {
            "name": "uncheckedFungible",
            "type": "bool"
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
    },
    {
      "name": "migrationMarker",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "deploymentV2",
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
            "name": "fungibleDecimals",
            "type": "u8"
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
            "name": "fungibleMint",
            "type": "publicKey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "proxyProgramId",
            "type": "publicKey"
          },
          {
            "name": "cosignerMint",
            "type": "publicKey"
          },
          {
            "name": "cosignerSwapToNft",
            "type": "publicKey"
          },
          {
            "name": "cosignerSwapToSpl",
            "type": "publicKey"
          },
          {
            "name": "fungibleType",
            "type": {
              "defined": "FungibleType"
            }
          },
          {
            "name": "nonFungibleType",
            "type": {
              "defined": "NonFungibleType"
            }
          },
          {
            "name": "deployed",
            "type": "bool"
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
    },
    {
      "name": "inscriptionSummary",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "inscriptionCountTotal",
            "type": "u64"
          },
          {
            "name": "inscriptionCountImmutables",
            "type": "u64"
          },
          {
            "name": "lastInscription",
            "type": "publicKey"
          },
          {
            "name": "lastInscriber",
            "type": "publicKey"
          },
          {
            "name": "lastInscriptionCreateTime",
            "type": "i64"
          },
          {
            "name": "extension",
            "type": {
              "defined": "SummaryExtension"
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionV3",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "inscriptionData",
            "type": "publicKey"
          },
          {
            "name": "order",
            "type": "u64"
          },
          {
            "name": "size",
            "type": "u32"
          },
          {
            "name": "contentType",
            "type": "string"
          },
          {
            "name": "encoding",
            "type": "string"
          },
          {
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseRawInput",
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
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "proxyProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerMint",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerSwapToSpl",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerSwapToNft",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "fungibleType",
            "type": {
              "defined": "FungibleType"
            }
          },
          {
            "name": "nonFungibleType",
            "type": {
              "defined": "NonFungibleType"
            }
          }
        ]
      }
    },
    {
      "name": "InitialiseInputV3",
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
            "name": "creatorCosignProgramId",
            "type": {
              "option": "publicKey"
            }
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
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintInLamports",
            "type": "u64"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "defined": "MultiplierLimits"
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "TransferFeeInputConfig"
              }
            }
          }
        ]
      }
    },
    {
      "name": "TransferFeeInputConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "withdrawAuthority",
            "type": "publicKey"
          },
          {
            "name": "targetWallet",
            "type": "publicKey"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "ReduceMintCountInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "MintInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "multiplierNumerator",
            "type": "u16"
          },
          {
            "name": "multiplierDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "DeploymentConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "publicKey"
          },
          {
            "name": "creatorFeePerMintLamports",
            "type": "u64"
          },
          {
            "name": "transferFeeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "option": {
                "defined": "MultiplierLimits"
              }
            }
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "transferFeeTargetWallet",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "totalSplEquivalentMinted",
            "type": "u64"
          },
          {
            "name": "splExcessInEscrow",
            "type": "u64"
          },
          {
            "name": "allowBurn",
            "docs": [
              "used for variable-rate swaps"
            ],
            "type": "bool"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          },
          {
            "name": "uncheckedFungible",
            "type": "bool"
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
      "name": "MultiplierLimits",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumerator",
            "type": "u16"
          },
          {
            "name": "minDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "FungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "TokenKeg"
          },
          {
            "name": "Token2022"
          }
        ]
      }
    },
    {
      "name": "NonFungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "TokenKeg"
          },
          {
            "name": "Token2022"
          },
          {
            "name": "Nifty"
          }
        ]
      }
    },
    {
      "name": "SummaryExtension",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeploymentActive",
      "fields": [
        {
          "name": "ticker",
          "type": "string",
          "index": false
        },
        {
          "name": "fungibleMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
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
      "name": "NewDeploymentV2",
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
        },
        {
          "name": "offChainUrl",
          "type": "string",
          "index": false
        },
        {
          "name": "requireCoSign",
          "type": "bool",
          "index": false
        },
        {
          "name": "deploymentTemplate",
          "type": "string",
          "index": false
        },
        {
          "name": "mintTemplate",
          "type": "string",
          "index": false
        },
        {
          "name": "usesInscriptions",
          "type": "bool",
          "index": false
        },
        {
          "name": "decimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "deploymentType",
          "type": "u8",
          "index": false
        },
        {
          "name": "config",
          "type": {
            "option": {
              "defined": "DeploymentConfig"
            }
          },
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
      "name": "OffchainUrlTooLong",
      "msg": "Offchain URL too long"
    },
    {
      "code": 6003,
      "name": "DeploymentTemplateTooLong",
      "msg": "Deployment template too long"
    },
    {
      "code": 6004,
      "name": "RootTypeTooLong",
      "msg": "Root type too long"
    },
    {
      "code": 6005,
      "name": "MintedOut",
      "msg": "Minted out"
    },
    {
      "code": 6006,
      "name": "LegacyMigrationsAreMintedOut",
      "msg": "Legacy migrations are minted out"
    },
    {
      "code": 6007,
      "name": "MissingGlobalTreeDelegate",
      "msg": "Global tree delegate is missing"
    },
    {
      "code": 6008,
      "name": "IncorrectMintType",
      "msg": "Incorrect mint type"
    },
    {
      "code": 6009,
      "name": "InvalidMetadata",
      "msg": "Invalid Metadata"
    },
    {
      "code": 6010,
      "name": "CreatorFeeTooHigh",
      "msg": "Creator fee too high"
    },
    {
      "code": 6011,
      "name": "MultiplierMissMatch",
      "msg": "Custom multiplier mints require co signer"
    },
    {
      "code": 6012,
      "name": "IncorrectMintCosigner",
      "msg": "Incorrect cosigner for mint"
    },
    {
      "code": 6013,
      "name": "IncorrectSwapToSplCosigner",
      "msg": "Incorrect cosigner for swap to spl"
    },
    {
      "code": 6014,
      "name": "IncorrectSwapToNftCosigner",
      "msg": "Incorrect cosigner for swap to NFT"
    }
  ]
};
