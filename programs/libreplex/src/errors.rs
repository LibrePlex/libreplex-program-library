use anchor_lang::prelude::*;

#[error_code]
pub enum MetadataError {
    #[msg("Bad bump")]
    InvalidBump,
    
    #[msg("Missing bump")]
    MissingBump,

    #[msg("Cannot remove verified creator")]
    CannotRemoveVerifiedCreator,


    #[msg("Cannot add verified creator")]
    CannotAddVerifiedCreator
}
