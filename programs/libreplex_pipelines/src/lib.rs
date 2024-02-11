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

    pub fn mcc_pipeline_initialise(
        ctx: Context<MccPipelineInitialiseCtx>,
        fair_launch_input: libreplex_fair_launch::InitialiseInputV2,
        pipeline_input: InitialiseMetaplexPipelineInput,
    ) -> Result<()> {
        instructions::mcc_pipeline_initialise::mcc_pipeline_initialise(ctx, fair_launch_input, pipeline_input)
    }

    pub fn mcc_pipeline_create_swap(ctx: Context<MccPipelineCreateSwap>) -> Result<()> {
        instructions::mcc_pipeline_create_swap::mcc_pipeline_create_swap(ctx)
    }
}
