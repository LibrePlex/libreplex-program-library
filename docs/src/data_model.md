# Data Model

Libreplex provides Metadata for [Token-2022](https://spl.solana.com/token-2022) mints.

## Metadata structure

| Field Name                                 | Type                                                                   | Extra Info |
|:-------------------------------------------|:--------------------------------------------------------------------------|--------------------------------------------------------------------------|
| Mint                                       | [Pubkey](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html)  |
| Update Authority                           | [Pubkey](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html)  |
| Creator                                    | [Pubkey](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html)  | Cannot be changed
| Is Mutable | Bool |
| Group ( Collection )                       | Optional [Pubkey](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html) |
| Name                                       | String |
| Symbol                                     | String |
| Asset                                      |        |
| Extension                                  |        |