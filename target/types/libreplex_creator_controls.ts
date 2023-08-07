export type LibreplexCreatorControls = {
  "version": "0.1.0",
  "name": "libreplex_creator_controls",
  "instructions": [
    {
      "name": "mint",
      "accounts": [
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "groupPermissions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterNumbers",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "attributeConfig",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "libreplexCreatorProgram",
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
      "name": "initialize",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "type": {
                  "defined": "InitializeInput"
                },
                "path": "input.seed"
              }
            ]
          }
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
        },
        {
          "name": "libreplexCreatorProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitializeInput"
          }
        }
      ]
    },
    {
      "name": "update",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
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
            "defined": "UpdateInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "creatorController",
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
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "AllowList",
      "type": {
        "kind": "struct",
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
    },
    {
      "name": "Payment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "recepient",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "MintLimit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "limit",
            "type": "u32"
          },
          {
            "name": "accountKey",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "scopedToBuyer",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "SplPayment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "recepient",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CustomProgram",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "instructionData",
            "type": "bytes"
          },
          {
            "name": "remainingAccountsToUse",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "InitializeInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "phases",
            "type": {
              "vec": {
                "defined": "Phase"
              }
            }
          },
          {
            "name": "seed",
            "type": "publicKey"
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
            "name": "args",
            "type": {
              "vec": "bytes"
            }
          },
          {
            "name": "chosenPhase",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateInput",
      "type": {
        "kind": "struct",
        "fields": [
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
      "name": "Phase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "start",
            "type": "i64"
          },
          {
            "name": "end",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "label",
            "type": "string"
          },
          {
            "name": "controls",
            "type": {
              "vec": {
                "defined": "ControlType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "ControlType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "AllowList",
            "fields": [
              {
                "defined": "AllowList"
              }
            ]
          },
          {
            "name": "Payment",
            "fields": [
              {
                "defined": "Payment"
              }
            ]
          },
          {
            "name": "SplPayment",
            "fields": [
              {
                "defined": "SplPayment"
              }
            ]
          },
          {
            "name": "MintLimit",
            "fields": [
              {
                "defined": "MintLimit"
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
      "name": "MissingArgument",
      "msg": "Missing argument"
    },
    {
      "code": 6001,
      "name": "InvalidProof",
      "msg": "Invalid proof"
    },
    {
      "code": 6002,
      "name": "MissingAccount",
      "msg": "Missing Account"
    },
    {
      "code": 6003,
      "name": "InvalidMintFundsRecepient",
      "msg": "Invalid Mint funds recepient"
    },
    {
      "code": 6004,
      "name": "InvalidTokenRecepient",
      "msg": "Invalid token recepient"
    },
    {
      "code": 6005,
      "name": "InvalidTotalMintsAccount",
      "msg": "Invalid total mints account"
    },
    {
      "code": 6006,
      "name": "MintLimitExceeded",
      "msg": "Mint Limit Exceeded"
    },
    {
      "code": 6007,
      "name": "NoActivePhases",
      "msg": "No Active Phases"
    },
    {
      "code": 6008,
      "name": "PhaseNotSpecified",
      "msg": "Phase not specified"
    }
  ]
};

export const IDL: LibreplexCreatorControls = {
  "version": "0.1.0",
  "name": "libreplex_creator_controls",
  "instructions": [
    {
      "name": "mint",
      "accounts": [
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "groupPermissions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minterNumbers",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "attributeConfig",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "libreplexCreatorProgram",
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
      "name": "initialize",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "type": {
                  "defined": "InitializeInput"
                },
                "path": "input.seed"
              }
            ]
          }
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
        },
        {
          "name": "libreplexCreatorProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitializeInput"
          }
        }
      ]
    },
    {
      "name": "update",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creatorController",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
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
            "defined": "UpdateInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "creatorController",
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
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "AllowList",
      "type": {
        "kind": "struct",
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
    },
    {
      "name": "Payment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "recepient",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "MintLimit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "limit",
            "type": "u32"
          },
          {
            "name": "accountKey",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "scopedToBuyer",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "SplPayment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "recepient",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CustomProgram",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "instructionData",
            "type": "bytes"
          },
          {
            "name": "remainingAccountsToUse",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "InitializeInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "phases",
            "type": {
              "vec": {
                "defined": "Phase"
              }
            }
          },
          {
            "name": "seed",
            "type": "publicKey"
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
            "name": "args",
            "type": {
              "vec": "bytes"
            }
          },
          {
            "name": "chosenPhase",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateInput",
      "type": {
        "kind": "struct",
        "fields": [
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
      "name": "Phase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "start",
            "type": "i64"
          },
          {
            "name": "end",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "label",
            "type": "string"
          },
          {
            "name": "controls",
            "type": {
              "vec": {
                "defined": "ControlType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "ControlType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "AllowList",
            "fields": [
              {
                "defined": "AllowList"
              }
            ]
          },
          {
            "name": "Payment",
            "fields": [
              {
                "defined": "Payment"
              }
            ]
          },
          {
            "name": "SplPayment",
            "fields": [
              {
                "defined": "SplPayment"
              }
            ]
          },
          {
            "name": "MintLimit",
            "fields": [
              {
                "defined": "MintLimit"
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
      "name": "MissingArgument",
      "msg": "Missing argument"
    },
    {
      "code": 6001,
      "name": "InvalidProof",
      "msg": "Invalid proof"
    },
    {
      "code": 6002,
      "name": "MissingAccount",
      "msg": "Missing Account"
    },
    {
      "code": 6003,
      "name": "InvalidMintFundsRecepient",
      "msg": "Invalid Mint funds recepient"
    },
    {
      "code": 6004,
      "name": "InvalidTokenRecepient",
      "msg": "Invalid token recepient"
    },
    {
      "code": 6005,
      "name": "InvalidTotalMintsAccount",
      "msg": "Invalid total mints account"
    },
    {
      "code": 6006,
      "name": "MintLimitExceeded",
      "msg": "Mint Limit Exceeded"
    },
    {
      "code": 6007,
      "name": "NoActivePhases",
      "msg": "No Active Phases"
    },
    {
      "code": 6008,
      "name": "PhaseNotSpecified",
      "msg": "Phase not specified"
    }
  ]
};
