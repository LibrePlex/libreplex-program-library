use anchor_lang::prelude::*;

#[error_code]
pub enum FairLaunchError {
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
    LegacyMigrationsAreMintedOut


}
