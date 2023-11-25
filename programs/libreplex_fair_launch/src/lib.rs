use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("insFmVukT9LYVygNbdpSjbxPy4FtQ6WgcuChnxDLbAm");

pub mod state;
pub mod errors;

pub use state::*;

#[program]
pub mod libreplex_fair_launch {

    use super::*;


   pub fn deploy(
        ctx: Context<DeployCtx>,
        input: DeployInput
    ) -> Result<()> {
        instructions::deploy::deploy(
            ctx,
            input
        )
    }   

    pub fn mint(
        ctx: Context<MintCtx>,
        input: MintInput
    ) -> Result<()> {
        instructions::mint::mint(
            ctx,
            input
        )
    }   


    pub fn swap_to_fungible(
        ctx: Context<SwapToFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_fungible::swap_to_fungible(
            ctx
        )
    }   


    pub fn swap_to_nonfungible(
        ctx: Context<SwapToNonFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_nonfungible::swap_to_nonfungible(
            ctx
        )
    }   




}
