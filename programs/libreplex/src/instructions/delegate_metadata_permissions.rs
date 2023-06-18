use anchor_lang::prelude::*;
use crate::{PermissionEvent, PermissionEventType, DelegatePermissions, Metadata};

use super::update_permissions::EditPermissionsInput;


#[derive(Accounts)]
#[instruction(input: EditPermissionsInput)]
pub struct DelegateMetadataPermissions<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(init, space = DelegatePermissions::BASE_SIZE + input.permissions.len(), seeds = ["permissions".as_ref(), 
                        delegated_user.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        bump, payer = update_authority)]
    pub user_permissions: Box<Account<'info, DelegatePermissions>>,

    #[account(has_one = update_authority)]
    pub metadata: Account<'info, Metadata>,

    /// CHECK: No checks requires as the authority can assign delegate to anything they want
    pub delegated_user: AccountInfo<'info>,

    
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<DelegateMetadataPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    
    let EditPermissionsInput {
         permissions,
    } = edit_permissions_input;

    
    user_permissions.permissions = permissions;
    user_permissions.bump = *ctx.bumps.get("user_permissions").unwrap();
    user_permissions.reference = ctx.accounts.metadata.key();
    user_permissions.user = ctx.accounts.delegated_user.key();
    user_permissions.update_authority = ctx.accounts.update_authority.key();


    emit!(PermissionEvent {
        reference: user_permissions.reference,
        user: user_permissions.user,
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}