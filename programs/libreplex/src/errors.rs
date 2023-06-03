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
    CannotAddVerifiedCreator,

    #[msg("Cannot verify a collection when one is not set")]
    NoCollectionSet,

    #[msg("Incorrect collection authority")]
    IncorrectCollectionAuthority,

    #[msg("Collection has verified items. Cannot delete.")]
    CannotDeleteCollectionWithVerifiedItems,

    #[msg("Signer not in creator array.")]
    SignerNotInCreatorArray,

    #[msg("This signer has already signed this metadata.")]
    AlreadySigned,

    #[msg("Before deleting an NFT metadata, you must delete the override account first.")]
    MustDeleteOverrideFirst,

    #[msg("Metadata is not mutable.")]
    MetadataIsNotMutable,

    #[msg("Cannot sign non-NFT metadata.")]
    CannotSignNonNftMetadata,

    #[msg("Cannot sign item in non NFT collection.")]
    CannotSignItemInNonNftCollection,

    #[msg("Not NFT collection.")]
    NotNftCollection,

    #[msg("Not SPL collection.")]
    NotSplCollection
}