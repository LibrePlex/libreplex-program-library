use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod initialise;
pub use initialise::*;

pub mod relinquish_cosigner;
pub use relinquish_cosigner::*;

pub mod reduce_mint_count;
pub use reduce_mint_count::*;

pub mod create_lookup_table_for_liquidity;
pub use create_lookup_table_for_liquidity::*;

pub mod mint;
pub use mint::*;

pub mod mint_spl;
pub use mint_spl::*;

pub mod swap_to_fungible;
pub use swap_to_fungible::*;

pub mod bootstrap_pool;
pub use bootstrap_pool::*;

pub mod initialise_v2;
pub use initialise_v2::*;

pub mod join;
pub use join::*;

declare_id!("LiquGRWGrp8JKspo8zDDu6qpRmX1p6U3PX2USqiE1eg");

#[program]
pub mod libreplex_liquidity {
    use super::*;

    pub fn swap_to_fungible(ctx: Context<SwapToFungible>) -> Result<()> {
        swap_to_fungible_handler(ctx)
    }

    pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
        mint_handler(ctx)
    }

    pub fn join<'info>(ctx: Context<'_, '_, '_, 'info, JoinCtx<'info>>, input: JoinInput) -> Result<()> {
        join_handler(ctx, input)
    }

    pub fn create_lookup_table(ctx: Context<CreateLookupTableForLiquidityCtx>,
    recent_slot: u64) -> Result<()> {
        create_lookup_table_for_liquidity(ctx, recent_slot)
    }

    pub fn initialise(ctx: Context<Initialise>, input: InitialiseInput) -> Result<()> {
        init_handler(ctx, input)
    }

    pub fn reduce_mint_count<'info>(
        ctx: Context<'_, '_, '_, 'info, ReduceMintCountCtx<'info>>,
        input: ReduceMintCountInputLiquidity
    ) -> Result<()> {
        handle_reduce_mint_count(ctx, input)
    }
    
    pub fn initialise_v2(ctx: Context<InitialiseV2>, input: InitialiseInputV2) -> Result<()> {
        init_handler_v2(ctx, input)
    }
    

    pub fn bootstrap_pool<'a>(ctx: Context<'_,'_,'_, 'a, BootstapPool<'a>>) -> Result<()> {
        bootstrap_pool_handler(ctx)
    }

    pub fn relinquish_cosigner(ctx: Context<RelinquishCosignersCtx>) -> Result<()> {
        relinquish_cosigner_handler(ctx)
    }

    pub fn fix_deployment_type<'a>(ctx: Context<'_,'_,'_, 'a, BootstapPool<'a>>) -> Result<()> {
        bootstrap_pool_handler(ctx)
    }


    pub fn prepare_native_escrow(ctx: Context<PrepareNativeEscrow>) -> Result<()> {
        prepare_native_escrow_handler(ctx)
    }

    pub fn mint_spl<'info>(ctx: Context<'_, '_, '_, 'info, MintSplCtx<'info>>) -> Result<()> {
        mint_spl_handler(ctx)
    }

    

}
