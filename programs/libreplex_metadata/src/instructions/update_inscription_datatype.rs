use anchor_lang::prelude::*;

use crate::instructions::EditMetadataEvent;
use crate::{Metadata, DelegatePermissions, PermissionType, Collection, Asset};


use crate::errors::ErrorCode;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateInscriptionDataTypeInput {
    data_type: String,
}





// Who can edit the Metadata?
// The update authority!
// The editor of delegated_metadata_specific_permissions

// If part of a group
// The group authority
// The editor with delegated group wide permissions

#[derive(Accounts)]
#[instruction(metadata_input: UpdateInscriptionDataTypeInput)]
pub struct UpdateInscriptionDataType<'info> {
    #[account(mut)]
    pub editor: Signer<'info>,

    #[account(mut,
        realloc = metadata.get_size() + metadata_input.data_type.len() - match &metadata.asset {
            Asset::Inscription {
                data_type, ..
            } => data_type.len(),
            _ => 0
        },
        realloc::payer = editor,
        realloc::zero = false)]
    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 

                        editor.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 

                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(seeds = ["permissions".as_ref(), editor.key().as_ref(), 
                        collection.as_ref().expect("Group must be provided with group wide permissions").key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(constraint = metadata.collection.expect("Metadata must have a collection if you provided a collection.") == collection.key())]
    pub collection: Option<Box<Account<'info, Collection>>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateInscriptionDataType>,
    update_metadata_input: UpdateInscriptionDataTypeInput,
) -> Result<()> {
    let editor = &ctx.accounts.editor;

    
    let metadata = &mut ctx.accounts.metadata;

    let mut can_edit = editor.key == &metadata.update_authority;


    if let Some(group) = ctx.accounts.collection.as_ref() {
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

    // match &metadata.asset {
    //     Asset::Inscription {
    //         data_type:_,
    //         account_id,
    //         description,
    //     } => {
    //         msg!("Updating asset datatype: {}", update_metadata_input.data_type);
    //         metadata.asset= Asset::Inscription { account_id: *account_id, data_type: "asdasd".to_owned(), //update_metadata_input.data_type.clone(), 
    //             description:  match description {
    //                 Some(x)=>Some(x.clone()),
    //                 None => None
    //             } }
    //     },
    //     _ => {
    //         return Err(ErrorCode::WrongAssetType.into())
    //     }
    // }

    let old_val = metadata.asset.clone();
    metadata.asset = match old_val {
        Asset::Inscription { account_id, data_type:_, description } => Asset::Inscription { account_id, data_type: update_metadata_input.data_type,
             description },
        _ => {
            return Err(ErrorCode::WrongAssetType.into())
        }
    };

    emit!(EditMetadataEvent{
        id: metadata.key(),
        name: metadata.name.clone()
    });

    Ok(())
}
