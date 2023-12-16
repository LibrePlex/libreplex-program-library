export type LibreplexInscriptions = {
  "version": "0.4.0",
  "name": "libreplex_inscriptions",
  "instructions": [
    {
      "name": "createInscriptionRankPage",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "page",
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
            "defined": "CreateInscriptionRankInput"
          }
        }
      ]
    },
    {
      "name": "makeInscriptionImmutable",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
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
          "name": "inscription2",
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
      "name": "claimExcessRent",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriptionV3",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
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
      "name": "migrateToV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "root",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "migrator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscription2",
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
      "name": "setValidationHash",
      "accounts": [
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
          "name": "inscription",
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
          "name": "validationHash",
          "type": {
            "option": "string"
          }
        }
      ]
    },
    {
      "name": "createInscriptionV3",
      "accounts": [
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
          "name": "root",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
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
          "name": "inscriptionInput",
          "type": {
            "defined": "CreateInscriptionInputV3"
          }
        }
      ]
    },
    {
      "name": "makeInscriptionImmutableV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
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
      "name": "resizeInscriptionV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ResizeInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "writeToInscriptionV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "WriteToInscriptionInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "inscription",
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
            "name": "mediaType",
            "type": {
              "defined": "MediaType"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": "EncodingType"
            }
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionData",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "inscriptionRankPage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "size",
            "type": "u32"
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
    },
    {
      "name": "migrator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "migrator",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SignerType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Root"
          },
          {
            "name": "LegacyMetadataSigner"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionRankInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pageIndex",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ResizeInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "change",
            "type": "i32"
          },
          {
            "name": "expectedStartSize",
            "type": "u32"
          },
          {
            "name": "targetSize",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionInputV3",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "signerType",
            "type": {
              "defined": "SignerType"
            }
          },
          {
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "WriteToInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "bytes"
          },
          {
            "name": "startPos",
            "type": "u32"
          },
          {
            "name": "mediaType",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "EncodingType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Base64"
          }
        ]
      }
    },
    {
      "name": "InscriptionEventData",
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
            "name": "mediaType",
            "type": {
              "defined": "MediaType"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": "EncodingType"
            }
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "InscriptionV3EventData",
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
            "name": "contentType",
            "type": "string"
          },
          {
            "name": "encoding",
            "type": "string"
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "MediaType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Audio",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Application",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Image",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Video",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Text",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Custom",
            "fields": [
              {
                "name": "mediaType",
                "type": "string"
              }
            ]
          },
          {
            "name": "Erc721"
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
      "name": "InscriptionEventCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionEventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionEventUpdate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionEventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionResizeEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "size",
          "type": "u32",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionResizeEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "size",
          "type": "u32",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionV3EventCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionV3EventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionV3EventUpdate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionV3EventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionWriteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionWriteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6001,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    },
    {
      "code": 6002,
      "name": "BadInscriptionRankPage",
      "msg": "Bad page"
    },
    {
      "code": 6003,
      "name": "IncorrectInscriptionDataAccount",
      "msg": "Incorrect inscription data account"
    },
    {
      "code": 6004,
      "name": "RootSignerMismatch",
      "msg": "Root signer mismatch"
    },
    {
      "code": 6005,
      "name": "LegacyMetadataSignerMismatch",
      "msg": "Legacy metadata signer key does not match the expected PDA"
    },
    {
      "code": 6006,
      "name": "MismatchingInscriptions",
      "msg": "Mismatching mints between v1 and v2 inscriptions"
    }
  ]
};

export const IDL: LibreplexInscriptions = {
  "version": "0.4.0",
  "name": "libreplex_inscriptions",
  "instructions": [
    {
      "name": "createInscriptionRankPage",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "page",
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
            "defined": "CreateInscriptionRankInput"
          }
        }
      ]
    },
    {
      "name": "makeInscriptionImmutable",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
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
          "name": "inscription2",
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
      "name": "claimExcessRent",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscriptionV3",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
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
      "name": "migrateToV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "root",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "migrator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscription2",
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
      "name": "setValidationHash",
      "accounts": [
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
          "name": "inscription",
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
          "name": "validationHash",
          "type": {
            "option": "string"
          }
        }
      ]
    },
    {
      "name": "createInscriptionV3",
      "accounts": [
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
          "name": "root",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
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
          "name": "inscriptionInput",
          "type": {
            "defined": "CreateInscriptionInputV3"
          }
        }
      ]
    },
    {
      "name": "makeInscriptionImmutableV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
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
      "name": "resizeInscriptionV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ResizeInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "writeToInscriptionV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "WriteToInscriptionInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "inscription",
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
            "name": "mediaType",
            "type": {
              "defined": "MediaType"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": "EncodingType"
            }
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionData",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "inscriptionRankPage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "size",
            "type": "u32"
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
    },
    {
      "name": "migrator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "migrator",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SignerType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Root"
          },
          {
            "name": "LegacyMetadataSigner"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionRankInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pageIndex",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ResizeInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "change",
            "type": "i32"
          },
          {
            "name": "expectedStartSize",
            "type": "u32"
          },
          {
            "name": "targetSize",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionInputV3",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "signerType",
            "type": {
              "defined": "SignerType"
            }
          },
          {
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "WriteToInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "bytes"
          },
          {
            "name": "startPos",
            "type": "u32"
          },
          {
            "name": "mediaType",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "EncodingType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Base64"
          }
        ]
      }
    },
    {
      "name": "InscriptionEventData",
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
            "name": "mediaType",
            "type": {
              "defined": "MediaType"
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": "EncodingType"
            }
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "InscriptionV3EventData",
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
            "name": "contentType",
            "type": "string"
          },
          {
            "name": "encoding",
            "type": "string"
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
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "MediaType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Audio",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Application",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Image",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Video",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Text",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "Custom",
            "fields": [
              {
                "name": "mediaType",
                "type": "string"
              }
            ]
          },
          {
            "name": "Erc721"
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
      "name": "InscriptionEventCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionEventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionEventUpdate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionEventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionResizeEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "size",
          "type": "u32",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionResizeEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "size",
          "type": "u32",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionV3EventCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionV3EventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionV3EventUpdate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "data",
          "type": {
            "defined": "InscriptionV3EventData"
          },
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionWriteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "InscriptionWriteEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6001,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    },
    {
      "code": 6002,
      "name": "BadInscriptionRankPage",
      "msg": "Bad page"
    },
    {
      "code": 6003,
      "name": "IncorrectInscriptionDataAccount",
      "msg": "Incorrect inscription data account"
    },
    {
      "code": 6004,
      "name": "RootSignerMismatch",
      "msg": "Root signer mismatch"
    },
    {
      "code": 6005,
      "name": "LegacyMetadataSignerMismatch",
      "msg": "Legacy metadata signer key does not match the expected PDA"
    },
    {
      "code": 6006,
      "name": "MismatchingInscriptions",
      "msg": "Mismatching mints between v1 and v2 inscriptions"
    }
  ]
};
