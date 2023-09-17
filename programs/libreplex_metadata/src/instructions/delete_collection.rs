use anchor_lang::prelude::*;
use crate::state::Collection;

use crate::errors::ErrorCode;


#[event]
pub struct CollectionEventDelete {
    pub authority: Pubkey,
    pub name: String,
    pub id: Pubkey,    
}

#[derive(Accounts)]
pub struct DeleteCollection<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, 
        close = authority,
        constraint = group.update_authority == authority.key())]
    pub group: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteCollection>
) -> anchor_lang::Result<()> {

    let group = &ctx.accounts.group;

    if group.item_count > 0 {
        return Err(ErrorCode::CollectionHasItems.into());
    }
    emit!(CollectionEventDelete{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key()
    });

    Ok(())
}
