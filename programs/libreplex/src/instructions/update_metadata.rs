use anchor_lang::prelude::*;
use crate::instructions::has_permission;

use crate::{Metadata, DelegatePermissions, PermissionType, UpdateMetadataInput, Group};


use prog_common::{errors::ErrorCode};

#[event]
struct EditMetadataEvent {
    id: Pubkey,
    name: String,
}





// Who can edit the Metadata?
// The update authority!
// The editor of delegated_metadata_specific_permissions

// If part of a group
// The group authority
// The editor with delegated group wide permissions

#[derive(Accounts)]
#[instruction(metadata_input: UpdateMetadataInput)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub editor: Signer<'info>,

    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 

                        editor.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        
                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(seeds = ["permissions".as_ref(), editor.key().as_ref(), 
                        group.expect("Group must be provided with group wide permissions").key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(constraint = metadata.group.expect("Metadata must have a group if you provided a group.") == group.key())]
    pub group: Option<Box<Account<'info, Group>>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMetadata>,
    update_metadata_input: UpdateMetadataInput,
) -> Result<()> {
    let editor = &ctx.accounts.editor;
    let metadata = &ctx.accounts.metadata;
    let permissions = &ctx.accounts.permissions;
    

    let UpdateMetadataInput {name, 
            symbol,
            asset,
            description,
            invoked_permission
        } = update_metadata_input;

    if invoked_permission != PermissionType::Admin {
        return Err(ErrorCode::InvalidPermissions.into())
    }
    
    let metadata_key = metadata.key();
    let editor_key = editor.key();



    let metadata_permissions_path = &[b"permissions", metadata_key.as_ref(), editor_key.as_ref()];
    let (metadata_permissions_key, metadata_bump) = Pubkey::find_program_address(metadata_permissions_path, &crate::id());

    if metadata_permissions_key == permissions.key() {
        if permissions.bump != metadata_bump  {
            return Err(ErrorCode::InvalidBump.into())
        }

        if has_permission(&permissions.permissions, invoked_permission).is_none() {
            // TODO: Add support for edit as well as admin
            return Err(ErrorCode::InvalidPermissions.into())
        }
        
    }
    
    let metadata = &mut ctx.accounts.metadata;

    

    // Update the metadata state account
    metadata.name = name.clone();
    metadata.asset = asset.clone();
    metadata.description = description;
    metadata.symbol= symbol;
    
    emit!(EditMetadataEvent{
        id: metadata.key(),
        name
    });

    Ok(())
}
