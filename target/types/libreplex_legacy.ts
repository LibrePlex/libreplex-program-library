/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_legacy.json`.
 */
export type LibreplexLegacy = {
  "address": "",
  "metadata": {
    "name": "libreplexLegacy",
    "version": "0.2.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/LibrePlex/metadata"
  },
  "instructions": [
    {
      "name": "claimExcessRentAsUauth",
      "discriminator": [
        98,
        206,
        169,
        44,
        204,
        10,
        47,
        111
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
          "name": "mint"
        },
        {
          "name": "legacyMetadata"
        },
        {
          "name": "inscriptionV3"
        },
        {
          "name": "inscriptionData",
          "writable": true
        },
        {
          "name": "legacyInscription",
          "docs": [
            "is passed on to inscriptions program as the authority of the INSCRIPTION"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
        }
      ],
      "args": []
    },
    {
      "name": "inscribeLegacyMetadataAsUauthV3",
      "discriminator": [
        106,
        73,
        39,
        184,
        151,
        74,
        183,
        66
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "legacySigner",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint"
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
          "name": "inscriptionSummary",
          "writable": true
        },
        {
          "name": "legacyInscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "legacyMetadata"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
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
      "name": "makeLegacyInscriptionImmutableV3",
      "discriminator": [
        162,
        51,
        73,
        185,
        59,
        65,
        177,
        64
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "inscriptionV3",
          "writable": true
        },
        {
          "name": "inscriptionSummary",
          "writable": true
        },
        {
          "name": "legacyMetadata"
        },
        {
          "name": "legacyInscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
        }
      ],
      "args": []
    },
    {
      "name": "resizeLegacyInscriptionAsUauthV3",
      "discriminator": [
        69,
        5,
        7,
        55,
        175,
        204,
        134,
        9
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
          "name": "mint"
        },
        {
          "name": "legacyMetadata"
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
          "name": "legacyInscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "resizeLegacyInscriptionInput"
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
          "name": "mint"
        },
        {
          "name": "inscription",
          "writable": true
        },
        {
          "name": "legacyMetadata"
        },
        {
          "name": "legacyInscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
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
      "name": "writeToLegacyInscriptionAsUauthV3",
      "discriminator": [
        241,
        255,
        17,
        224,
        137,
        137,
        5,
        221
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
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
          "name": "legacyMetadata"
        },
        {
          "name": "legacyInscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  101,
                  103,
                  97,
                  99,
                  121,
                  95,
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
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "inscriptionsProgram",
          "address": "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp"
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
      "name": "legacyInscription",
      "discriminator": [
        0,
        39,
        45,
        80,
        158,
        103,
        99,
        146
      ]
    }
  ],
  "types": [
    {
      "name": "authorityType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "holder"
          },
          {
            "name": "updateAuthority"
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
      "name": "legacyInscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "inscription",
            "type": "pubkey"
          },
          {
            "name": "legacyType",
            "type": {
              "defined": {
                "name": "legacyType"
              }
            }
          },
          {
            "name": "authorityType",
            "type": {
              "defined": {
                "name": "authorityType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "legacyType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "metaplexMint"
          }
        ]
      }
    },
    {
      "name": "resizeLegacyInscriptionInput",
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
    }
  ]
};
