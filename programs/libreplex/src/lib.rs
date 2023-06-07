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

    pub fn create_group(
        ctx: Context<CreateGroup>,
        collection_input: GroupInput,
    ) -> Result<()> {
        msg!("creating collection data");
        instructions::create_group::handler(
            ctx,
            collection_input
        )
    }

    pub fn update_group(
        ctx: Context<UpdateGroup>,
        collection_input: GroupInput,
    ) -> Result<()> {
        msg!("Update group");
        instructions::update_group::handler(
            ctx,
            collection_input
        )
    }

    pub fn delete_group(
        ctx: Context<DeleteGroup>,
    ) -> Result<()> {
        msg!("delete group data");
        instructions::delete_group::handler(ctx)
    }

    pub fn edit_permissions(ctx: Context<UpdatePermissions>, 
        input: EditPermissionsInput) -> Result<()> {
        instructions::update_permissions::handler(ctx, input)
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
        instructions::delete_permissions::handler(ctx)
    }


}
