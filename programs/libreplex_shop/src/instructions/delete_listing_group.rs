use crate::{state::{ ListingGroup}, errors::ShopError};
use anchor_lang::{prelude::*};

#[event]
struct DeleteListingGroupEvent {
    id: Pubkey,
}

#[derive(Accounts)]
pub struct DeleteListingGroup<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        close = admin,
        constraint = listing_group.admin == admin.key(),
    )]
    pub listing_group: Account<'info, ListingGroup>,


    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteListingGroup>) -> Result<()> {
    let listing_group = &mut ctx.accounts.listing_group;
    // if listing_group.listings_active > 0 {
    //     return Err(ShopError::GroupHasActiveListings.into());
    // }

    if listing_group.filter_count > 0 {
        return Err(ShopError::GroupHasActiveFilters.into());
    }

    emit!(DeleteListingGroupEvent{
        id: listing_group.key(),
    });

    Ok(())
}
