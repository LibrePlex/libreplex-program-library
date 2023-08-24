
use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod errors;
pub mod state;

declare_id!("9SXDHUdtfvBGT3H2uPCNEkxmWREoqdeS1qdBudLDD6KX");

#[program]
pub mod libreplex_nft {
    
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

