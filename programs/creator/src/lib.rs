use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("toysDBtkQ7P1NWwaTtXZbXBeGdM16fReT5qzCgc7wig");

pub mod instructions;
pub mod state;
pub mod constants;

pub use constants::*;
pub use state::*;

#[program]
pub mod creator {

    use super::*;

    // pub fn create_group(
    //     ctx: Context<CreateGroup>,
    //     collection_input: GroupInput,
    // ) -> Result<()> {
    //     msg!("creating collection data");
    //     instructions::create_group::handler(
    //         ctx,
    //         collection_input
    //     )
    // }



}
