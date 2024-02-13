export type LibreplexMonoswap = {
  "version": "0.0.0",
  "name": "libreplex_monoswap",
  "instructions": [
    {
      "name": "createMonoswap",
      "accounts": [
        {
          "name": "swapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "namespace"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_incoming"
              }
            ]
          }
        },
        {
          "name": "mintIncoming",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoing",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintOutgoingOwner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "namespace",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "swapperProgram",
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
            "defined": "CreateMonoSwapInput"
          }
        }
      ]
    },
    {
      "name": "swap",
      "accounts": [
        {
          "name": "swapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "swapper_program"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_incoming"
              }
            ]
          }
        },
        {
          "name": "swapMarkerReverse",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "swapping always creates a symmetrical swap marker that enables a swap back"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "swapper_program"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_outgoing"
              }
            ]
          }
        },
        {
          "name": "mintIncoming",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintOutgoing",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintIncomingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintIncomingTokenAccountTarget",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountTarget",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram2022",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "swapperProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "swapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "namespace",
            "type": "publicKey"
          },
          {
            "name": "mintIncoming",
            "type": "publicKey"
          },
          {
            "name": "mintOutgoing",
            "type": "publicKey"
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
  ],
  "types": [
    {
      "name": "CreateMonoSwapInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintOutgoingAmount",
            "type": "u64"
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
    },
    {
      "code": 6009,
      "name": "DataHashMismatch",
      "msg": "Metadata data missmatch"
    }
  ]
};

export const IDL: LibreplexMonoswap = {
  "version": "0.0.0",
  "name": "libreplex_monoswap",
  "instructions": [
    {
      "name": "createMonoswap",
      "accounts": [
        {
          "name": "swapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "namespace"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_incoming"
              }
            ]
          }
        },
        {
          "name": "mintIncoming",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoing",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintOutgoingOwner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "namespace",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "swapperProgram",
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
            "defined": "CreateMonoSwapInput"
          }
        }
      ]
    },
    {
      "name": "swap",
      "accounts": [
        {
          "name": "swapMarker",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "swapper_program"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_incoming"
              }
            ]
          }
        },
        {
          "name": "swapMarkerReverse",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "swapping always creates a symmetrical swap marker that enables a swap back"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "swap_marker"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "swapper_program"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint_outgoing"
              }
            ]
          }
        },
        {
          "name": "mintIncoming",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintOutgoing",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintIncomingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintIncomingTokenAccountTarget",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountSource",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintOutgoingTokenAccountTarget",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram2022",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "swapperProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "swapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "namespace",
            "type": "publicKey"
          },
          {
            "name": "mintIncoming",
            "type": "publicKey"
          },
          {
            "name": "mintOutgoing",
            "type": "publicKey"
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
  ],
  "types": [
    {
      "name": "CreateMonoSwapInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintOutgoingAmount",
            "type": "u64"
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
    },
    {
      "code": 6009,
      "name": "DataHashMismatch",
      "msg": "Metadata data missmatch"
    }
  ]
};
