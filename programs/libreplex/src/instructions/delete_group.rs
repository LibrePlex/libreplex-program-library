use anchor_lang::prelude::*;

use prog_common::{close_account, errors::ErrorCode};

use crate::{
    assert_valid_permissions,
    Group, PermissionType, Permissions, GroupEvent, GroupEventType,
};

#[derive(Accounts)]
pub struct DeleteGroup<'info> {
    pub signer: Signer<'info>,

    #[account(mut,
        close = creator,
        seeds = ["permissions".as_ref(), group.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,

    /// CHECK: checked in macro. This is the collection creator
    pub creator: UncheckedAccount<'info>,

    #[account(mut,
        constraint = group.creator == creator.key())]
    pub group: Box<Account<'info, Group>>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteGroup>) -> Result<()> {
    //assert_valid_collection_permissionsmports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;
    let permissions = &ctx.accounts.permissions;
    let group = &ctx.accounts.group;
    assert_valid_permissions(
        permissions,
        ctx.accounts.group.key(),
        ctx.accounts.signer.key(),
        &PermissionType::Admin,
    )?;

    if ctx.accounts.group.item_count > 0 {
        return Err(ErrorCode::CollectionHasItems.into());
    }

    // Close the collection data state account
    let collection_data_account_info = &mut (*ctx.accounts.group).to_account_info();
    close_account(collection_data_account_info, receiver)?;

    msg!(
        "Collection data with pubkey {} now deleted",
        ctx.accounts.group.key()
    );

    emit!(GroupEvent {
        authority: ctx.accounts.signer.key(),
        name: group.name.clone(),
        id: group.key(),
        event_type: GroupEventType::Delete
    });
    Ok(())
}
