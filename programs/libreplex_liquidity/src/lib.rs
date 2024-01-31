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

declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXH");

#[program]
pub mod libreplex_liqudity {
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
}
