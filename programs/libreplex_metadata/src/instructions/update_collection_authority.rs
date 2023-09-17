use anchor_lang::prelude::*;

use crate::Collection;


#[derive(Accounts)]
pub struct UpdateCollectionAuthority<'info> {
    pub update_authority: Signer<'info>,

    #[account(mut, has_one = update_authority)]
    pub collection: Box<Account<'info, Collection>>,
}

pub fn handler(ctx: Context<UpdateCollectionAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.collection.update_authority = new_authority;

    Ok(())
}