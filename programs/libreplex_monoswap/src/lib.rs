use anchor_lang::prelude::*;

declare_id!("MonoRPwMWxcsVEJV27jyEt1f5VoWg3szDBRYUenm221");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod libreplex_monoswap {

    use super::*;

    pub fn create_monoswap(
        ctx: Context<CreateMonoSwapCtx>,
        input: CreateMonoSwapInput,
    ) -> Result<()> {
        process_create_swap(ctx, input)
    }

    pub fn swap(ctx: Context<SwapCtx>) -> Result<()> {
        process_swap(ctx)
    }

    pub fn create_nifty_swap(ctx: Context<CreateNiftySwapCtx>, amount: u64) -> Result<()> {
        process_create_nifty_swap(ctx, amount)
    }

    pub fn nifty_swap(ctx: Context<NiftySwapCtx>, direction: SwapDirection) -> Result<()> {
        process_nifty_swap(ctx, direction)
    }
}
