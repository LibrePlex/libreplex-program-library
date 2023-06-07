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

    pub fn update_collection(
        ctx: Context<EditCollection>,
        collection_input: GroupInput,
    ) -> Result<()> {
        msg!("editing collection data");
        instructions::edit_collection::handler(
            ctx,
            collection_input
        )
    }

    pub fn delete_group(
        ctx: Context<DeleteCollection>,
        permission_type: PermissionType
    ) -> Result<()> {
        msg!("deleting collection data");
        instructions::delete_group::handler(ctx, &permission_type)
    }

    pub fn edit_collection_permissions(ctx: Context<EditCollectionPermissions>, 
        input: EditCollectionPermissionsInput) -> Result<()> {
        msg!("Editing collection permissions");
        instructions::edit_permissions::handler(ctx, input)
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

    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>,
    ) -> Result<()> {
        msg!("deleting metadata");
        instructions::delete_metadata::handler(ctx)
    }



}
