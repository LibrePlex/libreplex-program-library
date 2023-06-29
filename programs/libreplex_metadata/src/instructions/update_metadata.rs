use anchor_lang::prelude::*;

use crate::{Metadata, DelegatePermissions, PermissionType, UpdateMetadataInput, Group};


use crate::{errors::ErrorCode};

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
                        group.as_ref().expect("Group must be provided with group wide permissions").key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(constraint = metadata.group.expect("Metadata must have a group if you provided a group.") == group.key())]
    pub group: Option<Box<Account<'info, Group>>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMetadata>,
    update_metadata_input: UpdateMetadataInput,
) -> Result<()> {
    let editor = &ctx.accounts.editor;


    let UpdateMetadataInput {name, 
            symbol,
            asset,
            description,
        } = update_metadata_input;

    
    let metadata = &mut ctx.accounts.metadata;

    let mut can_edit = editor.key == &metadata.update_authority;


    if let Some(group) = ctx.accounts.group.as_ref() {
        can_edit = can_edit || &group.update_authority == editor.key;

        if let Some(delegated_group_wide_permissions_account) 
            = ctx.accounts.delegated_group_wide_permissions.as_ref() {
                let delegated_group_wide_permissions = &delegated_group_wide_permissions_account.permissions;

            can_edit = can_edit || delegated_group_wide_permissions.contains(&PermissionType::Update); 
        }
    }

    if let Some(delegated_metadata_specific_permissions_account) = ctx.accounts.delegated_metadata_specific_permissions.as_ref() {
        let delegated_metadata_specific_permissions = &delegated_metadata_specific_permissions_account.permissions;

        can_edit = can_edit || delegated_metadata_specific_permissions.contains(&PermissionType::Update);
    }

    if !can_edit {
        return Err(ErrorCode::InvalidPermissions.into())
    }

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
