/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_monoswap.json`.
 */
export type LibreplexMonoswap = {
  "address": "MonoRPwMWxcsVEJV27jyEt1f5VoWg3szDBRYUenm221",
  "metadata": {
    "name": "libreplexMonoswap",
    "version": "0.0.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/LibrePlex/metadata"
  },
  "instructions": [
    {
      "name": "createMonoswap",
      "discriminator": [
        98,
        245,
        160,
        178,
        252,
        5,
        159,
        225
      ],
      "accounts": [
        {
          "name": "swapMarker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  109,
                  97,
                  114,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "namespace"
              },
              {
                "kind": "account",
                "path": "mintOutgoing"
              },
              {
                "kind": "account",
                "path": "mintIncoming"
              }
            ]
          }
        },
        {
          "name": "mintIncoming",
          "writable": true
        },
        {
          "name": "mintOutgoing"
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "writable": true
        },
        {
          "name": "escrowHolder",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "namespace"
              },
              {
                "kind": "account",
                "path": "mintIncoming"
              }
            ]
          }
        },
        {
          "name": "mintOutgoingTokenAccountEscrow",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "mintOutgoingOwner",
          "writable": true,
          "signer": true
        },
        {
          "name": "namespace",
          "signer": true
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
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
              "name": "createMonoSwapInput"
            }
          }
        }
      ]
    },
    {
      "name": "swap",
      "discriminator": [
        248,
        198,
        158,
        145,
        225,
        117,
        135,
        200
      ],
      "accounts": [
        {
          "name": "swapMarker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  109,
                  97,
                  114,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "swap_marker.namespace",
                "account": "swapMarker"
              },
              {
                "kind": "account",
                "path": "mintOutgoing"
              },
              {
                "kind": "account",
                "path": "mintIncoming"
              }
            ]
          }
        },
        {
          "name": "swapMarkerReverse",
          "docs": [
            "swapping always creates a symmetrical swap marker that enables a swap back"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  109,
                  97,
                  114,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "swap_marker.namespace",
                "account": "swapMarker"
              },
              {
                "kind": "account",
                "path": "mintIncoming"
              },
              {
                "kind": "account",
                "path": "mintOutgoing"
              }
            ]
          }
        },
        {
          "name": "mintIncoming"
        },
        {
          "name": "mintOutgoing"
        },
        {
          "name": "mintIncomingTokenAccountSource",
          "writable": true
        },
        {
          "name": "escrowHolder",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "swap_marker.namespace",
                "account": "swapMarker"
              },
              {
                "kind": "account",
                "path": "mintIncoming"
              }
            ]
          }
        },
        {
          "name": "escrowHolderReverse",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "swap_marker.namespace",
                "account": "swapMarker"
              },
              {
                "kind": "account",
                "path": "mintOutgoing"
              }
            ]
          }
        },
        {
          "name": "mintIncomingTokenAccountTarget",
          "writable": true
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "writable": true
        },
        {
          "name": "mintOutgoingTokenAccountTarget",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenProgram2022",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "swapMarker",
      "discriminator": [
        186,
        7,
        231,
        231,
        117,
        67,
        107,
        191
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "badMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "cannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "badAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "badAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "badAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "multiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "notSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "inscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    },
    {
      "code": 6009,
      "name": "dataHashMismatch",
      "msg": "Metadata data missmatch"
    }
  ],
  "types": [
    {
      "name": "createMonoSwapInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintOutgoingAmount",
            "type": "u64"
          },
          {
            "name": "mintIncomingAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "swapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "namespace",
            "type": "pubkey"
          },
          {
            "name": "mintIncoming",
            "type": "pubkey"
          },
          {
            "name": "mintOutgoing",
            "type": "pubkey"
          },
          {
            "name": "mintIncomingAmount",
            "type": "u64"
          },
          {
            "name": "mintOutgoingAmount",
            "type": "u64"
          },
          {
            "name": "used",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
