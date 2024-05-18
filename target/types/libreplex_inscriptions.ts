/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_inscriptions.json`.
 */
export type LibreplexInscriptions = {
  "address": "",
  "metadata": {
    "name": "libreplexInscriptions",
    "version": "0.5.0",
    "spec": "0.1.0",
    "description": "Inscriptions from LibrePlex",
    "repository": "https://github.com/LibrePlex/libreplex-program-library"
  },
  "instructions": [
    {
      "name": "claimExcessRent",
      "discriminator": [
        106,
        43,
        164,
        6,
        220,
        74,
        225,
        17
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "inscriptionV3"
        },
        {
          "name": "inscriptionData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "inscription_v3.root",
                "account": "inscriptionV3"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createGhostRootInscription",
      "discriminator": [
        168,
        13,
        68,
        18,
        130,
        171,
        33,
        121
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "inscriptionSummary",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  117,
                  109,
                  109,
                  97,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "inscriptionData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "arg",
                "path": "inscription_input.root"
              }
            ]
          }
        },
        {
          "name": "inscriptionV3",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  118,
                  51
                ]
              },
              {
                "kind": "arg",
                "path": "inscription_input.root"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "inscriptionInput",
          "type": {
            "defined": {
              "name": "createGhostRootInscriptionInput"
            }
          }
        }
      ]
    },
    {
      "name": "createInscriptionRankPage",
      "discriminator": [
        250,
        76,
        231,
        197,
        254,
        132,
        31,
        136
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "page",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  114,
                  97,
                  110,
                  107
                ]
              },
              {
                "kind": "arg",
                "path": "input.page_index"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "createInscriptionRankInput"
            }
          }
        }
      ]
    },
    {
      "name": "createInscriptionV3",
      "discriminator": [
        168,
        43,
        77,
        230,
        240,
        247,
        161,
        175
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "root"
        },
        {
          "name": "inscriptionSummary",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  117,
                  109,
                  109,
                  97,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "inscriptionData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "root"
              }
            ]
          }
        },
        {
          "name": "inscriptionV3",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  118,
                  51
                ]
              },
              {
                "kind": "account",
                "path": "root"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "inscriptionInput",
          "type": {
            "defined": {
              "name": "createInscriptionInputV3"
            }
          }
        }
      ]
    },
    {
      "name": "makeInscriptionImmutable",
      "discriminator": [
        161,
        66,
        113,
        199,
        96,
        233,
        223,
        172
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "inscriptionSummary",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  117,
                  109,
                  109,
                  97,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "inscription",
          "writable": true
        },
        {
          "name": "inscription2",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "makeInscriptionImmutableV3",
      "discriminator": [
        98,
        71,
        230,
        185,
        46,
        129,
        233,
        48
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "inscriptionSummary",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  117,
                  109,
                  109,
                  97,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "inscriptionV3",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "migrateToV3",
      "discriminator": [
        255,
        126,
        224,
        187,
        202,
        169,
        102,
        214
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "root"
        },
        {
          "name": "migrator",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  103,
                  114,
                  97,
                  116,
                  111,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "root"
              }
            ]
          }
        },
        {
          "name": "inscription",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "root"
              }
            ]
          }
        },
        {
          "name": "inscription2",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  118,
                  51
                ]
              },
              {
                "kind": "account",
                "path": "root"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "resizeInscriptionV3",
      "discriminator": [
        129,
        45,
        209,
        5,
        231,
        155,
        217,
        211
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "inscriptionV3",
          "writable": true
        },
        {
          "name": "inscriptionData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110,
                  95,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "inscription_v3.root",
                "account": "inscriptionV3"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "resizeInscriptionInput"
            }
          }
        }
      ]
    },
    {
      "name": "setValidationHash",
      "discriminator": [
        220,
        65,
        43,
        233,
        137,
        215,
        187,
        229
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "inscription",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
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
      "name": "writeToInscriptionV3",
      "discriminator": [
        118,
        248,
        99,
        188,
        226,
        144,
        151,
        46
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "inscriptionV3",
          "writable": true
        },
        {
          "name": "inscriptionData",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "writeToInscriptionInput"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "inscription",
      "discriminator": [
        100,
        11,
        151,
        42,
        228,
        38,
        69,
        187
      ]
    },
    {
      "name": "inscriptionData",
      "discriminator": [
        44,
        58,
        66,
        156,
        82,
        136,
        80,
        64
      ]
    },
    {
      "name": "inscriptionRankPage",
      "discriminator": [
        192,
        133,
        192,
        195,
        42,
        49,
        155,
        198
      ]
    },
    {
      "name": "inscriptionSummary",
      "discriminator": [
        189,
        189,
        190,
        90,
        73,
        71,
        253,
        107
      ]
    },
    {
      "name": "inscriptionV3",
      "discriminator": [
        232,
        120,
        205,
        47,
        153,
        239,
        229,
        224
      ]
    },
    {
      "name": "migrator",
      "discriminator": [
        74,
        71,
        185,
        52,
        75,
        186,
        114,
        78
      ]
    }
  ],
  "types": [
    {
      "name": "createGhostRootInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "signerType",
            "type": {
              "defined": {
                "name": "signerType"
              }
            }
          },
          {
            "name": "validationHash",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "root",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "createInscriptionInputV3",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "signerType",
            "type": {
              "defined": {
                "name": "signerType"
              }
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
      "name": "createInscriptionRankInput",
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
      "name": "encodingType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "base64"
          }
        ]
      }
    },
    {
      "name": "inscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": "pubkey"
          },
          {
            "name": "mediaType",
            "type": {
              "defined": {
                "name": "mediaType"
              }
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": {
                "name": "encodingType"
              }
            }
          },
          {
            "name": "inscriptionData",
            "type": "pubkey"
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
      "name": "inscriptionEventCreate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "data",
            "type": {
              "defined": {
                "name": "inscriptionEventData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionEventData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": "pubkey"
          },
          {
            "name": "mediaType",
            "type": {
              "defined": {
                "name": "mediaType"
              }
            }
          },
          {
            "name": "encodingType",
            "type": {
              "defined": {
                "name": "encodingType"
              }
            }
          },
          {
            "name": "inscriptionData",
            "type": "pubkey"
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
      "name": "inscriptionEventUpdate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "data",
            "type": {
              "defined": {
                "name": "inscriptionEventData"
              }
            }
          }
        ]
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
            "type": "pubkey"
          },
          {
            "name": "lastInscriber",
            "type": "pubkey"
          },
          {
            "name": "lastInscriptionCreateTime",
            "type": "i64"
          },
          {
            "name": "extension",
            "type": {
              "defined": {
                "name": "summaryExtension"
              }
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
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": "pubkey"
          },
          {
            "name": "inscriptionData",
            "type": "pubkey"
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
      "name": "inscriptionV3EventCreate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "data",
            "type": {
              "defined": {
                "name": "inscriptionV3EventData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "inscriptionV3EventData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": "pubkey"
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
            "type": "pubkey"
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
      "name": "inscriptionV3EventUpdate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "data",
            "type": {
              "defined": {
                "name": "inscriptionV3EventData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "mediaType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "audio",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "application",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "image",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "video",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "text",
            "fields": [
              {
                "name": "subtype",
                "type": "string"
              }
            ]
          },
          {
            "name": "custom",
            "fields": [
              {
                "name": "mediaType",
                "type": "string"
              }
            ]
          },
          {
            "name": "erc721"
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
            "type": "pubkey"
          },
          {
            "name": "migrator",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "resizeInscriptionInput",
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
      "name": "signerType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "root"
          },
          {
            "name": "legacyMetadataSigner"
          },
          {
            "name": "fairLaunchGhostRootSigner"
          }
        ]
      }
    },
    {
      "name": "summaryExtension",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          }
        ]
      }
    },
    {
      "name": "writeToInscriptionInput",
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
      "name": "libreplex_inscriptions::instructions::resize_inscription::InscriptionResizeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "size",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "libreplex_inscriptions::instructions::v3::resize_inscription_v3::InscriptionResizeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "size",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "libreplex_inscriptions::instructions::v3::write_to_inscription_v3::InscriptionWriteEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "libreplex_inscriptions::instructions::write_to_inscription::InscriptionWriteEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
