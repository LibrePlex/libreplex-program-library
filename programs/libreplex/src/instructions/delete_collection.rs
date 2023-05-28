use anchor_lang::prelude::*;

use crate::{assert_valid_user_permissions, state::Collection, CollectionPermissions};
use prog_common::{close_account, errors::ErrorCode};

#[derive(Accounts)]
pub struct DeleteCollection<'info> {
    pub signer: Signer<'info>,

    #[account(mut,
        close = creator,
        seeds = ["permissions".as_ref(), collection.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub signer_collection_permissions: Box<Account<'info, CollectionPermissions>>,

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

pub fn handler(ctx: Context<DeleteCollection>) -> Result<()> {
    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;
    let permissions = &ctx.accounts.signer_collection_permissions;

    assert_valid_user_permissions(
        permissions,
        &ctx.accounts.collection.key(),
        ctx.accounts.signer.key,
    )?;

    if !permissions.can_delete_collection {
        return Err(ErrorCode::MissingPermissionDeleteCollection.into());
    }

    // Close the collection data state account
    let collection_data_account_info = &mut (*ctx.accounts.collection).to_account_info();
    close_account(collection_data_account_info, receiver)?;

    msg!(
        "Collection data with pubkey {} now deleted",
        ctx.accounts.collection.key()
    );
    Ok(())
}
