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

    pub fn create_inscription(
        ctx: Context<CreateInscription>,
        ordinal_input: CreateInscriptionInput,
    ) -> Result<()> {
        msg!("creating metadata");
        instructions::create_inscription::handler(
            ctx,
            ordinal_input
        )
    }

    pub fn append_to_inscription(
        ctx: Context<AppendToInscription>,
        input: AppendToInscriptionInput,
    ) -> Result<()> {
        instructions::append_to_inscription::handler(
            ctx,
            input
        )
    }



}
