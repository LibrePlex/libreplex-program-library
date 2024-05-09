use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod relinquish_cosigner;
pub use relinquish_cosigner::*;


pub mod mint;
pub use mint::*;

pub mod update_metadata;
pub use update_metadata::*;


pub mod initialise;
pub use initialise::*;

declare_id!("N1FTSwV4sLvYzPK1wxkeTV88ycD9m6vUoSVV34wkf7c");

#[program]
pub mod libreplex_nifty_hybrid {

    use super::*;

    pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
        mint_handler(ctx)
    }

    pub fn update_metadata<'info>(ctx: Context<'_, '_, '_, 'info, UpdateMetadataCtx<'info>>) -> Result<()> {
        update_metadata_handler(ctx)
    }

    pub fn initialise(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
        init_handler(ctx, input)
    }

    // no deploy method here.  deploy_raw can be called directly 

}
