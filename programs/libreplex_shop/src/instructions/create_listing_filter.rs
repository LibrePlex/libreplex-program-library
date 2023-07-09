use crate::{
    constants::{LISTING_GROUP, LISTING_FILTER},
    state::{Listing, ListingFilter, ListingGroup, Price, ListingFilterType},
};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::{associated_token::AssociatedToken, token::TokenAccount};

use libreplex_shared::transfer_tokens;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateListingFilterInput {
    pub filter_type: ListingFilterType,
    pub seed: Pubkey,
}


#[derive(Accounts)]
#[instruction(create_listing_filter_input: CreateListingFilterInput)]
pub struct CreateListingFilter<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ListingFilter::BASE_SIZE,
        seeds = [
            LISTING_FILTER.as_ref(),
            admin.key().as_ref(),
            create_listing_filter_input.seed.as_ref(),
        ],
        bump
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
    ctx: Context<CreateListingFilter>,
    create_listing_filter_input: CreateListingFilterInput
) -> Result<()> {
    let listing_group = &mut ctx.accounts.listing_group;
    listing_group.filter_count += 1;

    let listing_filter = &mut ctx.accounts.listing_filter;
    listing_filter.listing_group = listing_group.key();
    listing_filter.filter_type = create_listing_filter_input.filter_type;
    listing_filter.seed = create_listing_filter_input.seed;
    

    Ok(())
}
