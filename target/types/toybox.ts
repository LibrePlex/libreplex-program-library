export type Toybox = {
  "version": "0.1.0",
  "name": "toybox",
  "instructions": [],
  "accounts": [
    {
      "name": "toybox",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "minted",
            "type": "u64"
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
            "name": "attributes",
            "type": "bytes"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateToyboxInput",
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
            "type": "publicKey"
          },
          {
            "name": "priceQuantity",
            "type": "u64"
          },
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "collection",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Blueprint",
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
      "name": "toybox",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "minted",
            "type": "u64"
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
            "name": "attributes",
            "type": "bytes"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateToyboxInput",
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
            "type": "publicKey"
          },
          {
            "name": "priceQuantity",
            "type": "u64"
          },
          {
            "name": "maxMints",
            "type": "u64"
          },
          {
            "name": "collection",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Blueprint",
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
