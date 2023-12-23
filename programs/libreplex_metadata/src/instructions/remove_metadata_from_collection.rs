use anchor_lang::{prelude::*, system_program};

use crate::{Metadata, DelegatePermissions, PermissionType, Collection};

use crate::errors::ErrorCode;


// removes metadata from collection
#[derive(Accounts)]
pub struct RemoveFromCollectionCtx<'info> {
    #[account(mut)]
    pub collection_authority: Signer<'info>,

    #[account(mut,
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(seeds = ["permissions".as_ref(),
        collection_authority.key().as_ref(), 
        collection.key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(mut)]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RemoveFromCollectionCtx>
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    if metadata.collection.eq(&system_program::ID) {
        return Err(ErrorCode::MetadataDoesNotBelongToACollection.into())
    }

    let collection = &mut ctx.accounts.collection;

    
    let collection_authority = &ctx.accounts.collection_authority;

    let mut can_edit_group = &collection.update_authority == collection_authority.key;

    if let Some(delegated_group_wide_permissions) = &ctx.accounts.delegated_group_wide_permissions {
        can_edit_group = can_edit_group || delegated_group_wide_permissions.permissions.contains(&PermissionType::AddToGroup)
    }

    if !can_edit_group  {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    collection.item_count -= 1;

    metadata.collection = system_program::ID;
    // reassign authority to the authority instead of the group itself
    metadata.update_authority = collection_authority.key();
    
    Ok(())
}
