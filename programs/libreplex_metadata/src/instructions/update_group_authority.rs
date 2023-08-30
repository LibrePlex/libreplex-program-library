use anchor_lang::prelude::*;

use crate::Group;


#[derive(Accounts)]
pub struct UpdateGroupAuthority<'info> {
    pub update_authority: Signer<'info>,

    #[account(mut, has_one = update_authority)]
    pub group: Box<Account<'info, Group>>,
}

pub fn handler(ctx: Context<UpdateGroupAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.group.update_authority = new_authority;

    Ok(())
}