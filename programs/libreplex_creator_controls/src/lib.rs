
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod controls;

use instructions::*;

declare_id!("G9whLiLT9nSkxwWzWvbiKKrTL6yWxvzh2UXqNht5VXqV");


#[program]
pub mod libreplex_creator_controls {
    use super::*;

    pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>, input: MintInput) -> Result<()> {
        mint::handler(ctx, input)
    }


    pub fn initialize(ctx: Context<Initialize>, input: InitializeInput) -> Result<()> {
        initialize::handler(ctx, input)
    }

    pub fn update(ctx: Context<Update>, input: UpdateInput) -> Result<()> {
        instructions::update::handler(ctx, input)
    }

}


