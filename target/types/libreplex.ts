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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
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
      "name": "updateCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userPermissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
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
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "signerCollectionPermissions",
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
                "account": "Collection",
                "path": "collection"
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
          "name": "creator",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collection",
          "isMut": true,
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
      "args": []
    },
    {
      "name": "editPermissions",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authPermissions",
          "isMut": false,
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
                "path": "reference"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "userPermissions",
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
                "path": "reference"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "reference",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK it doesn't matter what type of account we are editing permissions for. Hence unchecked!"
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
            "defined": "EditPermissionsInput"
          }
        }
      ]
    },
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "permissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
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
          "name": "collection",
          "isMut": true,
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
          "name": "permissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "collection",
          "isMut": true,
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
            "collection",
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
      "args": []
    },
    {
      "name": "deletePermissions",
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
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
                "path": "collection"
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
          "name": "collection",
          "isMut": true,
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
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "itemCount",
            "type": "u32"
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
            "name": "collectionRenderMode",
            "type": {
              "defined": "CollectionRenderMode"
            }
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
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
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "name",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          },
          {
            "name": "renderModeData",
            "type": {
              "vec": {
                "defined": "MetadataRenderModeData"
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
    },
    {
      "name": "permissions",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "reference",
            "type": "publicKey"
          },
          {
            "name": "permissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "EditPermissionsInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addPermissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          },
          {
            "name": "removePermissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "AttributeType",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "permittedValues",
            "type": {
              "vec": "string"
            }
          },
          {
            "name": "deleted",
            "type": "bool"
          },
          {
            "name": "continuedAtIndex",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "continuedFromIndex",
            "type": {
              "option": "u32"
            }
          }
        ]
      }
    },
    {
      "name": "BaseUrlConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "prefix",
            "type": "string"
          },
          {
            "name": "suffix",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "NftCollectionData",
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
          },
          {
            "name": "attributeTypes",
            "type": {
              "vec": {
                "defined": "AttributeType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "RoyaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "recipient",
            "type": "publicKey"
          },
          {
            "name": "share",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "CollectionInput",
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
            "name": "collectionRenderMode",
            "type": {
              "defined": "CollectionRenderMode"
            }
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
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
      "name": "NftMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
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
      "name": "NftMetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "MetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "renderModeData",
            "type": {
              "defined": "MetadataRenderModeData"
            }
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadataInput"
              }
            }
          }
        ]
      }
    },
    {
      "name": "CollectionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Edit"
          },
          {
            "name": "Delete"
          }
        ]
      }
    },
    {
      "name": "CollectionRenderMode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
            "fields": [
              {
                "name": "collection_url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataRenderMode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
            "fields": [
              {
                "name": "base_url_configuration",
                "type": {
                  "option": {
                    "defined": "BaseUrlConfiguration"
                  }
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataRenderModeData",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
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
      "name": "PermissionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Update"
          },
          {
            "name": "Delete"
          }
        ]
      }
    },
    {
      "name": "PermissionType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Admin"
          }
        ]
      }
    },
    {
      "name": "PermissionCounts",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Admin",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Create",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Edit",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Delete",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "CollectionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "CollectionEventType"
          },
          "index": false
        }
      ]
    },
    {
      "name": "CreateMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collection",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "EditCollectionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "EditMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collection",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "PermissionEvent",
      "fields": [
        {
          "name": "reference",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "PermissionEventType"
          },
          "index": false
        }
      ]
    },
    {
      "name": "MetadataPermissionEvent",
      "fields": [
        {
          "name": "metadata",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        }
      ]
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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
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
      "name": "updateCollection",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userPermissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
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
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "signerCollectionPermissions",
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
                "account": "Collection",
                "path": "collection"
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
          "name": "creator",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collection",
          "isMut": true,
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
      "args": []
    },
    {
      "name": "editPermissions",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authPermissions",
          "isMut": false,
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
                "path": "reference"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "userPermissions",
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
                "path": "reference"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "reference",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK it doesn't matter what type of account we are editing permissions for. Hence unchecked!"
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
            "defined": "EditPermissionsInput"
          }
        }
      ]
    },
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "permissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
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
          "name": "collection",
          "isMut": true,
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
          "name": "permissions",
          "isMut": false,
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
                "account": "Collection",
                "path": "collection"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "collection",
          "isMut": true,
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
            "collection",
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
      "args": []
    },
    {
      "name": "deletePermissions",
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
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
                "path": "collection"
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
          "name": "collection",
          "isMut": true,
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
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "itemCount",
            "type": "u32"
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
            "name": "collectionRenderMode",
            "type": {
              "defined": "CollectionRenderMode"
            }
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
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
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "name",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadata"
              }
            }
          },
          {
            "name": "renderModeData",
            "type": {
              "vec": {
                "defined": "MetadataRenderModeData"
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
    },
    {
      "name": "permissions",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "reference",
            "type": "publicKey"
          },
          {
            "name": "permissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "EditPermissionsInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addPermissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          },
          {
            "name": "removePermissions",
            "type": {
              "vec": {
                "defined": "PermissionType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "AttributeType",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "permittedValues",
            "type": {
              "vec": "string"
            }
          },
          {
            "name": "deleted",
            "type": "bool"
          },
          {
            "name": "continuedAtIndex",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "continuedFromIndex",
            "type": {
              "option": "u32"
            }
          }
        ]
      }
    },
    {
      "name": "BaseUrlConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "prefix",
            "type": "string"
          },
          {
            "name": "suffix",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "NftCollectionData",
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
          },
          {
            "name": "attributeTypes",
            "type": {
              "vec": {
                "defined": "AttributeType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "RoyaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "recipient",
            "type": "publicKey"
          },
          {
            "name": "share",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "CollectionInput",
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
            "name": "collectionRenderMode",
            "type": {
              "defined": "CollectionRenderMode"
            }
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
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
      "name": "NftMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
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
      "name": "NftMetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "MetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "renderModeData",
            "type": {
              "defined": "MetadataRenderModeData"
            }
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": {
                "defined": "NftMetadataInput"
              }
            }
          }
        ]
      }
    },
    {
      "name": "CollectionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Edit"
          },
          {
            "name": "Delete"
          }
        ]
      }
    },
    {
      "name": "CollectionRenderMode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
            "fields": [
              {
                "name": "collection_url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataRenderMode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
            "fields": [
              {
                "name": "base_url_configuration",
                "type": {
                  "option": {
                    "defined": "BaseUrlConfiguration"
                  }
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataRenderModeData",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Program",
            "fields": [
              {
                "name": "program_id",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Url",
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
      "name": "PermissionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Update"
          },
          {
            "name": "Delete"
          }
        ]
      }
    },
    {
      "name": "PermissionType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Admin"
          }
        ]
      }
    },
    {
      "name": "PermissionCounts",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Admin",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Create",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Edit",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          },
          {
            "name": "Delete",
            "fields": [
              {
                "name": "count",
                "type": "u32"
              }
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "CollectionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "CollectionEventType"
          },
          "index": false
        }
      ]
    },
    {
      "name": "CreateMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collection",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "EditCollectionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "EditMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collection",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "PermissionEvent",
      "fields": [
        {
          "name": "reference",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "PermissionEventType"
          },
          "index": false
        }
      ]
    },
    {
      "name": "MetadataPermissionEvent",
      "fields": [
        {
          "name": "metadata",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        }
      ]
    }
  ]
};
