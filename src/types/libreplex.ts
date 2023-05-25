export type Libreplex = {
  "version": "0.1.0",
  "name": "libreplex",
  "instructions": [
    {
      "name": "createMetadataSpl",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "imageUrl",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        }
      ]
    },
    {
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "metadataOverride",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
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
      "name": "createMetadataNft",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
          "name": "mint",
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "offchainUrl",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "attributes",
          "type": {
            "vec": {
              "defined": "Attribute"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "royaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "share",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "verification",
      "type": {
        "kind": "struct",
        "fields": []
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
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "tokenType",
            "type": "u8"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "MetadataNft"
              }
            }
          }
        ]
      }
    },
    {
      "name": "metadataNft",
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
      "name": "metadataNftOverride",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadataNft",
            "type": "publicKey"
          },
          {
            "name": "royaltyBpsOverride",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "vec": {
                  "defined": "RoyaltyShare"
                }
              }
            }
          },
          {
            "name": "permittedSigners",
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
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
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
            "name": "itemCount",
            "type": "u32"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "CollectionNftData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "collectionNftData",
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidBump",
      "msg": "Bad bump"
    },
    {
      "code": 6001,
      "name": "MissingBump",
      "msg": "Missing bump"
    },
    {
      "code": 6002,
      "name": "CannotRemoveVerifiedCreator",
      "msg": "Cannot remove verified creator"
    },
    {
      "code": 6003,
      "name": "CannotAddVerifiedCreator",
      "msg": "Cannot add verified creator"
    },
    {
      "code": 6004,
      "name": "NoCollectionSet",
      "msg": "Cannot verify a collection when one is not set"
    },
    {
      "code": 6005,
      "name": "IncorrectCollectionAuthority",
      "msg": "Incorrect collection authority"
    },
    {
      "code": 6006,
      "name": "CannotDeleteCollectionWithVerifiedItems",
      "msg": "Collection has verified items. Cannot delete."
    },
    {
      "code": 6007,
      "name": "SignerNotInCreatorArray",
      "msg": "Signer not in creator array."
    },
    {
      "code": 6008,
      "name": "AlreadySigned",
      "msg": "This signer has already signed this metadata."
    },
    {
      "code": 6009,
      "name": "MustDeleteOverrideFirst",
      "msg": "Before deleting an NFT metadata, you must delete the override account first."
    },
    {
      "code": 6010,
      "name": "MetadataIsNotMutable",
      "msg": "Metadata is not mutable."
    },
    {
      "code": 6011,
      "name": "CannotSignNonNftMetadata",
      "msg": "Cannot sign non-NFT metadata."
    },
    {
      "code": 6012,
      "name": "CannotSignItemInNonNftCollection",
      "msg": "Cannot sign item in non NFT collection."
    },
    {
      "code": 6013,
      "name": "NotNftCollection",
      "msg": "Not NFT collection."
    },
    {
      "code": 6014,
      "name": "NotSplCollection",
      "msg": "Not SPL collection."
    }
  ]
};

export const IDL: Libreplex = {
  "version": "0.1.0",
  "name": "libreplex",
  "instructions": [
    {
      "name": "createMetadataSpl",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "imageUrl",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        }
      ]
    },
    {
      "name": "deleteMetadata",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "metadataOverride",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
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
      "name": "createMetadataNft",
      "accounts": [
        {
          "name": "authority",
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
                "account": "Mint",
                "path": "mint"
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
          "name": "mint",
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "offchainUrl",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "attributes",
          "type": {
            "vec": {
              "defined": "Attribute"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "royaltyShare",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "share",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "verification",
      "type": {
        "kind": "struct",
        "fields": []
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
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "tokenType",
            "type": "u8"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "MetadataNft"
              }
            }
          }
        ]
      }
    },
    {
      "name": "metadataNft",
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
      "name": "metadataNftOverride",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadataNft",
            "type": "publicKey"
          },
          {
            "name": "royaltyBpsOverride",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "royalties",
            "type": {
              "option": {
                "vec": {
                  "defined": "RoyaltyShare"
                }
              }
            }
          },
          {
            "name": "permittedSigners",
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
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
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
            "name": "itemCount",
            "type": "u32"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "nftData",
            "type": {
              "option": {
                "defined": "CollectionNftData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "collectionNftData",
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidBump",
      "msg": "Bad bump"
    },
    {
      "code": 6001,
      "name": "MissingBump",
      "msg": "Missing bump"
    },
    {
      "code": 6002,
      "name": "CannotRemoveVerifiedCreator",
      "msg": "Cannot remove verified creator"
    },
    {
      "code": 6003,
      "name": "CannotAddVerifiedCreator",
      "msg": "Cannot add verified creator"
    },
    {
      "code": 6004,
      "name": "NoCollectionSet",
      "msg": "Cannot verify a collection when one is not set"
    },
    {
      "code": 6005,
      "name": "IncorrectCollectionAuthority",
      "msg": "Incorrect collection authority"
    },
    {
      "code": 6006,
      "name": "CannotDeleteCollectionWithVerifiedItems",
      "msg": "Collection has verified items. Cannot delete."
    },
    {
      "code": 6007,
      "name": "SignerNotInCreatorArray",
      "msg": "Signer not in creator array."
    },
    {
      "code": 6008,
      "name": "AlreadySigned",
      "msg": "This signer has already signed this metadata."
    },
    {
      "code": 6009,
      "name": "MustDeleteOverrideFirst",
      "msg": "Before deleting an NFT metadata, you must delete the override account first."
    },
    {
      "code": 6010,
      "name": "MetadataIsNotMutable",
      "msg": "Metadata is not mutable."
    },
    {
      "code": 6011,
      "name": "CannotSignNonNftMetadata",
      "msg": "Cannot sign non-NFT metadata."
    },
    {
      "code": 6012,
      "name": "CannotSignItemInNonNftCollection",
      "msg": "Cannot sign item in non NFT collection."
    },
    {
      "code": 6013,
      "name": "NotNftCollection",
      "msg": "Not NFT collection."
    },
    {
      "code": 6014,
      "name": "NotSplCollection",
      "msg": "Not SPL collection."
    }
  ]
};
