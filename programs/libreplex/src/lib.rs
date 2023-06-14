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
        group_input: GroupInput,
    ) -> Result<()> {
        msg!("creating collection data");
        instructions::create_group::handler(
            ctx,
            group_input
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
        metadata_input: CreateMetadataInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_metadata::handler(
            ctx,
            metadata_input
        )
    }

    pub fn create_ordinal(
        ctx: Context<CreateOrdinal>,
        ordinal_input: CreateOrdinalInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_ordinal::handler(
            ctx,
            ordinal_input
        )
    }

    pub fn append_to_ordinal(
        ctx: Context<AppendToOrdinal>,
        input: AppendToOrdinalInput,
    ) -> Result<()> {
        instructions::append_to_ordinal::handler(
            ctx,
            input
        )
    }


    pub fn extend_metadata(
        ctx: Context<ExtendMetadata>,
        metadata_input: ExtendMetadataInput,
    ) -> Result<()> {
        msg!("extending metadata");
        instructions::extend_metadata::handler(
            ctx,
            metadata_input
        )
    }

    pub fn delete_metadata_extension(
        ctx: Context<DeleteMetadataExtension>,
    ) -> Result<()> {
        msg!("deleting metadata");
        instructions::delete_metadataextension::handler(ctx)
    }

    pub fn delete_permissions(
        ctx: Context<DeletePermissions>,
    ) -> Result<()> {
        msg!("deleting collection permissions");
        instructions::delete_permissions::handler(ctx)
    }


}
