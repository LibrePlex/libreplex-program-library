use anchor_lang::prelude::*;

use crate::{Metadata, DelegatePermissions, PermissionType, Collection};

use crate::errors::ErrorCode;


// Adds a metadata to a group
#[derive(Accounts)]
pub struct GroupRemove<'info> {
    #[account(mut)]
    pub collection_authority: Signer<'info>,

    #[account(mut,
        realloc = metadata.get_size() - match &metadata.collection {
            Some(_) => 32, // reduce the size as we no longer need the group
            None => 0
        },
        realloc::payer = collection_authority,
        realloc::zero = false
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

pub fn handler(ctx: Context<GroupRemove>
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    if metadata.collection.is_none() {
        return Err(ErrorCode::MetadataDoesNotBelongToACollection.into())
    }

    let collection = &ctx.accounts.collection;

    
    let collection_authority = &ctx.accounts.collection_authority;

    let mut can_edit_group = &collection.update_authority == collection_authority.key;

    if let Some(delegated_group_wide_permissions) = &ctx.accounts.delegated_group_wide_permissions {
        can_edit_group = can_edit_group || delegated_group_wide_permissions.permissions.contains(&PermissionType::AddToGroup)
    }

    if !can_edit_group  {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    metadata.collection = None;
    // reassign authority to the authority instead of the group itself
    metadata.update_authority = collection_authority.key();
    
    Ok(())
}
