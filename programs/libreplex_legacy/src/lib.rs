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

    pub fn claim_excess_rent_as_uauth(
        ctx: Context<ClaimExcessRentAsUauth>,
        // not optional - for legacy metadata we need to have some validation hash
        // so we can verify the inscription once it's fully written
    ) -> Result<()> {
        instructions::claim_excess_rent_as_uauth::handler(
            ctx
        )
    }


    
    pub fn set_validation_hash(
        ctx: Context<SetValidationHash>,
        validation_hash: Option<String>,
    ) -> Result<()> {
        instructions::set_validation_hash::handler(ctx, validation_hash)
    }

    /* v3 methods */

    pub fn inscribe_legacy_metadata_as_uauth_v3(
        ctx: Context<InscribeLegacyMetadataAsUauthV3>,
        // not optional - for legacy metadata we need to have some validation hash
        // so we can verify the inscription once it's fully written
        validation_hash: String,
    ) -> Result<()> {
        instructions::inscribe_legacy_metadata_as_uauth_v3::handler(
            ctx,
            validation_hash,
        )
    }

    pub fn write_to_legacy_inscription_as_uauth_v3(
        ctx: Context<WriteToLegacyInscriptionAsUAuthV3>,
        input: libreplex_inscriptions::instructions::WriteToInscriptionInput,
    ) -> Result<()> {
        instructions::write_to_legacy_inscription_as_uauth_v3::handler(ctx, input)
    }

    pub fn resize_legacy_inscription_as_uauth_v3(
        ctx: Context<ResizeLegacyInscriptionAsUauthV3>,
        input: ResizeLegacyInscriptionInput,
    ) -> Result<()> {
        instructions::resize_legacy_inscription_as_uauth_v3::handler(ctx, input)
    }


}
