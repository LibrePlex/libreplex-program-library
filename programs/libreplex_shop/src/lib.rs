use anchor_lang::prelude::*;
use instructions::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};



pub mod instructions;
pub mod state;
pub mod constants;

pub mod errors;


declare_id!("ListjawGEdhxuAErSyYwcTEGWQswFoi6FScnGG1RKSB");


#[program]
pub mod libreplex_shop {

    use super::*;
    pub fn list(
        ctx: Context<List>,
        list_input: ListInput
    ) -> Result<()> {
        instructions::list::handler(ctx, list_input)

        
    }

    pub fn delist(
        ctx: Context<Delist>
    ) -> Result<()> {
        instructions::delist::handler(ctx)
    }

    pub fn execute<'info>(
        ctx: Context<'_, '_, '_, 'info, Execute<'info>>
    ) -> Result<()> {
        instructions::execute::handler(ctx)
    }


}
