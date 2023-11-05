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

    pub fn inscribe_legacy_metadata(
        ctx: Context<InscribeLegacyMetadata>,
        authority_type: AuthorityType
    ) -> Result<()> {
        instructions::inscribe_metaplex_metadata::handler(ctx,authority_type)
    }

    pub fn write_to_legacy_inscription(
        ctx: Context<WriteToLegacyInscription>,
        authority_type: AuthorityType,
        input: libreplex_inscriptions::instructions::WriteToInscriptionInput,
    ) -> Result<()> {
        instructions::write_to_legacy_inscription::handler(ctx, authority_type,  input)
    }

    pub fn resize_legacy_inscription(
        ctx: Context<ResizeLegacyInscription>,
        input: libreplex_inscriptions::instructions::ResizeInscriptionInput,
        authority_type: AuthorityType
    ) -> Result<()> {
        instructions::resize_legacy_inscription::handler(ctx, input, authority_type)
    }

    pub fn make_immutable(
        ctx: Context<MakeImmutable>,
        authority_type: AuthorityType
    ) -> Result<()> {
        instructions::make_immutable::handler(ctx, authority_type)
    }
}
