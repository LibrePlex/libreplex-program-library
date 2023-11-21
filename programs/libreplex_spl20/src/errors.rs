use anchor_lang::prelude::*;

#[error_code]
pub enum Spl20Error {
    #[msg("Ticker to long")]
    TickerToLong,

    #[msg("Root type to long")]
    RootTypeToLong,
}
