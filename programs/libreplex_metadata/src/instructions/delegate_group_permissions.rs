use anchor_lang::prelude::*;
use crate::{PermissionEvent, PermissionEventType, DelegatePermissions,  Group};

use super::update_permissions::EditPermissionsInput;


#[derive(Accounts)]
#[instruction(input: EditPermissionsInput)]
pub struct DelegateGroupPermissions<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(init, payer = update_authority, space = DelegatePermissions::BASE_SIZE + input.permissions.len(), 
                        seeds = ["permissions".as_ref(), 
                        delegated_user.key().as_ref(), 
                        group.key().as_ref()], 
                        bump)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,

    #[account(has_one = update_authority)]
    pub group: Account<'info, Group>,

    /// CHECK: No checks requires as the authority can assign delegate to anything they want
    pub delegated_user: AccountInfo<'info>,
        
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DelegateGroupPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    
    let EditPermissionsInput {
         permissions,
    } = edit_permissions_input;

    
    user_permissions.permissions = permissions;
    user_permissions.bump = *ctx.bumps.get("user_permissions").unwrap();
    user_permissions.reference = ctx.accounts.group.key();
    user_permissions.user = ctx.accounts.delegated_user.key();
    user_permissions.update_authority = ctx.accounts.update_authority.key();

    emit!(PermissionEvent {
        reference: user_permissions.reference,
        user: user_permissions.user,
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}