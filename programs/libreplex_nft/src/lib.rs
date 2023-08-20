use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod errors;
pub mod state;

declare_id!("LibrAsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p");

#[program]
pub mod libreplex_metadata {
    
    use super::*;

    pub fn wrap(
        ctx: Context<WrapCtx>,
    ) -> Result<()> {
        instructions::wrap::handler(ctx)
    }

    pub fn toggle_freeze(
        ctx: Context<ToggleFreezeCtx>,
        input: ToggleFreezeInput,
    ) -> Result<()> {
        instructions::toggle_freeze::handler(ctx, input)
    }

}

