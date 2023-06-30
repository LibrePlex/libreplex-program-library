export type Toybox = {
  "version": "0.1.0",
  "name": "toybox",
  "instructions": [],
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
      "name": "CreateMakerInput",
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

export const IDL: Toybox = {
  "version": "0.1.0",
  "name": "toybox",
  "instructions": [],
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
      "name": "CreateMakerInput",
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
