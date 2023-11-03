use anchor_lang::prelude::*;
use instructions::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("Leg1xVbrpq5gY6mprak3Ud4q4mBwcJi5C9ZruYjWv7n");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::*;

pub use constants::*;
pub use state::*;

#[program]
pub mod libreplex_metadata {

    use super::*;

    pub fn inscribe_legacy_object(
        ctx: Context<InscribeLegacy>,
        input: InscribeLegacyInput,
    ) -> Result<()> {
        instructions::inscribe_legacy::handler(ctx, input)
    }

    pub fn write_to_legacy_inscription(
        ctx: Context<WriteToLegacyInscription>,
        input: libreplex_inscriptions::instructions::WriteToInscriptionInput,
        legacy_input: InscribeLegacyInput,
    ) -> Result<()> {
        instructions::write_to_legacy_inscription::handler(ctx, input, legacy_input)
    }

    pub fn resize_legacy_inscription(
        ctx: Context<ResizeLegacyInscription>,
        input: libreplex_inscriptions::instructions::ResizeInscriptionInput,
        legacy_input: InscribeLegacyInput,
    ) -> Result<()> {
        instructions::resize_legacy_inscription::handler(ctx, input, legacy_input)
    }

    pub fn make_immutable(
        ctx: Context<MakeImmutable>,
        legacy_input: InscribeLegacyInput,
    ) -> Result<()> {
        instructions::make_immutable::handler(ctx, legacy_input)
    }
}
