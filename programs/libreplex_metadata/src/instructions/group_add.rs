use anchor_lang::prelude::*;

use crate::{Metadata, DelegatePermissions, PermissionType, Group};

use prog_common::{errors::ErrorCode};


// Adds a metadata to a group
#[derive(Accounts)]
pub struct GroupAdd<'info> {
    pub metadata_authority: Signer<'info>,

    pub group_authority: Signer<'info>,

    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 
                        metadata_authority.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(seeds = ["permissions".as_ref(),
                        group_authority.key().as_ref(), 
                        group.key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(mut)]
    pub group: Box<Account<'info, Group>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<GroupAdd>
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    if metadata.group.is_some() {
        return Err(ErrorCode::MetadataAlreadyHasAGroup.into())
    }

    let group = &ctx.accounts.group;

    let metadata_authority = &ctx.accounts.metadata_authority;
    let group_authority = &ctx.accounts.group_authority;

    let mut can_edit_metadata = &metadata.update_authority == metadata_authority.key;
    
    if let Some(delegated_metadata_specific_permissions_account) 
        = &ctx.accounts.delegated_metadata_specific_permissions {
            can_edit_metadata = can_edit_metadata || delegated_metadata_specific_permissions_account.permissions.contains(&PermissionType::AddToGroup)
    }

    let mut can_edit_group = &group.update_authority == group_authority.key;

    if let Some(delegated_group_wide_permissions) = &ctx.accounts.delegated_group_wide_permissions {
        can_edit_group = can_edit_group || delegated_group_wide_permissions.permissions.contains(&PermissionType::AddToGroup)
    }

    if !can_edit_group || !can_edit_metadata {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    metadata.group = Some(group.key());
    metadata.update_authority = group.key();
    
    Ok(())
}
