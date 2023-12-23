use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

// replace with vanity address when available
declare_id!("LimezkueDBjU8mJGt7ctwXgE3h1vmJ89ebyCriYBLU7");

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

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        collection_input: CreateCollectionInput,
    ) -> Result<()> {
        msg!("creating collection data");
        instructions::create_collection::handler(
            ctx,
            collection_input
        )
    }

    pub fn update_collection(
        ctx: Context<UpdateCollectionCtx>,
        collection_input: CreateCollectionInput,
    ) -> Result<()> {
        instructions::update_collection::handler(
            ctx,
            collection_input
        )
    }

    pub fn update_collection_authority(ctx: Context<UpdateCollectionAuthority>, new_update_authority: Pubkey) -> Result<()>  {
        instructions::update_collection_authority::handler(
            ctx,
            new_update_authority
        )
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, 
        // shares input with create metadata
        input: UpdateMetadataInput) -> Result<()> {
        instructions::update_metadata::handler(ctx, input)
    }

    pub fn add_metadata_to_collection(ctx: Context<AddMetadataToCollection>) -> Result<()> {
        instructions::add_metadata_to_collection::handler(ctx)
    }

    pub fn remove_metadata_from_collection(ctx: Context<RemoveFromCollectionCtx>) -> Result<()> {
        instructions::remove_metadata_from_collection::handler(ctx)
    }

    pub fn update_permissions(ctx: Context<UpdatePermissionsDelegate>, 
        input: EditPermissionsInput) -> Result<()> {
        instructions::update_permissions::handler(ctx, input)
    }

    pub fn delegate_collection_permissions(ctx: Context<DelegateCollectionPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
        instructions::delegate_collection_permissions::handler(ctx, edit_permissions_input)
    }

    pub fn delegate_metadata_permissions(ctx: Context<DelegateMetadataPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
        instructions::delegate_metadata_permissions::handler(ctx, edit_permissions_input)
    }

    pub fn create_metadata(
        ctx: Context<CreateMetadataCtx>,
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

    pub fn delete_collection(
        ctx: Context<DeleteCollection>
    ) -> Result<()> {
        msg!("delete collection");
        instructions::delete_collection::handler(
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
