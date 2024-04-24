use anchor_lang::prelude::*;

#[error_code]
pub enum FairLaunchError {
    #[msg("Ticker too long")]
    TickerTooLong,

    #[msg("Mint template too long")]
    MintTemplateTooLong,

    #[msg("Offchain URL too long")]
    OffchainUrlTooLong,



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
    CreatorFeeTooHigh,

    #[msg("Custom multiplier mints require co signer")]
    MultiplierMissMatch,

    #[msg("Incorrect cosigner for mint")]
    IncorrectMintCosigner,

    #[msg("Incorrect cosigner for swap to spl")]
    IncorrectSwapToSplCosigner,

    #[msg("Incorrect cosigner for swap to NFT")]
    IncorrectSwapToNftCosigner,
}
