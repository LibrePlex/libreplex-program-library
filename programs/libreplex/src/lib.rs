use anchor_lang::prelude::*;

pub mod processors;
pub use processors::*;

pub mod errors;
pub use errors::MetadataError;

pub mod constants;
pub use constants::*;

pub mod shared;
pub use shared::*;

pub mod state;
pub use state::*;

declare_id!("L1BRc7ZYjj7t9k7E5xbdnKy3KhaY6sTcJx4gAsqxUbh");

pub mod empty_account {
    use super::*;
    declare_id!("11111111111111111111111111111111");
}

#[program]
pub mod libreplex {

    use super::*;

    /* for creating base metadata (SPL) */
    pub fn create_metadata_spl(
        ctx: Context<CreateMetadataSpl>,
        name: String,
        image_url: String,
        is_mutable: bool,
    ) -> Result<()> {
        handle_create_metadata(ctx, name, image_url, is_mutable)
    }


    pub fn delete_metadata(ctx: Context<DeleteMetadata>) -> Result<()> {
        handle_delete_metadata(ctx)
    }

    /* for creating auxiliary metadata with creators and attributes  */
    pub fn create_metadata_nft(
        ctx: Context<CreateMetadataNft>,
        name: String,
        offchain_url: String,
        is_mutable: bool,
        attributes: Vec<Attribute>,
    ) -> Result<()> {
        handle_create_metadata_nft(
            ctx,
            name,
            offchain_url,
            is_mutable,
            attributes,
        )
    }


}
