/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_default_renderer.json`.
 */
export type LibreplexDefaultRenderer = {
  "address": "rndbQFmFfiQ4tTG9QPhfJeq1J9fLCSBMAPj3EbLQ7Co",
  "metadata": {
    "name": "libreplexDefaultRenderer",
    "version": "0.1.2",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "canonical",
      "discriminator": [
        233,
        11,
        68,
        244,
        108,
        63,
        142,
        79
      ],
      "accounts": [
        {
          "name": "metadata"
        },
        {
          "name": "mint"
        },
        {
          "name": "group"
        },
        {
          "name": "renderState",
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
          "name": "outputAccount"
        }
      ],
      "args": [
        {
          "name": "renderInput",
          "type": {
            "defined": {
              "name": "renderInput"
            }
          }
        }
      ],
      "returns": "bytes"
    }
  ],
  "types": [
    {
      "name": "renderInput",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ]
};
