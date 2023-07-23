use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp");

pub mod instructions;
pub mod state;

pub mod errors;
pub use errors::*;

pub use state::*;

#[program]
pub mod libreplex_inscriptions {

    use super::*;

    pub fn create_inscription(
        ctx: Context<CreateInscription>,
        ordinal_input: CreateInscriptionInput,
    ) -> Result<()> {
        instructions::create_inscription::handler(
            ctx,
            ordinal_input
        )
    }

    pub fn delete_inscription(
        ctx: Context<DeleteInscription>
    ) -> Result<()> {
        instructions::delete_inscription::handler(
            ctx
        )
    }

    pub fn resize_inscription(
        ctx: Context<ResizeInscription>,
        input: ResizeInscriptionInput,
    ) -> Result<()> {
        instructions::resize_inscription::handler(
            ctx,
            input
        )
    }


    pub fn write_to_inscription(
        ctx: Context<WriteToInscription>,
        input: WriteToInscriptionInput,
    ) -> Result<()> {
        instructions::write_to_inscription::handler(
            ctx,
            input
        )
    }



}