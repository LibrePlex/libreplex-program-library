use anchor_lang::prelude::*;

use crate::{
    errors::FairLaunchError, Deployment, DeploymentConfig, NewDeploymentV2, OFFCHAIN_URL_LIMIT,
    TEMPLATE_LIMIT, TICKER_LIMIT,
};

pub struct InitialiseInput {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub decimals: u8,
    pub ticker: String,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub deployment_type: u8,
}

pub fn initialise_logic(
    input: InitialiseInput,
    deployment: &mut Account<'_, Deployment>,
    creator: Pubkey,
    config: Option<&DeploymentConfig>,
) -> Result<()> {
    if input.ticker.len() > TICKER_LIMIT {
        return Err(FairLaunchError::TickerTooLong.into());
    }
    if input.offchain_url.len() > OFFCHAIN_URL_LIMIT {
        return Err(FairLaunchError::OffchainUrlTooLong.into());
    }
    if input.mint_template.len() > TEMPLATE_LIMIT {
        return Err(FairLaunchError::MintTemplateTooLong.into());
    }
    if input.deployment_template.len() > TEMPLATE_LIMIT {
        return Err(FairLaunchError::DeploymentTemplateTooLong.into());
    }

    deployment.require_creator_cosign = false;
    deployment.use_inscriptions = true;
    deployment.deployment_type = input.deployment_type;
    deployment.creator = creator;
    deployment.limit_per_mint = input.limit_per_mint;
    deployment.max_number_of_tokens = input.max_number_of_tokens;
    deployment.number_of_tokens_issued = 0;
    deployment.decimals = input.decimals;
    deployment.ticker = input.ticker;
    deployment.deployment_template = input.deployment_template;
    deployment.mint_template = input.mint_template;
    deployment.offchain_url = input.offchain_url;
    deployment.escrow_non_fungible_count = 0;
    deployment.migrated_from_legacy = false;
    (input.limit_per_mint)
        .checked_mul(input.max_number_of_tokens)
        .unwrap()
        .checked_mul((10_u64).checked_pow(input.decimals as u32).unwrap())
        .unwrap();

    // Try avoid blowing up the stack
    emit_init(deployment, config);
    // for now, we limit ticker sizes to 12 bytes

    Ok(())
}

fn emit_init(deployment: &Deployment, config: Option<&DeploymentConfig>) {
    emit!(NewDeploymentV2 {
        creator: deployment.creator,
        limit_per_mint: deployment.limit_per_mint,
        max_number_of_tokens: deployment.max_number_of_tokens,
        ticker: deployment.ticker.clone(),
        off_chain_url: deployment.offchain_url.clone(),
        require_co_sign: deployment.require_creator_cosign,
        uses_inscriptions: deployment.use_inscriptions,
        decimals: deployment.decimals,
        deployment_template: deployment.deployment_template.clone(),
        mint_template: deployment.mint_template.clone(),
        deployment_type: deployment.deployment_type,
        config: config.cloned()
    });
}
