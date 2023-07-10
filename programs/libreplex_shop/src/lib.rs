use anchor_lang::prelude::*;
use instructions::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

pub mod constants;
pub mod instructions;
pub mod state;

pub mod errors;

declare_id!("ListjawGEdhxuAErSyYwcTEGWQswFoi6FScnGG1RKSB");

pub mod empty_account_placeholder {
    use anchor_lang::declare_id;
    declare_id!("11111111111111111111111111111111");
}

#[program]
pub mod libreplex_shop {

    use super::*;
    pub fn list(ctx: Context<List>, list_input: ListInput) -> Result<()> {
        instructions::list::handler(ctx, list_input)
    }

    pub fn delist<'info>(ctx: Context<'_, '_, '_, 'info, Delist<'info>>) -> Result<()> {
        instructions::delist::handler(ctx)
    }

    pub fn execute<'info>(ctx: Context<'_, '_, '_, 'info, Execute<'info>>) -> Result<()> {
        instructions::execute::handler(ctx)
    }

    pub fn create_listing_group<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateListingGroup<'info>>,
        input: CreateListingGroupInput
    ) -> Result<()> {
        instructions::create_listing_group::handler(ctx, input)
    }

    pub fn delete_listing_group<'info>(
        ctx: Context<'_, '_, '_, 'info, DeleteListingGroup<'info>>
    ) -> Result<()> {
        instructions::delete_listing_group::handler(ctx)
    }

    pub fn create_listing_filter<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateListingFilter<'info>>,
        input: CreateListingFilterInput
    ) -> Result<()> {
        instructions::create_listing_filter::handler(ctx, input)
    }


    pub fn delete_listing_filter<'info>(
        ctx: Context<'_, '_, '_, 'info, DeleteListingFilter<'info>>
    ) -> Result<()> {
        instructions::delete_listing_filter::handler(ctx)
    }
}
