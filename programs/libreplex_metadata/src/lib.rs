use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("LibrQsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p");

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
        group_input: GroupInput,
    ) -> Result<()> {
        msg!("Update group");
        instructions::update_group::handler(
            ctx,
            group_input
        )
    }

    pub fn update_group_authority(ctx: Context<UpdateGroupAuthority>, new_update_authority: Pubkey) -> Result<()>  {
        instructions::update_group_authority::handler(
            ctx,
            new_update_authority
        )
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, input: UpdateMetadataInput) -> Result<()> {
        msg!("Update metadata");

        instructions::update_metadata::handler(ctx, input)
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
        msg!("delete metadata");
        instructions::delete_metadata::handler(
            ctx
        )
    }

    pub fn delete_group(
        ctx: Context<DeleteGroup>
    ) -> Result<()> {
        msg!("delete group");
        instructions::delete_group::handler(
            ctx
        )
    }

 
    pub fn create_inscription_metadata(
        ctx: Context<CreateInscriptionMetadata>,
        metadata_input: CreateMetadataInscriptionInput
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_metadata_inscription::handler(
            ctx,
            metadata_input,
        )
    }

    pub fn update_inscription_datatype(
        ctx: Context<UpdateInscriptionDataType>,
        inscription_input: UpdateInscriptionDataTypeInput
    ) -> Result<()> {
        msg!("update inscription datatype");
        instructions::update_inscription_datatype::handler(
            ctx,
            inscription_input,
        )
    }

    pub fn delete_metadata_inscription(
        ctx: Context<DeleteMetadataInscription>
    ) -> Result<()> {
        instructions::delete_metadata_inscription::handler(
            ctx
        )
    }

    pub fn delete_permissions(
        ctx: Context<DeletePermissions>,
    ) -> Result<()> {
        msg!("deleting collection permissions");
        instructions::delete_permissions::handler(ctx)
    }

}
