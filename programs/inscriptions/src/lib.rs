use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp");

pub mod instructions;
pub mod state;

pub use state::*;

#[program]
pub mod inscriptions {

    use super::*;

    pub fn create_ordinal(
        ctx: Context<CreateOrdinal>,
        ordinal_input: CreateOrdinalInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_ordinal::handler(
            ctx,
            ordinal_input
        )
    }

    pub fn append_to_ordinal(
        ctx: Context<AppendToOrdinal>,
        input: AppendToOrdinalInput,
    ) -> Result<()> {
        instructions::append_to_ordinal::handler(
            ctx,
            input
        )
    }



}
