use anchor_lang::prelude::*;

use crate::{Metadata, DelegatePermissions, PermissionType, Collection};

use crate::{errors::ErrorCode};


// Adds a metadata to a group
#[derive(Accounts)]
pub struct AddMetadataToCollection<'info> {
    pub metadata_authority: Signer<'info>,

    #[account(mut)]
    pub collection_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        realloc = metadata.get_size() + 32, // add group size. TODO: Remove the group size on base metadata
        realloc::payer = payer,
        realloc::zero = false)]
    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 
                        metadata_authority.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(seeds = ["permissions".as_ref(),
                        collection_authority.key().as_ref(), 
                        collection.key().as_ref()], bump)]
    pub delegated_collection_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(mut)]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddMetadataToCollection>
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    if metadata.collection.is_some() {
        return Err(ErrorCode::MetadataBelongsToCollection.into())
    }

    let collection = &ctx.accounts.collection;

    let metadata_authority = &ctx.accounts.metadata_authority;
    let collection_authority = &ctx.accounts.collection_authority;

    let mut can_edit_metadata = &metadata.update_authority == metadata_authority.key;
    
    if let Some(delegated_metadata_specific_permissions_account) 
        = &ctx.accounts.delegated_metadata_specific_permissions {
            can_edit_metadata = can_edit_metadata || delegated_metadata_specific_permissions_account.permissions.contains(&PermissionType::AddToGroup)
    }

    let mut can_edit_group = &collection.update_authority == collection_authority.key;

    if let Some(delegated_group_wide_permissions) = &ctx.accounts.delegated_collection_wide_permissions {
        can_edit_group = can_edit_group || delegated_group_wide_permissions.permissions.contains(&PermissionType::AddToGroup)
    }

    if !can_edit_group || !can_edit_metadata {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    msg!("Setting group to {}", collection.key());
    

    metadata.collection = Some(collection.key());
    metadata.update_authority = collection.key();
    
    Ok(())
}
