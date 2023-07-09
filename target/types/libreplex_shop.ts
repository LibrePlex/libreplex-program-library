export type LibreplexShop = {
  "version": "0.3.0",
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
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listingGroup",
          "isMut": true,
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
          "name": "listingGroup",
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
        },
        {
          "name": "tokenProgram2022",
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listerPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
        },
        {
          "name": "tokenProgram2022",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createListingGroup",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingGroup",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing_group"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "admin"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateListingGroupInput"
                },
                "path": "create_listing_group_input.seed"
              }
            ]
          }
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
            "defined": "CreateListingGroupInput"
          }
        }
      ]
    },
    {
      "name": "deleteListingGroup",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingGroup",
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
      "name": "createListingFilter",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing_filter"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "admin"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateListingFilterInput"
                },
                "path": "create_listing_filter_input.seed"
              }
            ]
          }
        },
        {
          "name": "listingGroup",
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
            "defined": "CreateListingFilterInput"
          }
        }
      ]
    },
    {
      "name": "deleteListingFilter",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listingGroup",
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
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          },
          {
            "name": "group",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          }
        ]
      }
    },
    {
      "name": "listingGroup",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "listingsActive",
            "type": "u32"
          },
          {
            "name": "listingsCreated",
            "type": "u32"
          },
          {
            "name": "listingsSold",
            "type": "u32"
          },
          {
            "name": "filterCount",
            "type": "u32"
          },
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "listingFilter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "listingGroup",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "type": {
              "defined": "ListingFilterType"
            }
          },
          {
            "name": "listingsActive",
            "type": "u32"
          },
          {
            "name": "listingsCreated",
            "type": "u32"
          },
          {
            "name": "listingsSold",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateListingFilterInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "filterType",
            "type": {
              "defined": "ListingFilterType"
            }
          },
          {
            "name": "seed",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CreateListingGroupInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    },
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
    },
    {
      "name": "ListingFilterType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Creator",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Lister",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Group",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeleteListingFilterEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "DeleteListingGroupEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "GroupHasActiveFilters",
      "msg": "Group has active filters"
    },
    {
      "code": 6001,
      "name": "UnsupportFilterType",
      "msg": "Unsupported filter type"
    },
    {
      "code": 6002,
      "name": "ListerNotAllowed",
      "msg": "Lister not allowed"
    },
    {
      "code": 6003,
      "name": "GroupNotAllowed",
      "msg": "Group not allowed"
    },
    {
      "code": 6004,
      "name": "GroupHasActiveListings",
      "msg": "Group has active listings"
    }
  ]
};

export const IDL: LibreplexShop = {
  "version": "0.3.0",
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
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listingGroup",
          "isMut": true,
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
          "name": "listingGroup",
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
        },
        {
          "name": "tokenProgram2022",
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listerPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
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
        },
        {
          "name": "tokenProgram2022",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createListingGroup",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingGroup",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing_group"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "admin"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateListingGroupInput"
                },
                "path": "create_listing_group_input.seed"
              }
            ]
          }
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
            "defined": "CreateListingGroupInput"
          }
        }
      ]
    },
    {
      "name": "deleteListingGroup",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingGroup",
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
      "name": "createListingFilter",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "listing_filter"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "admin"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "CreateListingFilterInput"
                },
                "path": "create_listing_filter_input.seed"
              }
            ]
          }
        },
        {
          "name": "listingGroup",
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
            "defined": "CreateListingFilterInput"
          }
        }
      ]
    },
    {
      "name": "deleteListingFilter",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "listingFilter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "listingGroup",
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
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "listingBump",
            "type": "u8"
          },
          {
            "name": "group",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": {
              "defined": "Price"
            }
          }
        ]
      }
    },
    {
      "name": "listingGroup",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "listingsActive",
            "type": "u32"
          },
          {
            "name": "listingsCreated",
            "type": "u32"
          },
          {
            "name": "listingsSold",
            "type": "u32"
          },
          {
            "name": "filterCount",
            "type": "u32"
          },
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "listingFilter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "listingGroup",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "type": {
              "defined": "ListingFilterType"
            }
          },
          {
            "name": "listingsActive",
            "type": "u32"
          },
          {
            "name": "listingsCreated",
            "type": "u32"
          },
          {
            "name": "listingsSold",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateListingFilterInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "filterType",
            "type": {
              "defined": "ListingFilterType"
            }
          },
          {
            "name": "seed",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CreateListingGroupInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    },
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
    },
    {
      "name": "ListingFilterType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Creator",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Lister",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          },
          {
            "name": "Group",
            "fields": [
              {
                "name": "pubkey",
                "type": "publicKey"
              }
            ]
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "DeleteListingFilterEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "DeleteListingGroupEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "GroupHasActiveFilters",
      "msg": "Group has active filters"
    },
    {
      "code": 6001,
      "name": "UnsupportFilterType",
      "msg": "Unsupported filter type"
    },
    {
      "code": 6002,
      "name": "ListerNotAllowed",
      "msg": "Lister not allowed"
    },
    {
      "code": 6003,
      "name": "GroupNotAllowed",
      "msg": "Group not allowed"
    },
    {
      "code": 6004,
      "name": "GroupHasActiveListings",
      "msg": "Group has active listings"
    }
  ]
};
