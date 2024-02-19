use anchor_lang::prelude::*;

declare_id!("Pipe6YuqZmoHeKTpwETFaZEiALNREGfZqCjMbk9P4UG");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod libreplex_pipelines {
    use super::*;

    pub fn initialise(
        ctx: Context<InitialisePipelineCtx>,
        input: InitialisePipeline,
    ) -> Result<()> {
        instructions::pipeline_initialise::initialise_pipeline(ctx, input)
    }

    pub fn claim_spl_as_liquidity_provider(
        ctx: Context<ClaimSplAsLiquidityProviderCtx>,
    ) -> Result<()> {
        instructions::claim_spl_as_liquidity_provider::claim_spl_as_liquidity_provider(ctx)

    }

    pub fn create_swap(ctx: Context<CreateSwapCtx>, input: FilterInput) -> Result<()> {
        instructions::create_swap::create_swap(ctx, input)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidityCtx>) -> Result<()> {
        instructions::add_liquidity::add_liquidity(ctx)
    }
}
