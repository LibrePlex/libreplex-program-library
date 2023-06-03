use anchor_lang::prelude::*;
use crate::{state::{CollectionPermissions, PERMISSIONS_SIZE, CollectionData}, assert_valid_user_permissions};
use prog_common::{errors::ErrorCode};


pub struct EditPermissionsInput {
    pub is_admin: bool,
    pub can_create_metadata: bool,
    pub can_edit_metadata: bool,
    pub can_delete_metadata: bool,
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

    if !auth_permissions.is_admin {
        return Err(ErrorCode::MissingPermissionAdmin.into());
    }
 
    let EditPermissionsInput {can_create_metadata, can_delete_collection, can_delete_metadata, can_edit_metadata, is_admin} = edit_permissions_input;

    user_permissions.collection = ctx.accounts.collection_data.key();
    user_permissions.user = ctx.accounts.user.key();
    user_permissions.can_create_metadata = can_create_metadata;
    user_permissions.can_delete_collection = can_delete_collection;
    user_permissions.can_delete_metadata = can_delete_metadata;
    user_permissions.can_edit_metadata = can_edit_metadata;
    user_permissions.is_admin = is_admin;


    emit!(PermissionEvent {
        collection: ctx.accounts.collection.key(),
        user: ctx.accounts.user.key(),
        event_type: PermissionEventType::Update,
    });

    Ok(())                                
}
