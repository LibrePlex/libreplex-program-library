use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("AJ5Hh5q4HegZWWu1ScY7ZRA6zELXmRzEWS5EXFSKqBC6");

pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;
pub use errors::*;

pub mod shared;
pub use shared::*;

pub use constants::*;
pub use state::*;

#[program]
pub mod libreplex_metadata {
    use super::*;

    pub fn create_group(
        ctx: Context<CreateGroup>,
        group_input: GroupInput,
    ) -> Result<()> {
        msg!("creating group data");
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

    pub fn group_add(ctx: Context<GroupAdd>) -> Result<()> {
        instructions::group_add::handler(ctx)
    }

    pub fn group_remove(ctx: Context<GroupRemove>) -> Result<()> {
        instructions::group_remove::handler(ctx)
    }

    pub fn update_permissions(ctx: Context<UpdatePermissionsDelegate>, 
        input: EditPermissionsInput) -> Result<()> {
        instructions::update_permissions::handler(ctx, input)
    }

    pub fn delegate_group_permissions(ctx: Context<DelegateGroupPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
        instructions::delegate_group_permissions::handler(ctx, edit_permissions_input)
    }

    pub fn delegate_metadata_permissions(ctx: Context<DelegateMetadataPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
        instructions::delegate_metadata_permissions::handler(ctx, edit_permissions_input)
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


    
    pub fn delete_metadata(
        ctx: Context<DeleteMetadata>
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::delete_metadata::handler(
            ctx
        )
    }

    pub fn create_ordinal_metadata(
        ctx: Context<CreateOrdinalMetadata>,
        metadata_input: CreateOrdinalMetadataInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_ordinal_metadata::handler(
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
