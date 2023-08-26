use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Missing argument")]
    MissingArgument, 

    #[msg("Invalid proof")]
    InvalidProof,


    #[msg("Missing Account")]
    MissingAccount,

    #[msg("Invalid Mint funds recepient")]
    InvalidMintFundsRecepient,

    #[msg("Invalid token recepient")]
    InvalidTokenRecepient,

    #[msg("Invalid total mints account")]
    InvalidTotalMintsAccount,

    #[msg("Mint Limit Exceeded")]
    MintLimitExceeded,

    #[msg("No Active Phases")]
    NoActivePhases,

    #[msg("Phase not specified")]
    PhaseNotSpecified,

    #[msg("Invalid token program")]
    InvalidTokenProgram,

    #[msg("Invalid remaining accounts for custom program control.")]
    InvalidRemainingAccountsForCustomProgramControl,

    #[msg("InvalidCustomProgram")]
    InvalidCustomProgram
}