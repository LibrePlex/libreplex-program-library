

use anchor_lang::{prelude::*, system_program};

use crate::{Metadata, DelegatePermissions, PermissionType};

use crate::errors::ErrorCode;


// Adds a metadata to a group
#[derive(Accounts)]
pub struct DeleteMetadata<'info> {
    pub metadata_authority: Signer<'info>,

    #[account(
        mut, 
        close = metadata_authority,
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 
                        metadata_authority.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct DeleteEvent {
    pub id: Pubkey
}

pub fn handler(ctx: Context<DeleteMetadata>
) -> Result<()> {

    

    let metadata = &mut ctx.accounts.metadata;

    
    if !metadata.collection.key().eq(&system_program::ID) {
        return Err(ErrorCode::MetadataBelongsToCollection.into())
    }

    if metadata.update_authority.eq(&system_program::ID) {
        return Err(ErrorCode::MetadataIsNotMutable.into())
    }

    let metadata_authority = &ctx.accounts.metadata_authority;

    let mut can_delete_metadata = &metadata.update_authority == metadata_authority.key;
    
    if let Some(delegated_metadata_specific_permissions_account) 
        = &ctx.accounts.delegated_metadata_specific_permissions {
            can_delete_metadata = can_delete_metadata || delegated_metadata_specific_permissions_account.permissions.contains(&PermissionType::Delete)
    }

    if !can_delete_metadata {
        return Err(ErrorCode::MissingPermissionDeleteMetadata.into())
    }

    emit!(DeleteEvent {
        id: metadata.mint.key(),
    });

    Ok(())
}
