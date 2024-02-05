use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod initialise;
pub use initialise::*;

pub mod mint;
pub use mint::*;

pub mod swap_to_fungible;
pub use swap_to_fungible::*;

pub mod bootstrap_pool;
pub use bootstrap_pool::*;

declare_id!("LiquGRWGrp8JKspo8zDDu6qpRmX1p6U3PX2USqiE1eg");

#[program]
pub mod libreplex_liquidity {
    use super::*;

    pub fn swap_to_fungible(ctx: Context<SwapToFungible>) -> Result<()> {
        swap_to_fungible_handler(ctx)
    }

    pub fn mint(ctx: Context<MintCtx>) -> Result<()> {
        mint_handler(ctx)
    }

    pub fn initialise(ctx: Context<Initialise>, input: InitialiseInput) -> Result<()> {
        init_handler(ctx, input)
    }

    pub fn bootstrap_pool(ctx: Context<BootstapPool>) -> Result<()> {
        bootstrap_pool_handler(ctx)
    }

    pub fn prepare_native_escrow(ctx: Context<PrepareNativeEscrow>) -> Result<()> {
        prepare_native_escrow_handler(ctx)
    }
}
