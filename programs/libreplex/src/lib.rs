use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("AJ5Hh5q4HegZWWu1ScY7ZRA6zELXmRzEWS5EXFSKqBC6");

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

    pub fn update_collection(
        ctx: Context<EditCollection>,
        collection_input: CollectionInput,
    ) -> Result<()> {
        msg!("editing collection data");
        instructions::edit_collection::handler(
            ctx,
            collection_input
        )
    }

    pub fn delete_collection(
        ctx: Context<DeleteCollection>,
    ) -> Result<()> {
        msg!("deleting collection data");
        instructions::delete_collection::handler(ctx)
    }

    pub fn edit_permissions(ctx: Context<EditPermissions>, 
        input: EditPermissionsInput) -> Result<()> {
        instructions::edit_permissions::handler(ctx, input)
    }

    
    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        metadata_input: MetadataInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_metadata::handler(
            ctx,
            metadata_input
        )
    }

    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>,
    ) -> Result<()> {
        msg!("deleting metadata");
        instructions::delete_metadata::handler(ctx)
    }

    pub fn delete_permissions(
        ctx: Context<DeletePermissions>,
    ) -> Result<()> {
        msg!("deleting collection permissions");
        instructions::delete_collection_permissions::handler(ctx)
    }


}
