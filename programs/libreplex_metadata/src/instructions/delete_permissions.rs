use anchor_lang::prelude::*;


use crate::DelegatePermissions;

/* 
    Intended for cleaning up one's own permissions Ø
    after the collection has been deleted.
 */
#[derive(Accounts)]
pub struct DeletePermissions<'info> {
    pub update_authority: Signer<'info>,

    #[account(mut, close = update_authority, has_one = update_authority)]
    pub permissions: Box<Account<'info, DelegatePermissions>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeletePermissions>) -> Result<()> {
    msg!(
        "Permissions with pubkey {} now deleted",
        ctx.accounts.permissions.key()
    );
    Ok(())
}