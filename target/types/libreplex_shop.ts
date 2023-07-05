export type LibreplexShop = {
  "version": "0.2.0",
  "name": "libreplex_shop",
  "instructions": [
    {
      "name": "list",
      "accounts": [
        {
          "name": "lister",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listing",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "escrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
          "name": "listInput",
          "type": {
            "defined": "ListInput"
          }
        }
      ]
    },
    {
      "name": "delist",
      "accounts": [
        {
          "name": "lister",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listing",
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
    },
    {
      "name": "execute",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "listing",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "escrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listerPaymentTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerPaymentTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "listing",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "lister",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ListInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Price",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Native",
            "fields": [
              {
                "name": "lamports",
                "type": "u64"
              }
            ]
          },
          {
            "name": "Spl",
            "fields": [
              {
                "name": "mint",
                "type": "publicKey"
              },
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadOwner",
      "msg": "Bad owner"
    },
    {
      "code": 6001,
      "name": "BadMint",
      "msg": "Bad mint"
    }
  ]
};

export const IDL: LibreplexShop = {
  "version": "0.2.0",
  "name": "libreplex_shop",
  "instructions": [
    {
      "name": "list",
      "accounts": [
        {
          "name": "lister",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listing",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "escrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
          "name": "listInput",
          "type": {
            "defined": "ListInput"
          }
        }
      ]
    },
    {
      "name": "delist",
      "accounts": [
        {
          "name": "lister",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listing",
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
    },
    {
      "name": "execute",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "listing",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "escrowTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listerPaymentTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerPaymentTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
    }
  ],
  "accounts": [
    {
      "name": "listing",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "lister",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ListInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Price",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Native",
            "fields": [
              {
                "name": "lamports",
                "type": "u64"
              }
            ]
          },
          {
            "name": "Spl",
            "fields": [
              {
                "name": "mint",
                "type": "publicKey"
              },
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadOwner",
      "msg": "Bad owner"
    },
    {
      "code": 6001,
      "name": "BadMint",
      "msg": "Bad mint"
    }
  ]
};
