use anchor_lang::prelude::*;
use crate::Group;

use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct DeleteGroup<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, 
        close = authority,
        constraint = group.update_authority == authority.key())]
    pub group: Box<Account<'info, Group>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteGroup>
) -> anchor_lang::Result<()> {

    let group = &ctx.accounts.group;

    if group.item_count > 0 {
        return Err(ErrorCode::CollectionHasItems.into());
    }

    Ok(())
}
