use anchor_lang::prelude::*;

#[error_code]
pub enum LegacyInscriptionErrorCode {
    
    #[msg("Metadata has a bad mint")]
    BadMint,
    
    #[msg("Cannot inscribe a fungible asset")]
    CannotInscribeFungible,
    
    #[msg("Bad authority")]
    BadAuthority,
    
    #[msg("Bad authority for holder inscription")]
    BadAuthorityForHolderInscription,
    
    #[msg("Bad authority for update auth inscription")]
    BadAuthorityForUpdateAuthInscription,

    #[msg("Multi Signature threshold must be one to create / edit inscriptions")]
    MultiSigThresholdMustBeOne,

    #[msg("Not squads member")]
    NotSquadsMember,

    #[msg("Inscription V2 key mismatch")]
    Inscription2KeyMismatch,

    #[msg("Inscription V3 key mismatch")]
    InscriptionV3KeyMismatch,

    #[msg("Metadata data missmatch")]
    DataHashMismatch,

}
