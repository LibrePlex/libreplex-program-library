export type LibreplexNft = {
  "version": "0.10.0",
  "name": "libreplex_nft",
  "instructions": [
    {
      "name": "wrap",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedMint",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "toggleFreeze",
      "accounts": [
        {
          "name": "delegate",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ToggleFreezeInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "wrappedMint",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ],
  "types": [
    {
      "name": "ToggleFreezeInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Freeze"
          },
          {
            "name": "Unfreeze"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidMintAuthority",
      "msg": "InvalidMintAuthority"
    },
    {
      "code": 6001,
      "name": "MintCannotRepresentNFT",
      "msg": "MintCannotRepresentNFT"
    },
    {
      "code": 6002,
      "name": "InvalidMint",
      "msg": "InvalidMint"
    },
    {
      "code": 6003,
      "name": "InvalidTokenAccount",
      "msg": "InvalidTokenAccount"
    }
  ]
};

export const IDL: LibreplexNft = {
  "version": "0.10.0",
  "name": "libreplex_nft",
  "instructions": [
    {
      "name": "wrap",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedMint",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "toggleFreeze",
      "accounts": [
        {
          "name": "delegate",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wrappedMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "ToggleFreezeInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "wrappedMint",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ],
  "types": [
    {
      "name": "ToggleFreezeInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Freeze"
          },
          {
            "name": "Unfreeze"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidMintAuthority",
      "msg": "InvalidMintAuthority"
    },
    {
      "code": 6001,
      "name": "MintCannotRepresentNFT",
      "msg": "MintCannotRepresentNFT"
    },
    {
      "code": 6002,
      "name": "InvalidMint",
      "msg": "InvalidMint"
    },
    {
      "code": 6003,
      "name": "InvalidTokenAccount",
      "msg": "InvalidTokenAccount"
    }
  ]
};
