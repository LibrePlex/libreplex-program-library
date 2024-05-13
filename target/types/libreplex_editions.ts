export type LibreplexEditions = {
  "version": "0.2.1",
  "name": "libreplex_editions",
  "instructions": [
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "editionsDeployment",
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
          "isSigner": false
        },
        {
          "name": "groupMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": true
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
          "name": "groupExtensionProgram",
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
      "name": "mint",
      "accounts": [
        {
          "name": "editionsDeployment",
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
          "name": "minter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "member",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
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
          "name": "groupExtensionProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "editionsDeployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "groupMint",
            "type": "publicKey"
          },
          {
            "name": "group",
            "type": "publicKey"
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
      "name": "hashlistMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "editionsDeployment",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
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
            "name": "creatorCosignProgramId",
            "type": {
              "option": "publicKey"
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
    },
    {
      "code": 6007,
      "name": "IncorrectMintType",
      "msg": "Incorrect mint type"
    },
    {
      "code": 6008,
      "name": "InvalidMetadata",
      "msg": "Invalid Metadata"
    },
    {
      "code": 6009,
      "name": "CreatorFeeTooHigh",
      "msg": "Creator fee too high"
    }
  ]
};

export const IDL: LibreplexEditions = {
  "version": "0.2.1",
  "name": "libreplex_editions",
  "instructions": [
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "editionsDeployment",
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
          "isSigner": false
        },
        {
          "name": "groupMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": true
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
          "name": "groupExtensionProgram",
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
      "name": "mint",
      "accounts": [
        {
          "name": "editionsDeployment",
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
          "name": "minter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "member",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
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
          "name": "groupExtensionProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "editionsDeployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "groupMint",
            "type": "publicKey"
          },
          {
            "name": "group",
            "type": "publicKey"
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
      "name": "hashlistMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "editionsDeployment",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
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
            "name": "creatorCosignProgramId",
            "type": {
              "option": "publicKey"
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
    },
    {
      "code": 6007,
      "name": "IncorrectMintType",
      "msg": "Incorrect mint type"
    },
    {
      "code": 6008,
      "name": "InvalidMetadata",
      "msg": "Invalid Metadata"
    },
    {
      "code": 6009,
      "name": "CreatorFeeTooHigh",
      "msg": "Creator fee too high"
    }
  ]
};
