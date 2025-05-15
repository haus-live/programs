/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/haus.json`.
 */
export type Haus = {
  "address": "8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5",
  "metadata": {
    "name": "haus",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "claimRealtimeAsset",
      "discriminator": [
        73,
        231,
        9,
        170,
        19,
        44,
        21,
        43
      ],
      "accounts": [
        {
          "name": "event",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "realtimeAsset"
              }
            ]
          }
        },
        {
          "name": "realtimeAsset",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mplCoreProgram",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        }
      ],
      "args": []
    },
    {
      "name": "createEvent",
      "discriminator": [
        49,
        219,
        29,
        203,
        22,
        98,
        100,
        87
      ],
      "accounts": [
        {
          "name": "realtimeAsset",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "event",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "realtimeAsset"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "mplCoreProgram",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createEventArgs"
            }
          }
        }
      ]
    },
    {
      "name": "initTippingCalculator",
      "discriminator": [
        117,
        30,
        89,
        81,
        237,
        6,
        85,
        111
      ],
      "accounts": [
        {
          "name": "realtimeAsset"
        },
        {
          "name": "event",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "realtimeAsset"
              }
            ]
          }
        },
        {
          "name": "tippingCalculator",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  112,
                  112,
                  105,
                  110,
                  103,
                  95,
                  99,
                  97,
                  108,
                  99,
                  117,
                  108,
                  97,
                  116,
                  111,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "realtimeAsset"
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "makeTip",
      "discriminator": [
        163,
        150,
        8,
        178,
        66,
        148,
        50,
        99
      ],
      "accounts": [
        {
          "name": "event",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "ix.realtime_asset_key"
              }
            ]
          }
        },
        {
          "name": "tippingCalculator",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  112,
                  112,
                  105,
                  110,
                  103,
                  95,
                  99,
                  97,
                  108,
                  99,
                  117,
                  108,
                  97,
                  116,
                  111,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "ix.realtime_asset_key"
              },
              {
                "kind": "account",
                "path": "tipping_calculator.authority",
                "account": "tippingCalculator"
              }
            ]
          }
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sessionToken",
          "optional": true
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "makeTipArgs"
            }
          }
        }
      ]
    },
    {
      "name": "withdrawTips",
      "discriminator": [
        107,
        192,
        228,
        68,
        165,
        120,
        164,
        23
      ],
      "accounts": [
        {
          "name": "event",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "event"
          ]
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
      "name": "baseAssetV1",
      "discriminator": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    },
    {
      "name": "event",
      "discriminator": [
        125,
        192,
        125,
        158,
        9,
        115,
        152,
        233
      ]
    },
    {
      "name": "sessionToken",
      "discriminator": [
        233,
        4,
        115,
        14,
        46,
        21,
        1,
        15
      ]
    },
    {
      "name": "tippingCalculator",
      "discriminator": [
        126,
        214,
        224,
        73,
        85,
        136,
        204,
        88
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidOwner",
      "msg": "Signer does not own the token account"
    },
    {
      "code": 6001,
      "name": "invalidMint",
      "msg": "Token account does not match mint"
    },
    {
      "code": 6002,
      "name": "invalidAmount",
      "msg": "Token account must hold exactly 1 token"
    },
    {
      "code": 6003,
      "name": "invalidMetadataAccount",
      "msg": "Invalid metadata account"
    },
    {
      "code": 6004,
      "name": "unverifiedCollection",
      "msg": "NFT collection is not verified"
    },
    {
      "code": 6005,
      "name": "invalidCollection",
      "msg": "NFT does not belong to the expected collection"
    },
    {
      "code": 6006,
      "name": "noCollectionData",
      "msg": "NFT has no collection data"
    }
  ],
  "types": [
    {
      "name": "artCategory",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "standupComedy"
          },
          {
            "name": "performanceArt"
          },
          {
            "name": "poetrySlam"
          },
          {
            "name": "openMicImprov"
          },
          {
            "name": "livePainting"
          },
          {
            "name": "creatingWorkshop"
          }
        ]
      }
    },
    {
      "name": "baseAssetV1",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": {
              "defined": {
                "name": "key"
              }
            }
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "updateAuthority",
            "type": {
              "defined": {
                "name": "updateAuthority"
              }
            }
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "seq",
            "type": {
              "option": "u64"
            }
          }
        ]
      }
    },
    {
      "name": "createEventArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "beginTimestamp",
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "type": "i64"
          },
          {
            "name": "reservePrice",
            "type": "u128"
          },
          {
            "name": "ticketCollection",
            "type": "pubkey"
          },
          {
            "name": "artCategory",
            "type": {
              "defined": {
                "name": "artCategory"
              }
            }
          }
        ]
      }
    },
    {
      "name": "event",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "realtimeAsset",
            "docs": [
              "The Real Time Asset (Metaplex Core) representing the event"
            ],
            "type": "pubkey"
          },
          {
            "name": "beginTimestamp",
            "docs": [
              "Start time of the event"
            ],
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "docs": [
              "End time of the event"
            ],
            "type": "i64"
          },
          {
            "name": "tippingLeader",
            "docs": [
              "The user with the highest total tipped amount"
            ],
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "tippingLeaderTotal",
            "docs": [
              "The higher total tipped amount"
            ],
            "type": "u128"
          },
          {
            "name": "reservePrice",
            "docs": [
              "Minimum total tipped amount needed to claim the assets' ownership"
            ],
            "type": "u128"
          },
          {
            "name": "ticketCollection",
            "docs": [
              "Ticket collection (Metaplex Token Metadata)"
            ],
            "type": "pubkey"
          },
          {
            "name": "artCategory",
            "docs": [
              "Event type (category)"
            ],
            "type": {
              "defined": {
                "name": "artCategory"
              }
            }
          }
        ]
      }
    },
    {
      "name": "key",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "uninitialized"
          },
          {
            "name": "assetV1"
          },
          {
            "name": "hashedAssetV1"
          },
          {
            "name": "pluginHeaderV1"
          },
          {
            "name": "pluginRegistryV1"
          },
          {
            "name": "collectionV1"
          }
        ]
      }
    },
    {
      "name": "makeTipArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "realtimeAssetKey",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "sessionToken",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "targetProgram",
            "type": "pubkey"
          },
          {
            "name": "sessionSigner",
            "type": "pubkey"
          },
          {
            "name": "validUntil",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tippingCalculator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalTippedAmount",
            "docs": [
              "Total tips made by the user"
            ],
            "type": "u128"
          },
          {
            "name": "authority",
            "docs": [
              "Authority"
            ],
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "updateAuthority",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "address",
            "fields": [
              "pubkey"
            ]
          },
          {
            "name": "collection",
            "fields": [
              "pubkey"
            ]
          }
        ]
      }
    }
  ]
};
