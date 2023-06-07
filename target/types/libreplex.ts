export type Libreplex = {
  "version": "0.1.0",
  "name": "libreplex",
  "instructions": [
    {
      "name": "createGroup",
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
                "account": "MetadataGroup",
                "path": "group"
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
          "name": "group",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "group"
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
            "defined": "GroupInput"
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
                "account": "MetadataGroup",
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
                "value": "group"
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
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "deleteGroup",
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
                "account": "MetadataGroup",
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
      "args": [
        {
          "name": "permissionType",
          "type": {
            "defined": "PermissionType"
          }
        }
      ]
    },
    {
      "name": "editCollectionPermissions",
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
                "account": "MetadataGroup",
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
                "account": "MetadataGroup",
                "path": "collection"
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
          "name": "collection",
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
            "defined": "EditCollectionPermissionsInput"
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
                "account": "Metadata",
                "path": "metadata"
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
            "defined": "CreateMetadataInput"
          }
        }
      ]
    },
    {
      "name": "extendMetadata",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupPermissions",
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
                "account": "MetadataGroup",
                "path": "group"
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
          "name": "metadataPermissions",
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
                "account": "Metadata",
                "path": "metadata"
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "metadataExtended",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata_extended"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "metadataInput",
          "type": {
            "defined": "ExtendMetadataInput"
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
          "isSigner": false
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          },
          "relations": [
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
    }
  ],
  "accounts": [
    {
      "name": "metadataGroup",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
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
      "name": "metadataExtended",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "group",
            "type": "publicKey"
          },
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "attributes",
            "type": "bytes"
          },
          {
            "name": "signers",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
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
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "creator",
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
            "name": "symbol",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "url",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
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
      "name": "EditCollectionPermissionsInput",
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
      "name": "Royalties",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bps",
            "type": "u16"
          },
          {
            "name": "shares",
            "type": {
              "vec": {
                "defined": "RoyaltyShare"
              }
            }
          }
        ]
      }
    },
    {
      "name": "ExtendMetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
              }
            }
          },
          {
            "name": "invokedPermission",
            "type": {
              "defined": "PermissionType"
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
              "vec": {
                "defined": "AttributeValue"
              }
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
      "name": "GroupInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
              }
            }
          },
          {
            "name": "attributeTypes",
            "type": {
              "vec": {
                "defined": "AttributeType"
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
      "name": "AttributesInput",
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
      "name": "CreateMetadataInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateMetadataInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "invokedPermission",
            "type": {
              "defined": "PermissionType"
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
      "name": "AttributeValue",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Word",
            "fields": [
              {
                "name": "value",
                "type": "string"
              }
            ]
          },
          {
            "name": "U8",
            "fields": [
              {
                "name": "value",
                "type": "u8"
              }
            ]
          },
          {
            "name": "U16",
            "fields": [
              {
                "name": "value",
                "type": "u16"
              }
            ]
          },
          {
            "name": "U32",
            "fields": [
              {
                "name": "value",
                "type": "u32"
              }
            ]
          },
          {
            "name": "U64",
            "fields": [
              {
                "name": "value",
                "type": "u64"
              }
            ]
          },
          {
            "name": "I8",
            "fields": [
              {
                "name": "value",
                "type": "i8"
              }
            ]
          },
          {
            "name": "I16",
            "fields": [
              {
                "name": "value",
                "type": "i16"
              }
            ]
          },
          {
            "name": "I32",
            "fields": [
              {
                "name": "value",
                "type": "i32"
              }
            ]
          },
          {
            "name": "I64",
            "fields": [
              {
                "name": "value",
                "type": "i64"
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
          },
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
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "ExtendMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "group",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "PermissionEvent",
      "fields": [
        {
          "name": "group",
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
      "name": "createGroup",
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
                "account": "MetadataGroup",
                "path": "group"
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
          "name": "group",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "group"
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
            "defined": "GroupInput"
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
                "account": "MetadataGroup",
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
                "value": "group"
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
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "deleteGroup",
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
                "account": "MetadataGroup",
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
      "args": [
        {
          "name": "permissionType",
          "type": {
            "defined": "PermissionType"
          }
        }
      ]
    },
    {
      "name": "editCollectionPermissions",
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
                "account": "MetadataGroup",
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
                "account": "MetadataGroup",
                "path": "collection"
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
          "name": "collection",
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
            "defined": "EditCollectionPermissionsInput"
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
                "account": "Metadata",
                "path": "metadata"
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
            "defined": "CreateMetadataInput"
          }
        }
      ]
    },
    {
      "name": "extendMetadata",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupPermissions",
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
                "account": "MetadataGroup",
                "path": "group"
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
          "name": "metadataPermissions",
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
                "account": "Metadata",
                "path": "metadata"
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "metadataExtended",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata_extended"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "metadataInput",
          "type": {
            "defined": "ExtendMetadataInput"
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
          "isSigner": false
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          },
          "relations": [
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
    }
  ],
  "accounts": [
    {
      "name": "metadataGroup",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
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
      "name": "metadataExtended",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "group",
            "type": "publicKey"
          },
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "attributes",
            "type": "bytes"
          },
          {
            "name": "signers",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
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
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "creator",
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
            "name": "symbol",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "url",
            "docs": [
              "from input - variable size"
            ],
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
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
      "name": "EditCollectionPermissionsInput",
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
      "name": "Royalties",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bps",
            "type": "u16"
          },
          {
            "name": "shares",
            "type": {
              "vec": {
                "defined": "RoyaltyShare"
              }
            }
          }
        ]
      }
    },
    {
      "name": "ExtendMetadataInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attributes",
            "type": "bytes"
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
              }
            }
          },
          {
            "name": "invokedPermission",
            "type": {
              "defined": "PermissionType"
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
              "vec": {
                "defined": "AttributeValue"
              }
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
      "name": "GroupInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "metadataRenderMode",
            "type": {
              "defined": "MetadataRenderMode"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "defined": "Royalties"
              }
            }
          },
          {
            "name": "attributeTypes",
            "type": {
              "vec": {
                "defined": "AttributeType"
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
      "name": "AttributesInput",
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
      "name": "CreateMetadataInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateMetadataInput",
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
            "name": "url",
            "type": "string"
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "invokedPermission",
            "type": {
              "defined": "PermissionType"
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
      "name": "AttributeValue",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Word",
            "fields": [
              {
                "name": "value",
                "type": "string"
              }
            ]
          },
          {
            "name": "U8",
            "fields": [
              {
                "name": "value",
                "type": "u8"
              }
            ]
          },
          {
            "name": "U16",
            "fields": [
              {
                "name": "value",
                "type": "u16"
              }
            ]
          },
          {
            "name": "U32",
            "fields": [
              {
                "name": "value",
                "type": "u32"
              }
            ]
          },
          {
            "name": "U64",
            "fields": [
              {
                "name": "value",
                "type": "u64"
              }
            ]
          },
          {
            "name": "I8",
            "fields": [
              {
                "name": "value",
                "type": "i8"
              }
            ]
          },
          {
            "name": "I16",
            "fields": [
              {
                "name": "value",
                "type": "i16"
              }
            ]
          },
          {
            "name": "I32",
            "fields": [
              {
                "name": "value",
                "type": "i32"
              }
            ]
          },
          {
            "name": "I64",
            "fields": [
              {
                "name": "value",
                "type": "i64"
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
          },
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
          "name": "name",
          "type": "string",
          "index": false
        }
      ]
    },
    {
      "name": "ExtendMetadataEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "group",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "PermissionEvent",
      "fields": [
        {
          "name": "group",
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
