use anchor_lang::prelude::*;

use crate::{state::{Group}, Permissions, assert_valid_permissions, PermissionType, MetadataExtension};
use prog_common::{close_account, TrySub};


#[derive(Accounts)]
pub struct DeleteMetadataExtension<'info> {

    pub authority: Signer<'info>,

    pub permissions: Box<Account<'info, Permissions>>,

    #[account(mut)]
    pub group: Box<Account<'info, Group>>,

    #[account(mut, has_one = group)]
    pub metadata_extension: Box<Account<'info, MetadataExtension>>,


    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteMetadataExtension>) -> Result<()> {

    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;
    let authority = &ctx.accounts.authority;
    let group = &mut ctx.accounts.group;
    let permissions = &ctx.accounts.permissions;

    // only collection admin can delete metadata. TODO: Extend logic to allow deletions when the authority has Delete
    // permissions on collection / metadata
    assert_valid_permissions(permissions, group.key(), authority.key(), &PermissionType::Admin)?;

    // Close the collection data state account
    let metadata_extension_account_info = &mut (*ctx.accounts.metadata_extension).to_account_info();
    close_account(metadata_extension_account_info, receiver)?;

    // Decrement collection data counter
    group.item_count.try_sub_assign(1)?;

    msg!("Metadata extension with pubkey {} now deleted", metadata_extension_account_info.key());
    Ok(())
}