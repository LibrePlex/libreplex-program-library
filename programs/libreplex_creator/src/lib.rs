use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};



pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;

pub use constants::*;
pub use state::*;

declare_id!("78deTr7qycJ6498vSd3pNMhdCKKWxMipniitVHQcM8RM");

#[program]
pub mod libreplex_creator {
    use super::*;

    pub fn create_creator(
        ctx: Context<CreateCreator>,
        creator_input: CreateCreatorInput,
    ) -> Result<()> {
        msg!("creating collection data");
        instructions::create_creator::handler(
            ctx,
            creator_input
        )
    }

    pub fn update(ctx: Context<UpdateCreator>, input: UpdateInput) -> Result<()> {
        instructions::update::handler(ctx, input)
    }


    pub fn mint(ctx: Context<Mint>,) -> Result<()> {
        instructions::mint::handler(ctx)
    }



}
