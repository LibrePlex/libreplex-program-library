use anchor_lang::{prelude::*, system_program};
use crate::{InitialiseInputV3, TOKEN2022_DEPLOYMENT_TYPE, HYBRID_DEPLOYMENT_TYPE};

use crate::{
    errors::FairLaunchError, Deployment, DeploymentConfig, NewDeploymentV2, OFFCHAIN_URL_LIMIT,
    TEMPLATE_LIMIT, TICKER_LIMIT,
};



pub fn initialise_logic(input: InitialiseInputV3, 
    deployment: &mut Account<'_, Deployment>, 
    creator: Pubkey, config: &mut DeploymentConfig) -> Result<()> {
    let deployment_type = input.deployment_type;

    if deployment_type != TOKEN2022_DEPLOYMENT_TYPE && deployment_type != HYBRID_DEPLOYMENT_TYPE{
        panic!("Bad deployment type")
    }
    
    if deployment_type == HYBRID_DEPLOYMENT_TYPE && input.transfer_fee_config.is_some(){
        panic!("Non-zero deflation rate requires a token-2022 deployment")
    }


    config.creator_fee_treasury = input.creator_fee_treasury;
    config.creator_fee_per_mint_lamports = input.creator_fee_per_mint_in_lamports;
    config.allow_burn = true;
    // NB: These impact the amount of SPL that is available to burn 
    // at the end of mint (in case there is excess in a multi-tier rarity set-up)
    config.spl_excess_in_escrow = 0;
    config.total_spl_equivalent_minted = 0;

    if let Some(x) = input.transfer_fee_config {
        config.transfer_fee_in_basis_points = x.fee_in_basis_points;
        config.transfer_fee_withdraw_authority = Some(x.withdraw_authority);
        config.transfer_fee_target_wallet = Some(x.target_wallet);
        config.allow_claim_transfer_fee_auth_as_creator = x.allow_claim_transfer_fee_auth_as_creator;
    } else {
        config.transfer_fee_in_basis_points = 0;
        config.transfer_fee_withdraw_authority = None;
        config.transfer_fee_target_wallet = None;
        config.allow_claim_transfer_fee_auth_as_creator = false;
    }
   
    config.multiplier_limits = Some(input.multiplier_limits);

    if let Some(limits) = config.multiplier_limits.as_ref() {
        if limits.min_denominator == 0 || limits.max_numerator == 0 {
            panic!("Invalid multiplier limits");
        }
    }
        
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


    deployment.require_creator_cosign = input.creator_cosign_program_id.is_some();

    config.cosigner_program_id = match input.creator_cosign_program_id {
        Some(x) => x,
        _ => system_program::ID
    };


    deployment.use_inscriptions = input.use_inscriptions;

    // Try avoid blowing up the stack
    emit_init(deployment, config);
    // for now, we limit ticker sizes to 12 bytes

    Ok(())
}

fn emit_init(deployment: &Deployment, config: &DeploymentConfig) {
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
        config: Some(config.clone())
    });
}
