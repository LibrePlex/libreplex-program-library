use anchor_lang::prelude::*;

use crate::{Collection};


#[derive(Accounts)]
#[instruction(
    seed: Pubkey
)]
pub struct DeleteCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, 
        close = authority,
        constraint = authority.key() == collection.authority)]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
    
}


