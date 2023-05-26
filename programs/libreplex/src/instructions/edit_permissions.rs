use anchor_lang::prelude::*;
use crate::{state::{CollectionPermissions, PERMISSIONS_SIZE, CollectionData}, assert_valid_user_permissions};
use prog_common::{errors::ErrorCode};


pub struct EditPermissionsInput {
    pub can_edit_permissions: bool,
    pub can_add_metadatas: bool,
    pub can_edit_metadatas: bool,
    pub can_delete_metadatas: bool,
    pub can_delete_collection: bool,
}

#[derive(Accounts)]
pub struct EditPermissions<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Can be any account
    pub user: AccountInfo<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection_data.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub auth_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(init_if_needed, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = ["permissions".as_ref(), collection_data.key().as_ref(), user.key().as_ref()], 
        bump)]
    pub user_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(mut)]
    pub collection_data: Box<Account<'info, CollectionData>>,

    pub system_program: Program<'info, System>,
}



pub fn handler(ctx: Context<EditPermissions>, edit_permissions_input: EditPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    let auth_permissions = & ctx.accounts.auth_permissions;
    let collection = &ctx.accounts.collection_data;
    let auth = &ctx.accounts.authority;
    
    assert_valid_user_permissions(auth_permissions, &collection.key(), auth.key)?;

    if !auth_permissions.can_edit_permissions {
        return Err(ErrorCode::CannotEditPermissions.into());
    }
 
    let EditPermissionsInput {can_add_metadatas, can_delete_collection, can_delete_metadatas, can_edit_metadatas, can_edit_permissions} = edit_permissions_input;

    user_permissions.collection = ctx.accounts.collection_data.key();
    user_permissions.user = ctx.accounts.user.key();
    user_permissions.can_add_metadatas = can_add_metadatas;
    user_permissions.can_delete_collection = can_delete_collection;
    user_permissions.can_delete_metadatas = can_delete_metadatas;
    user_permissions.can_edit_metadatas = can_edit_metadatas;
    user_permissions.can_edit_permissions = can_edit_permissions;

    Ok(())                                
}
