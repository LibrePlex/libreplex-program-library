use anchor_lang::prelude::*;
use crate::{state::{PERMISSIONS_SIZE, MetadataGroup}, PermissionEvent, PermissionEventType, Permissions, assert_valid_permissions, PermissionType};
use prog_common::{errors::ErrorCode};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct EditCollectionPermissionsInput {
    pub add_permissions: Vec<PermissionType>,
    pub remove_permissions: Vec<PermissionType>
}

#[derive(Accounts)]
pub struct EditCollectionPermissions<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Can be any account
    pub user: AccountInfo<'info>,

    // TODO: Move this check into logic to allow for either collection permission or metadata permission
    #[account(
        seeds = ["permissions".as_ref(), collection.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub auth_permissions: Box<Account<'info, Permissions>>,

    #[account(init_if_needed, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = ["permissions".as_ref(), collection.key().as_ref(), user.key().as_ref()], 
        bump)]
    pub user_permissions: Box<Account<'info, Permissions>>,

    #[account(mut)]
    pub collection: Box<Account<'info, MetadataGroup>>,

    pub system_program: Program<'info, System>,
}

pub fn has_permission(permissions: &Vec<PermissionType>, permission_type: PermissionType) -> Option<usize> {
    return permissions.into_iter().position(|x|(x.eq(&permission_type)));
    
}


pub fn handler(ctx: Context<EditCollectionPermissions>, edit_permissions_input: EditCollectionPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    let auth_permissions = & ctx.accounts.auth_permissions;
    let collection = &ctx.accounts.collection;
    let auth = &ctx.accounts.authority;
    
    assert_valid_permissions(auth_permissions, collection.key(), auth.key(), &crate::PermissionType::Admin)?;

    let EditCollectionPermissionsInput {
         add_permissions,
         remove_permissions,

    } = edit_permissions_input;

    for add_permission in add_permissions {
        let idx = has_permission(&user_permissions.permissions, add_permission);

        match idx {

            Some(pos) =>{
                // already found, no need to do anything
                        
            }, None => {
                user_permissions.permissions.push(PermissionType::Admin)
            }
        }
    }

    for remove_permission in remove_permissions {
        let idx = has_permission(&user_permissions.permissions, remove_permission);

        match idx {

            Some(pos) =>{
                    user_permissions.permissions.remove(pos);
            }, None => {
                 
            }
        }
    }

    // user_permissions.can_create_metadata = can_create_metadata;
    // user_permissions.can_delete_collection = can_delete_collection;
    // user_permissions.can_delete_metadata = can_delete_metadata;
    // user_permissions.can_edit_metadata = can_edit_metadata;
    // user_permissions.is_admin = is_admin;

    emit!(PermissionEvent {
        group: ctx.accounts.collection.key(),
        user: ctx.accounts.user.key(),
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}