use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("insFmVukT9LYVygNbdpSjbxPy4FtQ6WgcuChnxDLbAm");

pub mod state;
pub mod errors;

pub use state::*;

#[program]
pub mod libreplex_spl20 {

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

    // pub fn mint(
    //     ctx: Context<MintCtx>,
    // ) -> Result<()> {
    //     instructions::mint::mint(
    //         ctx,
    //     )
    // }   
}
