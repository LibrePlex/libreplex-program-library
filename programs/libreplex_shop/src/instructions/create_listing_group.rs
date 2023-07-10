use crate::{state::{Listing, Price, ListingFilter, ListingGroup}, constants::LISTING_GROUP};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TokenAccount},
};

use libreplex_shared::transfer_tokens;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateListingGroupInput {
    pub seed: Pubkey,
    pub name: String
}

impl CreateListingGroupInput {
    pub fn get_size(&self) -> u32 {
        8 + 32
    }
}

#[derive(Accounts)]
#[instruction(create_listing_group_input: CreateListingGroupInput)]
pub struct CreateListingGroup<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ListingGroup::BASE_SIZE,
        seeds = [
            LISTING_GROUP.as_ref(),
            admin.key().as_ref(),
            create_listing_group_input.seed.as_ref(),
        ],
        bump
    )]
    pub listing_group: Account<'info, ListingGroup>,


    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateListingGroup>, create_listing_group_input: CreateListingGroupInput) -> Result<()> {
    let listing_group = &mut ctx.accounts.listing_group;
    listing_group.admin = ctx.accounts.admin.key();
    listing_group.seed = create_listing_group_input.seed;
    msg!("name: {}", create_listing_group_input.name);
    listing_group.name = create_listing_group_input.name;
    listing_group.listings_active = 0;
    listing_group.listings_created = 0;
    listing_group.listings_sold = 0;
    listing_group.filter_count = 0;
    

    Ok(())
}
