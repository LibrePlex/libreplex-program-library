export type Creator = {
  "version": "0.1.0",
  "name": "creator",
  "instructions": [
    {
      "name": "createCreator",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "creator"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateCreatorInput"
                },
                "path": "create_creator_input.seed"
              }
            ]
          }
        },
        {
          "name": "permissions",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "permissions"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Creator",
                "path": "creator"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "group",
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
          "name": "creatorInput",
          "type": {
            "defined": "CreateCreatorInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "mintedCount",
            "type": "u32"
          },
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "baseName",
            "type": {
              "defined": "BaseUrl"
            }
          },
          {
            "name": "minted",
            "type": "u64"
          },
          {
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "phases",
            "type": {
              "vec": {
                "defined": "Phase"
              }
            }
          },
          {
            "name": "attributeMappings",
            "type": {
              "option": "publicKey"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateCreatorInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "seed",
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
    },
    {
      "name": "AttributeMapping",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "current",
            "type": "u32"
          },
          {
            "name": "maxOnchainAttributeCount",
            "type": "u32"
          },
          {
            "name": "attributes",
            "type": "bytes"
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
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "priceMint",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "priceQuantity",
            "type": "u64"
          },
          {
            "name": "maxMints",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BaseUrl",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Json",
            "fields": [
              {
                "name": "url",
                "type": "string"
              }
            ]
          },
          {
            "name": "Image",
            "fields": [
              {
                "name": "url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "AccountEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Update"
          },
          {
            "name": "Delete"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "AccountEvent",
      "fields": [
        {
          "name": "reference",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "AccountEventType"
          },
          "index": false
        }
      ]
    }
  ]
};

export const IDL: Creator = {
  "version": "0.1.0",
  "name": "creator",
  "instructions": [
    {
      "name": "createCreator",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "creator"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateCreatorInput"
                },
                "path": "create_creator_input.seed"
              }
            ]
          }
        },
        {
          "name": "permissions",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "permissions"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Creator",
                "path": "creator"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "group",
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
          "name": "creatorInput",
          "type": {
            "defined": "CreateCreatorInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "mintedCount",
            "type": "u32"
          },
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "baseName",
            "type": {
              "defined": "BaseUrl"
            }
          },
          {
            "name": "minted",
            "type": "u64"
          },
          {
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "phases",
            "type": {
              "vec": {
                "defined": "Phase"
              }
            }
          },
          {
            "name": "attributeMappings",
            "type": {
              "option": "publicKey"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateCreatorInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "seed",
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
    },
    {
      "name": "AttributeMapping",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "current",
            "type": "u32"
          },
          {
            "name": "maxOnchainAttributeCount",
            "type": "u32"
          },
          {
            "name": "attributes",
            "type": "bytes"
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
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "priceMint",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "priceQuantity",
            "type": "u64"
          },
          {
            "name": "maxMints",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BaseUrl",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Json",
            "fields": [
              {
                "name": "url",
                "type": "string"
              }
            ]
          },
          {
            "name": "Image",
            "fields": [
              {
                "name": "url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "AccountEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Update"
          },
          {
            "name": "Delete"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "AccountEvent",
      "fields": [
        {
          "name": "reference",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "AccountEventType"
          },
          "index": false
        }
      ]
    }
  ]
};
