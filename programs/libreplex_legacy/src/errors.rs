use anchor_lang::prelude::*;

#[error_code]
pub enum LegacyInscriptionErrorCode {
    #[msg("Metadata has a bad mint")]
    BadMint,
    #[msg("Cannot inscribe a fungible asset")]
    CannotInscribeFungible,
    #[msg("Bad authority")]
    BadAuthority
}
