use anchor_lang::prelude::*;
use instructions::*;

declare_id!("L1BRc7ZYjj7t9k7E5xbdnKy3KhaY6sTcJx4gAsqxUbh");

pub mod instructions;
pub mod state;
pub mod constants;

pub use constants::*;
pub use state::*;

#[program]
pub mod libreplex {

    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        collection_input: CollectionInput,
    ) -> Result<()> {
        msg!("creating collection data");
        instructions::create_collection::handler(
            ctx,
            collection_input
        )
    }

    pub fn delete_collection(
        ctx: Context<DeleteCollection>,
        _bump_collection_data: u8,
    ) -> Result<()> {
        msg!("deleting collection data");
        instructions::delete_collection::handler(ctx)
    }

    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        metadata_input: MetadataInput,
        _bump_collection_data: u8,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_metadata::handler(
            ctx,
            metadata_input
        )
    }

    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>,
        _bump_collection_data: u8,
        _bump_metadata: u8,
    ) -> Result<()> {
        msg!("deleting metadata");
        instructions::delete_metadata::handler(ctx)
    }


}
