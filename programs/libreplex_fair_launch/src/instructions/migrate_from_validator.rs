use anchor_lang::prelude::*;



use crate::{Deployment, TICKER_LIMIT, errors::FairLaunchError, OFFCHAIN_URL_LIMIT, TEMPLATE_LIMIT, InitialiseInput};


pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
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
#[instruction(input: InitialiseInput, validated_token_count: u64)]
pub struct MigrateFromValidatorCtx<'info>  {
    #[account(init, payer = payer, space = 8 + Deployment::INIT_SPACE, 
        seeds = ["deployment".as_ref(), input.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        constraint = payer.key().to_string() == *"11111111111111111111111111111111".to_owned())]
    pub payer: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,


}

pub fn migrate_from_validator(ctx: Context<MigrateFromValidatorCtx>, input: InitialiseInput, validated_token_count: u64) -> Result<()> {
    

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


    let deployment = &mut ctx.accounts.deployment;
    let payer = &ctx.accounts.payer;
    // create 
    deployment.creator = payer.key();
    deployment.limit_per_mint = input.limit_per_mint;
    deployment.max_number_of_tokens = validated_token_count;
    deployment.number_of_tokens_issued = validated_token_count;
    deployment.decimals = input.decimals;
    deployment.ticker = input.ticker;
    deployment.deployment_template = "".to_owned();
    deployment.mint_template = input.mint_template;
    deployment.offchain_url = input.offchain_url;
    deployment.escrow_non_fungible_count = 0;
    deployment.migrated_from_legacy = true;
    deployment.deployed = true;

    Ok(())
}