use anchor_lang::prelude::*;
use crate::{PermissionEvent, PermissionEventType, DelegatePermissions,  Collection};

use super::update_permissions::EditPermissionsInput;


#[derive(Accounts)]
#[instruction(input: EditPermissionsInput)]
pub struct DelegateCollectionPermissions<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(init, payer = update_authority, space = DelegatePermissions::BASE_SIZE + input.permissions.len(), 
                        seeds = ["permissions".as_ref(), 
                        delegated_user.key().as_ref(), 
                        collection.key().as_ref()], 
                        bump)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,

    #[account(has_one = update_authority)]
    pub collection: Account<'info, Collection>,

    /// CHECK: No checks requires as the authority can assign delegate to anything they want
    pub delegated_user: AccountInfo<'info>,
        
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DelegateCollectionPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    
    let EditPermissionsInput {
         permissions,
    } = edit_permissions_input;

    
    user_permissions.permissions = permissions;
    user_permissions.bump = ctx.bumps.user_permissions;
    user_permissions.reference = ctx.accounts.collection.key();
    user_permissions.user = ctx.accounts.delegated_user.key();
    user_permissions.update_authority = ctx.accounts.update_authority.key();

    emit!(PermissionEvent {
        reference: user_permissions.reference,
        user: user_permissions.user,
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}