use crate::{
    state::{ListingFilter, ListingGroup},
};
use anchor_lang::{prelude::*};

#[event]
struct DeleteListingFilterEvent {
    id: Pubkey,
}


#[derive(Accounts)]
pub struct DeleteListingFilter<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        close = admin,
        constraint = listing_filter.listing_group == listing_group.key()
    )]
    pub listing_filter: Account<'info, ListingFilter>,

    #[account(
        mut,
        constraint = listing_group.admin == admin.key(),
    )]
    pub listing_group: Account<'info, ListingGroup>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DeleteListingFilter>,
) -> Result<()>     {
    
    let listing_group = &mut ctx.accounts.listing_group;
  
    listing_group.filter_count -= 1;

    emit!(DeleteListingFilterEvent{
        id: listing_group.key(),
    });

    Ok(())
}
