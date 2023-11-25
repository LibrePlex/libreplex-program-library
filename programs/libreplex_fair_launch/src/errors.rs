use anchor_lang::prelude::*;

#[error_code]
pub enum Src20Error {
    #[msg("Ticker too long")]
    TickerTooLong,

    #[msg("Root type too long")]
    RootTypeTooLong,

    #[msg("Max number of tokens exceeded")]
    MaxNumberOfTokenExceeded
}
