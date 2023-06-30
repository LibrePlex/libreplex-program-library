
use anchor_lang::{prelude::*};

pub mod errors;
pub mod instructions;
pub mod state;
pub mod controls;

use instructions::*;

declare_id!("toysDBtkQ7P1NWwaTtXZbXBeGdM16fReT5qzCgc7wia");


#[program]
pub mod libreplex_creator_controls {
    use super::*;

    pub fn mint<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Mint<'info>>, input: MintInput) -> Result<()> {
        mint::handler(ctx, input)
    }


    pub fn initialize(ctx: Context<Initialize>, input: InitializeInput) -> Result<()> {
        initialize::handler(ctx, input)
    }
}


