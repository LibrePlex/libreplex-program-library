use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("Cb5quBzLtDfFWcAHrdQKc4TooCEuFRu6fiPNhsyATWxd");

pub mod errors;
pub mod state;

pub mod logic;
pub use logic::*;

pub use state::*;

pub mod group_extension_program {
    use anchor_lang::declare_id;
    declare_id!("TGRPp2mDGxSyH3We9hH8pwcmhajtszPAvWjVdVgsPa5");
}

#[program]
pub mod libreplex_editions {
    
    use super::*;

    // v2 endpoints. Prefer these over the original ones. 
    // they allow setting of optional creator co-signer
    // and toggling inscriptions on and off. 
    // for now, creator co-sign is disabled but will be enabled
    // soon to allow for wrapper contracts
    pub fn initialise(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
        instructions::initialise(ctx, input)
    }

    pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
        instructions::mint(ctx)
    }

    pub fn update_mint_authority<'info>(ctx: Context<'_, '_, '_, 'info, UpdateMintAuthorityCtx<'info>>) -> Result<()> {
        instructions::update_mint_authority(ctx)
    }

    
}
