export type Inscriptions = {
  "version": "0.1.0",
  "name": "inscriptions",
  "instructions": [
    {
      "name": "createOrdinal",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ordinal",
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
          "name": "ordinalInput",
          "type": {
            "defined": "CreateOrdinalInput"
          }
        }
      ]
    },
    {
      "name": "appendToOrdinal",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ordinal",
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
            "defined": "AppendToOrdinalInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "ordinal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "dataLengthCurrent",
            "type": "u32"
          },
          {
            "name": "dataLengthMax",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AppendToOrdinalInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "appendData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "CreateOrdinalInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxDataLength",
            "type": "u32"
          },
          {
            "name": "initialData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "OrdinalEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Append"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "OrdinalEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "OrdinalEventType"
          },
          "index": false
        }
      ]
    }
  ]
};

export const IDL: Inscriptions = {
  "version": "0.1.0",
  "name": "inscriptions",
  "instructions": [
    {
      "name": "createOrdinal",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ordinal",
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
          "name": "ordinalInput",
          "type": {
            "defined": "CreateOrdinalInput"
          }
        }
      ]
    },
    {
      "name": "appendToOrdinal",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ordinal",
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
            "defined": "AppendToOrdinalInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "ordinal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "dataLengthCurrent",
            "type": "u32"
          },
          {
            "name": "dataLengthMax",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AppendToOrdinalInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "appendData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "CreateOrdinalInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxDataLength",
            "type": "u32"
          },
          {
            "name": "initialData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "OrdinalEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Append"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "OrdinalEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "OrdinalEventType"
          },
          "index": false
        }
      ]
    }
  ]
};
