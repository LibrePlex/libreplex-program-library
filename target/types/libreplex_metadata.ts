export type LibreplexMetadata = {
  "version": "0.10.0",
  "name": "libreplex_metadata",
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
          "name": "groupInput",
          "type": {
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "updateGroup",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "groupInput",
          "type": {
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "updateMetadata",
      "accounts": [
        {
          "name": "editor",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
            "defined": "UpdateMetadataInput"
          }
        }
      ]
    },
    {
      "name": "groupAdd",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "group_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "groupRemove",
      "accounts": [
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "group_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updatePermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userPermissions",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
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
      "name": "delegateGroupPermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": true,
          "isSigner": true
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
                "path": "delegated_user"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
        },
        {
          "name": "delegatedUser",
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
          "name": "editPermissionsInput",
          "type": {
            "defined": "EditPermissionsInput"
          }
        }
      ]
    },
    {
      "name": "delegateMetadataPermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": true,
          "isSigner": true
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
                "path": "delegated_user"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "metadata",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
        },
        {
          "name": "delegatedUser",
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
          "name": "editPermissionsInput",
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
          "name": "payer",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "invokedMigratorProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createInscriptionMetadata",
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
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "metadataInput",
          "type": {
            "defined": "CreateMetadataInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "updateInscriptionDatatype",
      "accounts": [
        {
          "name": "editor",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "inscriptionInput",
          "type": {
            "defined": "UpdateInscriptionDataTypeInput"
          }
        }
      ]
    },
    {
      "name": "deleteMetadataInscription",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionsProgram",
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
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "permissions",
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
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "group",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
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
            "name": "description",
            "type": "string"
          },
          {
            "name": "templateConfiguration",
            "type": {
              "defined": "TemplateConfiguration"
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
      "name": "metadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
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
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "group",
            "type": {
              "option": "publicKey"
            }
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
            "name": "asset",
            "type": {
              "defined": "Asset"
            }
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
            }
          }
        ]
      }
    },
    {
      "name": "delegatePermissions",
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
          },
          {
            "name": "updateAuthority",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateMetadataInscriptionInput",
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
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
            }
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "dataType",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "UpdateInscriptionDataTypeInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dataType",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "EditPermissionsInput",
      "type": {
        "kind": "struct",
        "fields": [
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
    },
    {
      "name": "Hoo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "t",
            "type": "string"
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
            "name": "description",
            "type": "string"
          },
          {
            "name": "templateConfiguration",
            "type": {
              "defined": "TemplateConfiguration"
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
      "name": "MetadataExtensionInput",
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
            "name": "asset",
            "type": {
              "defined": "Asset"
            }
          },
          {
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
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
            "name": "asset",
            "type": {
              "defined": "Asset"
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
      "name": "TemplateConfiguration",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Template",
            "fields": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "image_url",
                "type": "string"
              },
              {
                "name": "description",
                "type": "string"
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
      "name": "GroupEventType",
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
    },
    {
      "name": "License",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NoLicense"
          },
          {
            "name": "Custom",
            "fields": [
              {
                "name": "license_url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataExtension",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Nft",
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
                "name": "license",
                "type": {
                  "option": {
                    "defined": "License"
                  }
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "Asset",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
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
            "name": "JsonTemplate",
            "fields": [
              {
                "name": "url_parameter",
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
              },
              {
                "name": "description",
                "type": {
                  "option": "string"
                }
              }
            ]
          },
          {
            "name": "ChainRenderer",
            "fields": [
              {
                "name": "render_output_address",
                "type": "publicKey"
              },
              {
                "name": "program_id",
                "type": "publicKey"
              },
              {
                "name": "description",
                "type": {
                  "option": "string"
                }
              }
            ]
          },
          {
            "name": "Inscription",
            "fields": [
              {
                "name": "account_id",
                "type": "publicKey"
              },
              {
                "name": "data_type",
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
        ]
      }
    },
    {
      "name": "MetadataEventType",
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
            "name": "Update"
          },
          {
            "name": "Delete"
          },
          {
            "name": "AddToGroup"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeleteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
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
      "name": "GroupEvent",
      "fields": [
        {
          "name": "eventType",
          "type": {
            "defined": "GroupEventType"
          },
          "index": false
        },
        {
          "name": "authority",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        },
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MetadataEvent",
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
          "name": "eventType",
          "type": {
            "defined": "MetadataEventType"
          },
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
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ArithmeticError",
      "msg": "failed to perform some math operation safely"
    },
    {
      "code": 6001,
      "name": "UnknownInstruction",
      "msg": "unknown instruction called"
    },
    {
      "code": 6002,
      "name": "InvalidParameter",
      "msg": "invalid parameter passed in"
    },
    {
      "code": 6003,
      "name": "AnchorSerializationIssue",
      "msg": "anchor serialization issue"
    },
    {
      "code": 6004,
      "name": "AmountMismatch",
      "msg": "two amounts that are supposed to be equal are not"
    },
    {
      "code": 6005,
      "name": "AccountDiscriminatorMismatch",
      "msg": "account discriminator doesn't match"
    },
    {
      "code": 6006,
      "name": "Reserved6"
    },
    {
      "code": 6007,
      "name": "Reserved7"
    },
    {
      "code": 6008,
      "name": "Reserved8"
    },
    {
      "code": 6009,
      "name": "Reserved9"
    },
    {
      "code": 6010,
      "name": "Reserved10"
    },
    {
      "code": 6011,
      "name": "InvalidStringInput",
      "msg": "A constraint on max string length was violated"
    },
    {
      "code": 6012,
      "name": "InvalidBpsInput",
      "msg": "The value of the basis points input must not exceed 10,000"
    },
    {
      "code": 6013,
      "name": "InvalidPermissions",
      "msg": "Invalid Permissions"
    },
    {
      "code": 6014,
      "name": "MissingPermissionAdmin",
      "msg": "Missing admin permission"
    },
    {
      "code": 6015,
      "name": "MissingPermissionEditCollection",
      "msg": "Missing edit collection permission"
    },
    {
      "code": 6016,
      "name": "MissingPermissionDeleteCollection",
      "msg": "Missing delete collection permission"
    },
    {
      "code": 6017,
      "name": "MissingPermissionCreateMetadata",
      "msg": "Missing create metadata permission"
    },
    {
      "code": 6018,
      "name": "MissingPermissionEditMetadata",
      "msg": "Missing edit metadata permission"
    },
    {
      "code": 6019,
      "name": "MissingPermissionDeleteMetadata",
      "msg": "Missing delete metadata permission"
    },
    {
      "code": 6020,
      "name": "CollectionExists",
      "msg": "Collection exists"
    },
    {
      "code": 6021,
      "name": "IncompatibleMetadataType",
      "msg": "Incompatible metadata type"
    },
    {
      "code": 6022,
      "name": "CollectionHasItems",
      "msg": "Collection has items"
    },
    {
      "code": 6023,
      "name": "PermissionAccountEmpty",
      "msg": "Permission account is empty"
    },
    {
      "code": 6024,
      "name": "InvalidBump",
      "msg": "Invalid bump"
    },
    {
      "code": 6025,
      "name": "RoyaltiesBadSum",
      "msg": "Royalties must add up to 10000"
    },
    {
      "code": 6026,
      "name": "UnexpectedPermissionsKey",
      "msg": "Unexpected permission is empty"
    },
    {
      "code": 6027,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    },
    {
      "code": 6028,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6029,
      "name": "MetadataBelongsToGroup",
      "msg": "Metadata belongs to a group"
    },
    {
      "code": 6030,
      "name": "DerivedKeyInvalid",
      "msg": "Derived key invalid"
    },
    {
      "code": 6031,
      "name": "InvalidSignedProgram",
      "msg": "Invalid signer program"
    },
    {
      "code": 6032,
      "name": "MetadataDoesNotHaveAGroup",
      "msg": "Metadata does not have a group"
    },
    {
      "code": 6033,
      "name": "MetadataIsNotMutable",
      "msg": "Metadata is not mutable"
    },
    {
      "code": 6034,
      "name": "InvokeDeleteInscriptionMetadata",
      "msg": "Inscription metadata is deleted via a separate method"
    },
    {
      "code": 6035,
      "name": "OnlyUsedForInscriptionMetadata",
      "msg": "Only used for inscription metadata"
    },
    {
      "code": 6036,
      "name": "WrongAssetType",
      "msg": "Wrong asset type"
    },
    {
      "code": 6037,
      "name": "InvalidMetadataPointer",
      "msg": "Invalid metadata pointer"
    },
    {
      "code": 6038,
      "name": "Reserved37"
    },
    {
      "code": 6039,
      "name": "Reserved38"
    },
    {
      "code": 6040,
      "name": "Reserved39"
    },
    {
      "code": 6041,
      "name": "Reserved40"
    },
    {
      "code": 6042,
      "name": "Reserved41"
    },
    {
      "code": 6043,
      "name": "Reserved42"
    },
    {
      "code": 6044,
      "name": "Reserved43"
    },
    {
      "code": 6045,
      "name": "Reserved44"
    },
    {
      "code": 6046,
      "name": "Reserved45"
    },
    {
      "code": 6047,
      "name": "Reserved46"
    },
    {
      "code": 6048,
      "name": "Reserved47"
    },
    {
      "code": 6049,
      "name": "Reserved48"
    },
    {
      "code": 6050,
      "name": "Reserved49"
    },
    {
      "code": 6051,
      "name": "Reserved50"
    },
    {
      "code": 6052,
      "name": "Reserved51"
    },
    {
      "code": 6053,
      "name": "Reserved52"
    },
    {
      "code": 6054,
      "name": "Reserved53"
    },
    {
      "code": 6055,
      "name": "Reserved54"
    },
    {
      "code": 6056,
      "name": "Reserved55"
    },
    {
      "code": 6057,
      "name": "Reserved56"
    },
    {
      "code": 6058,
      "name": "Reserved57"
    },
    {
      "code": 6059,
      "name": "Reserved58"
    },
    {
      "code": 6060,
      "name": "Reserved59"
    },
    {
      "code": 6061,
      "name": "Reserved60"
    },
    {
      "code": 6062,
      "name": "Reserved61"
    },
    {
      "code": 6063,
      "name": "Reserved62"
    },
    {
      "code": 6064,
      "name": "Reserved63"
    },
    {
      "code": 6065,
      "name": "Reserved64"
    },
    {
      "code": 6066,
      "name": "Reserved65"
    },
    {
      "code": 6067,
      "name": "Reserved66"
    },
    {
      "code": 6068,
      "name": "Reserved67"
    },
    {
      "code": 6069,
      "name": "Reserved68"
    },
    {
      "code": 6070,
      "name": "Reserved69"
    },
    {
      "code": 6071,
      "name": "Reserved70"
    },
    {
      "code": 6072,
      "name": "Reserved71"
    },
    {
      "code": 6073,
      "name": "Reserved72"
    },
    {
      "code": 6074,
      "name": "Reserved73"
    },
    {
      "code": 6075,
      "name": "Reserved74"
    },
    {
      "code": 6076,
      "name": "Reserved75"
    },
    {
      "code": 6077,
      "name": "Reserved76"
    },
    {
      "code": 6078,
      "name": "Reserved77"
    },
    {
      "code": 6079,
      "name": "Reserved78"
    },
    {
      "code": 6080,
      "name": "Reserved79"
    }
  ]
};

export const IDL: LibreplexMetadata = {
  "version": "0.10.0",
  "name": "libreplex_metadata",
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
          "name": "groupInput",
          "type": {
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "updateGroup",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "groupInput",
          "type": {
            "defined": "GroupInput"
          }
        }
      ]
    },
    {
      "name": "updateMetadata",
      "accounts": [
        {
          "name": "editor",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
            "defined": "UpdateMetadataInput"
          }
        }
      ]
    },
    {
      "name": "groupAdd",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "group_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "groupRemove",
      "accounts": [
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "group_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updatePermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userPermissions",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
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
      "name": "delegateGroupPermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": true,
          "isSigner": true
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
                "path": "delegated_user"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
        },
        {
          "name": "delegatedUser",
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
          "name": "editPermissionsInput",
          "type": {
            "defined": "EditPermissionsInput"
          }
        }
      ]
    },
    {
      "name": "delegateMetadataPermissions",
      "accounts": [
        {
          "name": "updateAuthority",
          "isMut": true,
          "isSigner": true
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
                "path": "delegated_user"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "metadata",
          "isMut": false,
          "isSigner": false,
          "relations": [
            "update_authority"
          ]
        },
        {
          "name": "delegatedUser",
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
          "name": "editPermissionsInput",
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
          "name": "payer",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "invokedMigratorProgram",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createInscriptionMetadata",
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
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "metadataInput",
          "type": {
            "defined": "CreateMetadataInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "updateInscriptionDatatype",
      "accounts": [
        {
          "name": "editor",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "delegatedGroupWidePermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "editor"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Group",
                "path": "group"
              }
            ]
          }
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "inscriptionInput",
          "type": {
            "defined": "UpdateInscriptionDataTypeInput"
          }
        }
      ]
    },
    {
      "name": "deleteMetadataInscription",
      "accounts": [
        {
          "name": "metadataAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "delegatedMetadataSpecificPermissions",
          "isMut": false,
          "isSigner": false,
          "isOptional": true,
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
                "path": "metadata_authority"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Metadata",
                "path": "metadata.update_authority"
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
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionsProgram",
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
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "permissions",
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
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "group",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
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
            "name": "description",
            "type": "string"
          },
          {
            "name": "templateConfiguration",
            "type": {
              "defined": "TemplateConfiguration"
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
      "name": "metadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
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
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "group",
            "type": {
              "option": "publicKey"
            }
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
            "name": "asset",
            "type": {
              "defined": "Asset"
            }
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
            }
          }
        ]
      }
    },
    {
      "name": "delegatePermissions",
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
          },
          {
            "name": "updateAuthority",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateMetadataInscriptionInput",
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
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
            }
          },
          {
            "name": "description",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "dataType",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "UpdateInscriptionDataTypeInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dataType",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "EditPermissionsInput",
      "type": {
        "kind": "struct",
        "fields": [
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
    },
    {
      "name": "Hoo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "t",
            "type": "string"
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
            "name": "description",
            "type": "string"
          },
          {
            "name": "templateConfiguration",
            "type": {
              "defined": "TemplateConfiguration"
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
      "name": "MetadataExtensionInput",
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
            "name": "asset",
            "type": {
              "defined": "Asset"
            }
          },
          {
            "name": "updateAuthority",
            "type": "publicKey"
          },
          {
            "name": "extension",
            "type": {
              "defined": "MetadataExtension"
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
            "name": "asset",
            "type": {
              "defined": "Asset"
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
      "name": "TemplateConfiguration",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Template",
            "fields": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "image_url",
                "type": "string"
              },
              {
                "name": "description",
                "type": "string"
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
      "name": "GroupEventType",
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
    },
    {
      "name": "License",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NoLicense"
          },
          {
            "name": "Custom",
            "fields": [
              {
                "name": "license_url",
                "type": "string"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "MetadataExtension",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Nft",
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
                "name": "license",
                "type": {
                  "option": {
                    "defined": "License"
                  }
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "Asset",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
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
            "name": "JsonTemplate",
            "fields": [
              {
                "name": "url_parameter",
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
              },
              {
                "name": "description",
                "type": {
                  "option": "string"
                }
              }
            ]
          },
          {
            "name": "ChainRenderer",
            "fields": [
              {
                "name": "render_output_address",
                "type": "publicKey"
              },
              {
                "name": "program_id",
                "type": "publicKey"
              },
              {
                "name": "description",
                "type": {
                  "option": "string"
                }
              }
            ]
          },
          {
            "name": "Inscription",
            "fields": [
              {
                "name": "account_id",
                "type": "publicKey"
              },
              {
                "name": "data_type",
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
        ]
      }
    },
    {
      "name": "MetadataEventType",
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
            "name": "Update"
          },
          {
            "name": "Delete"
          },
          {
            "name": "AddToGroup"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeleteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
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
      "name": "GroupEvent",
      "fields": [
        {
          "name": "eventType",
          "type": {
            "defined": "GroupEventType"
          },
          "index": false
        },
        {
          "name": "authority",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "name",
          "type": "string",
          "index": false
        },
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MetadataEvent",
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
          "name": "eventType",
          "type": {
            "defined": "MetadataEventType"
          },
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
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ArithmeticError",
      "msg": "failed to perform some math operation safely"
    },
    {
      "code": 6001,
      "name": "UnknownInstruction",
      "msg": "unknown instruction called"
    },
    {
      "code": 6002,
      "name": "InvalidParameter",
      "msg": "invalid parameter passed in"
    },
    {
      "code": 6003,
      "name": "AnchorSerializationIssue",
      "msg": "anchor serialization issue"
    },
    {
      "code": 6004,
      "name": "AmountMismatch",
      "msg": "two amounts that are supposed to be equal are not"
    },
    {
      "code": 6005,
      "name": "AccountDiscriminatorMismatch",
      "msg": "account discriminator doesn't match"
    },
    {
      "code": 6006,
      "name": "Reserved6"
    },
    {
      "code": 6007,
      "name": "Reserved7"
    },
    {
      "code": 6008,
      "name": "Reserved8"
    },
    {
      "code": 6009,
      "name": "Reserved9"
    },
    {
      "code": 6010,
      "name": "Reserved10"
    },
    {
      "code": 6011,
      "name": "InvalidStringInput",
      "msg": "A constraint on max string length was violated"
    },
    {
      "code": 6012,
      "name": "InvalidBpsInput",
      "msg": "The value of the basis points input must not exceed 10,000"
    },
    {
      "code": 6013,
      "name": "InvalidPermissions",
      "msg": "Invalid Permissions"
    },
    {
      "code": 6014,
      "name": "MissingPermissionAdmin",
      "msg": "Missing admin permission"
    },
    {
      "code": 6015,
      "name": "MissingPermissionEditCollection",
      "msg": "Missing edit collection permission"
    },
    {
      "code": 6016,
      "name": "MissingPermissionDeleteCollection",
      "msg": "Missing delete collection permission"
    },
    {
      "code": 6017,
      "name": "MissingPermissionCreateMetadata",
      "msg": "Missing create metadata permission"
    },
    {
      "code": 6018,
      "name": "MissingPermissionEditMetadata",
      "msg": "Missing edit metadata permission"
    },
    {
      "code": 6019,
      "name": "MissingPermissionDeleteMetadata",
      "msg": "Missing delete metadata permission"
    },
    {
      "code": 6020,
      "name": "CollectionExists",
      "msg": "Collection exists"
    },
    {
      "code": 6021,
      "name": "IncompatibleMetadataType",
      "msg": "Incompatible metadata type"
    },
    {
      "code": 6022,
      "name": "CollectionHasItems",
      "msg": "Collection has items"
    },
    {
      "code": 6023,
      "name": "PermissionAccountEmpty",
      "msg": "Permission account is empty"
    },
    {
      "code": 6024,
      "name": "InvalidBump",
      "msg": "Invalid bump"
    },
    {
      "code": 6025,
      "name": "RoyaltiesBadSum",
      "msg": "Royalties must add up to 10000"
    },
    {
      "code": 6026,
      "name": "UnexpectedPermissionsKey",
      "msg": "Unexpected permission is empty"
    },
    {
      "code": 6027,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    },
    {
      "code": 6028,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6029,
      "name": "MetadataBelongsToGroup",
      "msg": "Metadata belongs to a group"
    },
    {
      "code": 6030,
      "name": "DerivedKeyInvalid",
      "msg": "Derived key invalid"
    },
    {
      "code": 6031,
      "name": "InvalidSignedProgram",
      "msg": "Invalid signer program"
    },
    {
      "code": 6032,
      "name": "MetadataDoesNotHaveAGroup",
      "msg": "Metadata does not have a group"
    },
    {
      "code": 6033,
      "name": "MetadataIsNotMutable",
      "msg": "Metadata is not mutable"
    },
    {
      "code": 6034,
      "name": "InvokeDeleteInscriptionMetadata",
      "msg": "Inscription metadata is deleted via a separate method"
    },
    {
      "code": 6035,
      "name": "OnlyUsedForInscriptionMetadata",
      "msg": "Only used for inscription metadata"
    },
    {
      "code": 6036,
      "name": "WrongAssetType",
      "msg": "Wrong asset type"
    },
    {
      "code": 6037,
      "name": "InvalidMetadataPointer",
      "msg": "Invalid metadata pointer"
    },
    {
      "code": 6038,
      "name": "Reserved37"
    },
    {
      "code": 6039,
      "name": "Reserved38"
    },
    {
      "code": 6040,
      "name": "Reserved39"
    },
    {
      "code": 6041,
      "name": "Reserved40"
    },
    {
      "code": 6042,
      "name": "Reserved41"
    },
    {
      "code": 6043,
      "name": "Reserved42"
    },
    {
      "code": 6044,
      "name": "Reserved43"
    },
    {
      "code": 6045,
      "name": "Reserved44"
    },
    {
      "code": 6046,
      "name": "Reserved45"
    },
    {
      "code": 6047,
      "name": "Reserved46"
    },
    {
      "code": 6048,
      "name": "Reserved47"
    },
    {
      "code": 6049,
      "name": "Reserved48"
    },
    {
      "code": 6050,
      "name": "Reserved49"
    },
    {
      "code": 6051,
      "name": "Reserved50"
    },
    {
      "code": 6052,
      "name": "Reserved51"
    },
    {
      "code": 6053,
      "name": "Reserved52"
    },
    {
      "code": 6054,
      "name": "Reserved53"
    },
    {
      "code": 6055,
      "name": "Reserved54"
    },
    {
      "code": 6056,
      "name": "Reserved55"
    },
    {
      "code": 6057,
      "name": "Reserved56"
    },
    {
      "code": 6058,
      "name": "Reserved57"
    },
    {
      "code": 6059,
      "name": "Reserved58"
    },
    {
      "code": 6060,
      "name": "Reserved59"
    },
    {
      "code": 6061,
      "name": "Reserved60"
    },
    {
      "code": 6062,
      "name": "Reserved61"
    },
    {
      "code": 6063,
      "name": "Reserved62"
    },
    {
      "code": 6064,
      "name": "Reserved63"
    },
    {
      "code": 6065,
      "name": "Reserved64"
    },
    {
      "code": 6066,
      "name": "Reserved65"
    },
    {
      "code": 6067,
      "name": "Reserved66"
    },
    {
      "code": 6068,
      "name": "Reserved67"
    },
    {
      "code": 6069,
      "name": "Reserved68"
    },
    {
      "code": 6070,
      "name": "Reserved69"
    },
    {
      "code": 6071,
      "name": "Reserved70"
    },
    {
      "code": 6072,
      "name": "Reserved71"
    },
    {
      "code": 6073,
      "name": "Reserved72"
    },
    {
      "code": 6074,
      "name": "Reserved73"
    },
    {
      "code": 6075,
      "name": "Reserved74"
    },
    {
      "code": 6076,
      "name": "Reserved75"
    },
    {
      "code": 6077,
      "name": "Reserved76"
    },
    {
      "code": 6078,
      "name": "Reserved77"
    },
    {
      "code": 6079,
      "name": "Reserved78"
    },
    {
      "code": 6080,
      "name": "Reserved79"
    }
  ]
};
