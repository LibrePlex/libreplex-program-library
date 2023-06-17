use anchor_lang::prelude::*;
use crate::{state::{PERMISSIONS_SIZE}, PermissionEvent, PermissionEventType, DelegatePermissions, assert_valid_permissions, PermissionType, Metadata, Group};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct EditPermissionsInput {
    pub add_permissions: Vec<PermissionType>,
    pub remove_permissions: Vec<PermissionType>
}


#[derive(Accounts)]
pub struct DelegateMetadataPermissions<'info> {
    pub update_authority: Signer<'info>,

    #[account(seeds = ["permissions".as_ref(), 
                        delegated_user.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        
                        bump)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,

    #[account(has_one = update_authority)]
    pub metadata: Account<'info, Metadata>,

    pub delegated_user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DelegateGroupPermissions<'info> {
    pub update_authority: Signer<'info>,

    #[account(seeds = ["permissions".as_ref(), 
                        delegated_user.key().as_ref(), 
                        group.key().as_ref()], 
                        bump)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,

    #[account(has_one = update_authority)]
    pub group: Account<'info, Group>,

    pub delegated_user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePermissionsDelegate<'info> {
    pub update_authority: Signer<'info>,

    /// CHECK: Can be any account
    pub user: AccountInfo<'info>,

    #[account(mut, has_one = update_authority)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,
}

pub fn has_permission(permissions: &Vec<PermissionType>, permission_type: PermissionType) -> Option<usize> {
    return permissions.into_iter().position(|x|(x.eq(&permission_type)));
    
}


pub fn handler(ctx: Context<UpdatePermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    let auth_permissions = & ctx.accounts.auth_permissions;
    let reference = &ctx.accounts.reference;
    let auth = &ctx.accounts.authority;
    
    assert_valid_permissions(auth_permissions, reference.key(), auth.key(), &crate::PermissionType::Admin)?;

    let EditPermissionsInput {
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
    emit!(PermissionEvent {
        reference: ctx.accounts.reference.key(),
        user: ctx.accounts.user.key(),
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}