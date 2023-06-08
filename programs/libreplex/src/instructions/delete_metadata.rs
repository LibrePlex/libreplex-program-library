use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{state::{Group, Metadata}, METADATA, Permissions, assert_valid_permissions, PermissionType};
use prog_common::{close_account, TrySub};


#[derive(Accounts)]
pub struct DeleteMetadata<'info> {

    pub authority: Signer<'info>,

    pub permissions: Box<Account<'info, Permissions>>,

    #[account(mut)]
    pub group: Box<Account<'info, Group>>,

    #[account(mut, seeds = [METADATA.as_ref(), mint.key().as_ref()],
              bump, has_one = mint)]
    pub metadata: Box<Account<'info, Metadata>>,

    pub mint: Account<'info, Mint>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteMetadata>) -> Result<()> {

    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;
    let authority = &ctx.accounts.authority;
    let group = &mut ctx.accounts.group;
    let permissions = &ctx.accounts.permissions;

    // only collection admin can delete metadata. TODO: Extend logic to allow deletions when the authority has Delete
    // permissions on collection / metadata
    assert_valid_permissions(permissions, group.key(), authority.key(), &PermissionType::Admin)?;

    // Close the collection data state account
    let metadata_account_info = &mut (*ctx.accounts.metadata).to_account_info();
    close_account(metadata_account_info, receiver)?;

    // Decrement collection data counter
    group.item_count.try_sub_assign(1)?;

    msg!("Metadata with pubkey {} now deleted", ctx.accounts.metadata.key());
    Ok(())
}