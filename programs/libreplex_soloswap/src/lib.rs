use anchor_lang::prelude::*;

declare_id!("SoLopnx594TvVjTj5td3UxYEHFNWkkRcjmr13AaFXHe");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::*;

pub use constants::*;
pub use state::*;
pub use instructions::*;

#[program]
pub mod libreplex_multiswap {
    use super::*;

    pub fn create_swap(
        ctx: Context<CreateSwapCtx>,
        input: CreateSwapInput,
    ) -> Result<()> {
        instructions::create_swap::create_swap(ctx, input)
    }


    pub fn swap(
        ctx: Context<SwapCtx>,
    ) -> Result<()> {
        instructions::swap::swap(ctx)
    }

}
