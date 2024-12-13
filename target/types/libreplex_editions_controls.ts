export type LibreplexEditionsControls = {
  "version": "0.2.2",
  "name": "libreplex_editions_controls",
  "instructions": [
    {
      "name": "initialiseEditionsControls",
      "accounts": [
        {
          "name": "editionsControls",
          "isMut": true,
          "isSigner": false
        },
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
          "isMut": false,
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
        },
        {
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialiseControlInput"
          }
        }
      ]
    },
    {
      "name": "addPhase",
      "accounts": [
        {
          "name": "editionsControls",
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
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialisePhaseInput"
          }
        }
      ]
    },
    {
      "name": "mintWithControls",
      "accounts": [
        {
          "name": "editionsDeployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "editionsControls",
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
          "name": "minter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterStats",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterStatsPhase",
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
          "name": "treasury",
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
        },
        {
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mintInput",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "claimUpdateAuthority",
      "accounts": [
        {
          "name": "editionsControls",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "editionsDeployment",
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
          "name": "mint",
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
        },
        {
          "name": "libreplexEditionsProgram",
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
      "name": "editionsControls",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "editionsDeployment",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
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
                "defined": "Phase"
              }
            }
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
            "type": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "InitialisePhaseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "publicKey"
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
      "name": "InitialiseControlInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
              "option": "publicKey"
            }
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
            "name": "phaseIndex",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "Phase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "publicKey"
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
    },
    {
      "code": 6010,
      "name": "InvalidCreator",
      "msg": "Invalid creator"
    },
    {
      "code": 6011,
      "name": "InvalidSigner",
      "msg": "Invalid signer"
    }
  ]
};

export const IDL: LibreplexEditionsControls = {
  "version": "0.2.2",
  "name": "libreplex_editions_controls",
  "instructions": [
    {
      "name": "initialiseEditionsControls",
      "accounts": [
        {
          "name": "editionsControls",
          "isMut": true,
          "isSigner": false
        },
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
          "isMut": false,
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
        },
        {
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialiseControlInput"
          }
        }
      ]
    },
    {
      "name": "addPhase",
      "accounts": [
        {
          "name": "editionsControls",
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
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialisePhaseInput"
          }
        }
      ]
    },
    {
      "name": "mintWithControls",
      "accounts": [
        {
          "name": "editionsDeployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "editionsControls",
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
          "name": "minter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterStats",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterStatsPhase",
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
          "name": "treasury",
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
        },
        {
          "name": "libreplexEditionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mintInput",
          "type": {
            "defined": "MintInput"
          }
        }
      ]
    },
    {
      "name": "claimUpdateAuthority",
      "accounts": [
        {
          "name": "editionsControls",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "editionsDeployment",
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
          "name": "mint",
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
        },
        {
          "name": "libreplexEditionsProgram",
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
      "name": "editionsControls",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "editionsDeployment",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
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
                "defined": "Phase"
              }
            }
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
            "type": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "InitialisePhaseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "publicKey"
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
      "name": "InitialiseControlInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMintsPerWallet",
            "type": "u64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
              "option": "publicKey"
            }
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
            "name": "phaseIndex",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "Phase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "priceAmount",
            "type": "u64"
          },
          {
            "name": "priceToken",
            "type": "publicKey"
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
    },
    {
      "code": 6010,
      "name": "InvalidCreator",
      "msg": "Invalid creator"
    },
    {
      "code": 6011,
      "name": "InvalidSigner",
      "msg": "Invalid signer"
    }
  ]
};
