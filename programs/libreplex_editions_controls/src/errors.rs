use anchor_lang::prelude::*;

#[error_code]
pub enum EditionsError {
    #[msg("Ticker too long")]
    TickerTooLong,

    #[msg("Mint template too long")]
    MintTemplateTooLong,


    #[msg("Deployment template too long")]
    DeploymentTemplateTooLong,

    #[msg("Root type too long")]
    RootTypeTooLong,

    #[msg("Minted out")]
    MintedOut,

    #[msg("Legacy migrations are minted out")]
    LegacyMigrationsAreMintedOut,

    #[msg("Global tree delegate is missing")]
    MissingGlobalTreeDelegate,

    #[msg("Incorrect mint type")]
    IncorrectMintType,

    #[msg("Invalid Metadata")]
    InvalidMetadata,

    #[msg("Creator fee too high")]
    CreatorFeeTooHigh
}
