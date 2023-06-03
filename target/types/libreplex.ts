export type Libreplex = {
  "version": "0.1.0",
  "name": "libreplex",
  "instructions": [
    {
      "name": "createCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "collection",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "seed"
              }
            ]
          }
        },
        {
          "name": "seed",
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
          "name": "collectionInput",
          "type": {
            "defined": "CollectionInput"
          }
        }
      ]
    },
    {
      "name": "deleteCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "receiver",
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
          "name": "bumpCollectionData",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "metadataInput",
          "type": {
            "defined": "MetadataInput"
          }
        },
        {
          "name": "bumpCollectionData",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          },
          "relations": [
            "collection_data",
            "mint"
          ]
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "receiver",
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
          "name": "bumpCollectionData",
          "type": "u8"
        },
        {
          "name": "bumpMetadata",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collectionSeed",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "collectionUrl",
            "type": "string"
          },
          {
            "name": "collectionCount",
            "type": "u64"
          },
          {
            "name": "nftCollectionData",
            "type": {
              "option": {
                "defined": "NftCollectionData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftCollectionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "royaltyBps",
            "type": "u16"
          },
          {
            "name": "royaltyShares",
            "type": {
              "vec": {
                "defined": "RoyaltyShare"
              }
            }
          },
          {
            "name": "permittedSigners",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "royaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "royaltyAddress",
            "type": "publicKey"
          },
          {
            "name": "royaltyShare",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "collectionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "collectionUrl",
            "type": "string"
          },
          {
            "name": "nftCollectionData",
            "type": {
              "option": {
                "defined": "NftCollectionData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "metadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collectionData",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": {
              "vec": {
                "defined": "Attribute"
              }
            }
          },
          {
            "name": "signers",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "attribute",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "traitType",
            "type": "string"
          },
          {
            "name": "attribute",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "metadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "metadataUrl",
            "type": "string"
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftMetadataUnique",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "royaltyBpsOverride",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "royaltiesShareOverride",
            "type": {
              "option": {
                "vec": {
                  "defined": "RoyaltyShare"
                }
              }
            }
          },
          {
            "name": "permittedSignersOverride",
            "type": {
              "option": {
                "vec": "publicKey"
              }
            }
          }
        ]
      }
    }
  ]
};

export const IDL: Libreplex = {
  "version": "0.1.0",
  "name": "libreplex",
  "instructions": [
    {
      "name": "createCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "collection",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "seed"
              }
            ]
          }
        },
        {
          "name": "seed",
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
          "name": "collectionInput",
          "type": {
            "defined": "CollectionInput"
          }
        }
      ]
    },
    {
      "name": "deleteCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "receiver",
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
          "name": "bumpCollectionData",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "metadataInput",
          "type": {
            "defined": "MetadataInput"
          }
        },
        {
          "name": "bumpCollectionData",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collectionData",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "collection_seed"
              }
            ]
          },
          "relations": [
            "authority",
            "collection_seed"
          ]
        },
        {
          "name": "collectionSeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          },
          "relations": [
            "collection_data",
            "mint"
          ]
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "receiver",
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
          "name": "bumpCollectionData",
          "type": "u8"
        },
        {
          "name": "bumpMetadata",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collectionSeed",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "collectionUrl",
            "type": "string"
          },
          {
            "name": "collectionCount",
            "type": "u64"
          },
          {
            "name": "nftCollectionData",
            "type": {
              "option": {
                "defined": "NftCollectionData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftCollectionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "royaltyBps",
            "type": "u16"
          },
          {
            "name": "royaltyShares",
            "type": {
              "vec": {
                "defined": "RoyaltyShare"
              }
            }
          },
          {
            "name": "permittedSigners",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "royaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "royaltyAddress",
            "type": "publicKey"
          },
          {
            "name": "royaltyShare",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "collectionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "collectionUrl",
            "type": "string"
          },
          {
            "name": "nftCollectionData",
            "type": {
              "option": {
                "defined": "NftCollectionData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "metadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collectionData",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": {
              "vec": {
                "defined": "Attribute"
              }
            }
          },
          {
            "name": "signers",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "attribute",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "traitType",
            "type": "string"
          },
          {
            "name": "attribute",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "metadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "metadataUrl",
            "type": "string"
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nftMetadataUnique",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "royaltyBpsOverride",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "royaltiesShareOverride",
            "type": {
              "option": {
                "vec": {
                  "defined": "RoyaltyShare"
                }
              }
            }
          },
          {
            "name": "permittedSignersOverride",
            "type": {
              "option": {
                "vec": "publicKey"
              }
            }
          }
        ]
      }
    }
  ]
};
