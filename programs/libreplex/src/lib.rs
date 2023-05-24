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

#[program]
pub mod libreplex {

    use super::*;

    /* for creating base metadata (SPL) */
    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        name: String,
        symbol: String,
        image_url: String,
        is_mutable: bool,
        bump: u8,
    ) -> Result<()> {
        handle_create_metadata(ctx, name, symbol, image_url, is_mutable, bump)
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        name: Option<String>,
        symbol: Option<String>,
        image_url: Option<String>,
        is_mutable: Option<bool>,
        bump: u8,
    ) -> Result<()> {
        handle_update_metadata(ctx, name, symbol, image_url, is_mutable, bump)
    }

    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>
    ) -> Result<()> {
        handle_delete_metadata(ctx)
    }

    /* for creating auxiliary metadata with creators and attributes  */
    pub fn create_metadata_nft(
        ctx: Context<CreateMetadataNft>,
        creators: Vec<Creator>,
        attributes: Vec<Attribute>,
        collection: Option<Pubkey>,
        bump: u8
    ) -> Result<()> {
        handle_create_metadata_nft(ctx, creators,
            attributes, 
            collection,
            bump)
    }

    pub fn update_metadata_nft(
        ctx: Context<UpdateMetadataNft>,
        creators: Option<Vec<Creator>>,
        attributes: Option<Vec<Attribute>>,
        //TODO: Add update collection
    ) -> Result<()> {
        handle_update_metadata_nft(ctx, creators, attributes)
    }
}
