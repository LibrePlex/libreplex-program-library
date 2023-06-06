use anchor_lang::prelude::*;


use prog_common::{close_account, errors::ErrorCode};

use crate::{Collection, instructions::{CollectionEvent, CollectionEventType}, Permissions, assert_valid_permissions, PermissionType};

#[derive(Accounts)]
pub struct DeleteCollection<'info> {
    pub signer: Signer<'info>,

    #[account(mut,
        close = creator,
        seeds = ["permissions".as_ref(), collection.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,



    /// CHECK: checked in macro. This is the collection creator
    pub creator: UncheckedAccount<'info>,

    #[account(mut,
        constraint = collection.creator == creator.key())]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteCollection>, permission_invoked: PermissionType) -> Result<()> {
    //assert_valid_collection_permissionsmports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;
    let permissions = &ctx.accounts.permissions;
    let collection = &ctx.accounts.collection;

    if permission_invoked != PermissionType::Admin && permission_invoked != PermissionType::Delete {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    assert_valid_permissions(
        permissions,
        ctx.accounts.collection.key(),
        ctx.accounts.signer.key(),
        permission_invoked
    )?;

    if ctx.accounts.collection.item_count > 0 {
        return Err(ErrorCode::CollectionHasItems.into())
    }

    // Close the collection data state account
    let collection_data_account_info = &mut (*ctx.accounts.collection).to_account_info();
    close_account(collection_data_account_info, receiver)?;

    msg!(
        "Collection data with pubkey {} now deleted",
        ctx.accounts.collection.key()
    );


    emit!(CollectionEvent{
        creator: ctx.accounts.signer.key(),
        name: collection.name.clone(),
        id: collection.key(),
        event_type: CollectionEventType::Delete
    });
    Ok(())
}