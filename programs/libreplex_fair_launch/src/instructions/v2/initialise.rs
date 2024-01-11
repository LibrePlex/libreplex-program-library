use anchor_lang::prelude::*;



use crate::{Deployment, initialise_logic, InitialiseInput, TOKEN2022_DEPLOYMENT_TYPE, DeploymentConfig, errors::FairLaunchError};



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInputV2 {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub decimals: u8,
    pub ticker: String,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub require_creator_cosign: bool,
    pub use_inscriptions: bool,
    pub deployment_type: u8,
    pub creator_fee_treasury: Pubkey,
    pub creator_fee_per_mint_in_lamports: u64
}

/*

    Initialise sets the main template parameters of the deployment:
    1) ticker
    2) deployment template
    3) mint template
    4) decimals
    5) limit per mint
    6) max number of tokens

    It does not create any inscriptions / mints as these are handled by the deploy endpoints.
    This method is metadata agnostic.

*/

#[derive(Accounts)]
#[instruction(input: InitialiseInputV2)]
pub struct InitialiseV2Ctx<'info>  {
    #[account(init, payer = payer, space = 8 + Deployment::INIT_SPACE, 
        seeds = ["deployment".as_ref(), input.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(
        init,
        payer = payer,
        space = DeploymentConfig::SIZE,
        // deployment must be executed by the payer 
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump
    )]
    pub deployment_config: Account<'info, DeploymentConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}



pub fn initialise_v2(ctx: Context<InitialiseV2Ctx>, input: InitialiseInputV2) -> Result<()> {
    
    let deployment: &mut Account<'_, Deployment> = &mut ctx.accounts.deployment;

    let deployment_config = &mut ctx.accounts.deployment_config;

    

    let creator = &ctx.accounts.creator;

    let InitialiseInputV2 { 
        limit_per_mint, 
        max_number_of_tokens, 
        decimals, 
        ticker, 
        deployment_template, 
        mint_template, 
        offchain_url, 
        require_creator_cosign, 
        use_inscriptions, 
        deployment_type,
        creator_fee_per_mint_in_lamports: creator_fee_in_lamports,
        creator_fee_treasury} = input;

    if require_creator_cosign {
        panic!("Creator cosign not currently supported")
    }

    if deployment_type != TOKEN2022_DEPLOYMENT_TYPE {
        panic!("Only token 2022 currently supported in v2 methods")
    }


    if creator_fee_in_lamports  > 50_000_000 {
        return Err(FairLaunchError::CreatorFeeTooHigh.into())
    }

    deployment_config.creator_fee_treasury = creator_fee_treasury;
    deployment_config.creator_fee_per_mint_lamports = creator_fee_in_lamports;
    

    initialise_logic(InitialiseInput {
        limit_per_mint, 
        max_number_of_tokens, decimals, ticker, deployment_template, mint_template, offchain_url, deployment_type
    }, deployment, creator.key())?;

    
    deployment.require_creator_cosign = require_creator_cosign;
    deployment.use_inscriptions = use_inscriptions;

    Ok(())


}