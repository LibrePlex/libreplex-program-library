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
pub mod libreplex_legacy {

    use super::*;

    pub fn inscribe_legacy_metadata_as_uauth(
        ctx: Context<InscribeLegacyMetadataAsUauth>,
        // not optional - for legacy metadata we need to have some validation hash
        // so we can verify the inscription once it's fully written
        validation_hash: String,
    ) -> Result<()> {
        instructions::inscribe_legacy_metadata_as_uauth::handler(
            ctx,
            validation_hash,
        )
    }

    pub fn inscribe_legacy_metadata_as_holder(
        ctx: Context<InscribeLegacyMetadataAsHolder>,
        // not optional - for legacy metadata we need to have some validation hash
        // so we can verify the inscription once it's fully written
        validation_hash: String,
    ) -> Result<()> {
        instructions::inscribe_legacy_metadata_as_holder::handler(
            ctx,
            validation_hash,
        )
    }

    pub fn write_to_legacy_inscription(
        ctx: Context<WriteToLegacyInscription>,
        input: WriteToLegacyInscriptionInput,
    ) -> Result<()> {
        instructions::write_to_legacy_inscription::handler(ctx, input)
    }

    pub fn resize_legacy_inscription(
        ctx: Context<ResizeLegacyInscription>,
        input: ResizeLegacyInscriptionInput,
    ) -> Result<()> {
        instructions::resize_legacy_inscription::handler(ctx, input)
    }

    pub fn make_immutable(ctx: Context<MakeImmutable>) -> Result<()> {
        instructions::make_immutable::handler(ctx)
    }

    pub fn set_validation_hash(
        ctx: Context<SetValidationHash>,
        validation_hash: Option<String>,
    ) -> Result<()> {
        instructions::set_validation_hash::handler(ctx, validation_hash)
    }
}
