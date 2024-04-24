use anchor_lang::prelude::*;

use crate::{
    DeploymentV2, Hashlist
};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

/*

    Deploy takes no input parameters as all of the
    string parameter + decimals have already been set by
    initialise.

    Deploy creates all on-chain objects (inscriptions,
    mints + any metadata) that are required to keep track of the
    launch lifecycle.
*/
#[derive(Accounts)]
pub struct DeployRawCtx<'info> {
    #[account(
        mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()],
        bump
    )]
    pub deployment: Account<'info, DeploymentV2>,

    #[account(init, seeds = ["hashlist".as_bytes(), 
    deployment.key().as_ref()],
    bump, payer = payer, space = 8 + 32 + 4)]
    pub hashlist: Box<Account<'info, Hashlist>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account()]
    pub system_program: Program<'info, System>,

  
}

pub fn deploy_raw(ctx: Context<DeployRawCtx>) -> Result<()> {
    let hashlist = &mut ctx.accounts.hashlist;
    let deployment = &mut ctx.accounts.deployment;

    if deployment.deployed {
        //
        panic!("Already deployed");
    }
    // let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    hashlist.deployment = deployment.key();
    deployment.deployed = true;
    
    Ok(())
}

