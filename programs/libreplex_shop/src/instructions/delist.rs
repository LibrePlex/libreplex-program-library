use crate::state::{Listing};
use anchor_lang::{prelude::*};
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub lister: Signer<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut,
        close=lister,
        constraint = listing.lister == lister.key()
        )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
}

pub fn handler(_: Context<Delist>) -> Result<()> {
    Ok(())
}
