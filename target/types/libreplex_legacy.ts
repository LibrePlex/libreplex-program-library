export type LibreplexLegacy = {
  "version": "0.2.0",
  "name": "libreplex_legacy",
  "instructions": [
    {
      "name": "claimExcessRentAsUauth",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyInscription",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "is passed on to inscriptions program as the authority of the INSCRIPTION"
          ]
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
      "name": "setValidationHash",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
          "name": "validationHash",
          "type": {
            "option": "string"
          }
        }
      ]
    },
    {
      "name": "inscribeLegacyMetadataAsUauthV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "legacySigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
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
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
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
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "validationHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "writeToLegacyInscriptionAsUauthV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
          "name": "input",
          "type": {
            "defined": "WriteToInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "resizeLegacyInscriptionAsUauthV3",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyInscription",
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
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ResizeLegacyInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "makeLegacyInscriptionImmutableV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
    }
  ],
  "accounts": [
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
      "name": "legacyInscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "inscription",
            "type": "publicKey"
          },
          {
            "name": "legacyType",
            "type": {
              "defined": "LegacyType"
            }
          },
          {
            "name": "authorityType",
            "type": {
              "defined": "AuthorityType"
            }
          }
        ]
      }
    }
  ],
  "types": [
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
      "name": "AuthorityType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Holder"
          },
          {
            "name": "UpdateAuthority"
          }
        ]
      }
    },
    {
      "name": "ResizeLegacyInscriptionInput",
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
      "name": "LegacyType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MetaplexMint"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "CannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "BadAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "BadAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "MultiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "NotSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "Inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "InscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    }
  ]
};

export const IDL: LibreplexLegacy = {
  "version": "0.2.0",
  "name": "libreplex_legacy",
  "instructions": [
    {
      "name": "claimExcessRentAsUauth",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyInscription",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "is passed on to inscriptions program as the authority of the INSCRIPTION"
          ]
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
      "name": "setValidationHash",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
          "name": "validationHash",
          "type": {
            "option": "string"
          }
        }
      ]
    },
    {
      "name": "inscribeLegacyMetadataAsUauthV3",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "legacySigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
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
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
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
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "validationHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "writeToLegacyInscriptionAsUauthV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
          "name": "input",
          "type": {
            "defined": "WriteToInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "resizeLegacyInscriptionAsUauthV3",
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
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
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
          "name": "legacyInscription",
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
          "name": "inscriptionsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ResizeLegacyInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "makeLegacyInscriptionImmutableV3",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inscriptionV3",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inscriptionSummary",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "legacyInscription",
          "isMut": true,
          "isSigner": false
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
    }
  ],
  "accounts": [
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
      "name": "legacyInscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "inscription",
            "type": "publicKey"
          },
          {
            "name": "legacyType",
            "type": {
              "defined": "LegacyType"
            }
          },
          {
            "name": "authorityType",
            "type": {
              "defined": "AuthorityType"
            }
          }
        ]
      }
    }
  ],
  "types": [
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
      "name": "AuthorityType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Holder"
          },
          {
            "name": "UpdateAuthority"
          }
        ]
      }
    },
    {
      "name": "ResizeLegacyInscriptionInput",
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
      "name": "LegacyType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MetaplexMint"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "CannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "BadAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "BadAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "MultiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "NotSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "Inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "InscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    }
  ]
};
