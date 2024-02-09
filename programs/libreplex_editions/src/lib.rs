use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("Eddy2qpzTgTfUvNJ271Wgfw3RF2LsbmRJdNTSnZajmU3");

pub mod errors;
pub mod state;

pub use state::*;

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

    
}
