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
    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        name: String,
        symbol: String,
        image_url: String,
        is_mutable: bool
    ) -> Result<()> {
        handle_create_metadata(ctx, name, symbol, image_url, is_mutable)
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        name: Option<String>,
        symbol: Option<String>,
        image_url: Option<String>,
        is_mutable: Option<bool>
    ) -> Result<()> {
        handle_update_metadata(ctx, name, symbol, image_url, is_mutable)
    }

    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>
    ) -> Result<()> {
        handle_delete_metadata(ctx)
    }

    /* for creating auxiliary metadata with creators and attributes  */
    pub fn create_metadata_nft(
        ctx: Context<CreateMetadataNft>,
        creators: Option<Vec<Creator>>,
        attributes: Vec<Attribute>,
        collection: Option<Pubkey>
    ) -> Result<()> {
        handle_create_metadata_nft(ctx, 
            creators,
            attributes, 
            collection)
    }

    pub fn update_metadata_nft(
        ctx: Context<UpdateMetadataNft>,
        creators: Option<Vec<Creator>>,
        attributes: Option<Vec<Attribute>>,
        collection: Option<Pubkey>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_update_metadata_nft(ctx, creators, attributes, collection)
    }

    pub fn delete_metadata_nft(
        ctx: Context<DeleteMetadataNft>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_delete_metadata_nft(ctx)
    }


    pub fn verify_collection(
        ctx: Context<VerifyCollection>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_collection_verification(ctx, true)
    }

    pub fn unverify_collection(
        ctx: Context<VerifyCollection>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_collection_verification(ctx, false)
    }

    pub fn verify_creator_nft(
        ctx: Context<VerifyCreatorNft>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_verify_creator(ctx, true)
    }

    pub fn unverify_creator(
        ctx: Context<VerifyCreator>
        //TODO: Add update collection
    ) -> Result<()> {
        handle_unverify_creator(ctx, false)
    }


    
}
