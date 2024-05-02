use anchor_lang::prelude::*;

use crate::{initialise_logic, Deployment, DeploymentConfig, MultiplierLimits};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TransferFeeInputConfig {
    pub fee_in_basis_points: u16,
    pub withdraw_authority: Pubkey,
    pub target_wallet: Pubkey,
    pub allow_claim_transfer_fee_auth_as_creator: bool
}

// Same as v2 with multiplier_upper_limit added
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInputV3 {
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
    
    // The largest possible multiplier
    pub multiplier_limits: MultiplierLimits,
    pub transfer_fee_config: Option<TransferFeeInputConfig>
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

// Same as V2 just with extra field in input.
#[derive(Accounts)]
#[instruction(input: InitialiseInputV3)]
pub struct InitialiseV3Ctx<'info>  {
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

pub fn initialise_v3(ctx: Context<InitialiseV3Ctx>, input: InitialiseInputV3) -> Result<()> {
    let deployment: &mut Account<'_, Deployment> = &mut ctx.accounts.deployment;
    let deployment_config = &mut ctx.accounts.deployment_config;
    let creator = &ctx.accounts.creator;

    initialise_logic(input, 
        deployment, 
        creator.key(), 
        deployment_config)?;
  
    Ok(())
}