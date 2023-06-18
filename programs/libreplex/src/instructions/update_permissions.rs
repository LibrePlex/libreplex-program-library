use anchor_lang::prelude::*;
use crate::{PermissionEvent, PermissionEventType, DelegatePermissions, PermissionType};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct EditPermissionsInput {
    pub permissions: Vec<PermissionType>,
}

#[derive(Accounts)]
pub struct UpdatePermissionsDelegate<'info> {
    pub update_authority: Signer<'info>,

    /// CHECK: Can be any account
    pub user: AccountInfo<'info>,

    #[account(mut, has_one = update_authority)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,
}

pub fn handler(ctx: Context<UpdatePermissionsDelegate>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    
    let EditPermissionsInput {
         permissions,
    } = edit_permissions_input;

    
    user_permissions.permissions = permissions;

    emit!(PermissionEvent {
        reference: user_permissions.reference,
        user: ctx.accounts.user.key(),
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}