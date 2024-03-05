use anchor_lang::{prelude::*, system_program};



use crate::{initialise_logic, Deployment, DeploymentConfig, InitialiseInput, HYBRID_DEPLOYMENT_TYPE, TOKEN2022_DEPLOYMENT_TYPE};



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInputV2 {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub decimals: u8,
    pub ticker: String,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub creator_cosign_program_id: Option<Pubkey>,
    pub use_inscriptions: bool,
    pub deployment_type: u8,
    pub creator_fee_treasury: Pubkey,
    pub creator_fee_per_mint_in_lamports: u64,
    // this allows for interesting dynamics
    pub deflation_rate_per_swap: u16
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

    /// CHECK: Can be anyone.
    pub creator: UncheckedAccount<'info>,

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
        use_inscriptions, 
        deployment_type,
        creator_cosign_program_id,
        creator_fee_per_mint_in_lamports: creator_fee_in_lamports,
        creator_fee_treasury,
        deflation_rate_per_swap} = input;

    if deployment_type != TOKEN2022_DEPLOYMENT_TYPE && deployment_type != HYBRID_DEPLOYMENT_TYPE{
        panic!("Bad deployment type")
    }
    
    if deployment_type == HYBRID_DEPLOYMENT_TYPE && deflation_rate_per_swap > 0{
        panic!("Non-zero deflation rate requires a token-2022 deployment")
    }


    deployment_config.creator_fee_treasury = creator_fee_treasury;
    deployment_config.creator_fee_per_mint_lamports = creator_fee_in_lamports;
    deployment_config.deflation_rate_per_swap = deflation_rate_per_swap;

    initialise_logic(InitialiseInput {
        limit_per_mint, 
        max_number_of_tokens, decimals, ticker, deployment_template, mint_template, offchain_url, deployment_type
    }, deployment, creator.key(), Some(deployment_config))?;

    
    deployment.require_creator_cosign = creator_cosign_program_id.is_some();

    deployment_config.cosigner_program_id = match creator_cosign_program_id {
        Some(x) => x,
        _ => system_program::ID
    };


    deployment.use_inscriptions = use_inscriptions;

    Ok(())


}