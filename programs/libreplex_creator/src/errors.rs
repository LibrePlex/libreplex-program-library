use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Missing mint numbers")]
    MissingMintNumbers, 

    #[msg("Wrong mint numbers")]
    WrongMintNumbers, 

    #[msg("Attribute config missing")]
    MissingAttributeConfig,

    #[msg("Sold out")]
    SoldOut,
}