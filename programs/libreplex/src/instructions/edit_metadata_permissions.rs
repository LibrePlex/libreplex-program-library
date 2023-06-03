use anchor_lang::prelude::*;
use crate::{state::{CollectionPermissions, PERMISSIONS_SIZE, Collection}, assert_valid_collection_permissions, Metadata, MetadataPermissionEvent, MetadataPermissions};
use prog_common::{errors::ErrorCode};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct EditMetadataPermissionsInput {
    can_modify: bool,
}

#[derive(Accounts)]
pub struct EditMetadataPermissions<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Can be any account
    pub user: AccountInfo<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub auth_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(init_if_needed, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = ["permissions".as_ref(), collection.key().as_ref(), user.key().as_ref(), metadata.key().as_ref()], 
        bump)]
    pub user_permissions: Box<Account<'info, MetadataPermissions>>,

    #[account(mut, has_one = collection)]
    pub metadata: Box<Account<'info, Metadata>>,

    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EditMetadataPermissions>, edit_permissions_input: EditMetadataPermissionsInput) -> Result<()> {
    let user_permissions = &mut ctx.accounts.user_permissions;
    let auth_permissions = & ctx.accounts.auth_permissions;
    let collection = &ctx.accounts.collection;
    let auth = &ctx.accounts.authority;
    let user = &ctx.accounts.user;
    let metadata = &ctx.accounts.metadata;
    
    assert_valid_collection_permissions(auth_permissions, &collection.key(), auth.key)?;

    if !auth_permissions.is_admin {
        return Err(ErrorCode::MissingPermissionAdmin.into());
    }
 
    let EditMetadataPermissionsInput {can_modify} = edit_permissions_input;

    user_permissions.can_modify = can_modify;
    user_permissions.metadata = ctx.accounts.metadata.key();
    user_permissions.user = user.key();

    emit!(MetadataPermissionEvent {
        metadata: metadata.key(),
        user: user.key(),
    });

    Ok(())                                
}