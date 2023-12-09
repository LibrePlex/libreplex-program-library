use anchor_lang::prelude::*;



use crate::{Deployment, errors::FairLaunchError, OFFCHAIN_URL_LIMIT, TEMPLATE_LIMIT, NewDeploymentEvent};


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
#[instruction(input: InitialiseInput)]
pub struct InitialiseCtx<'info>  {
    #[account(init, payer = payer, space = 8 + Deployment::INIT_SPACE, 
        seeds = ["deployment".as_ref(), input.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut
        // ,
        // constraint = payer.key().to_string() == "11111111111111111111111111111111".to_owned()
    )]
    pub payer: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,


}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub decimals: u8,
    pub ticker: String,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
}

pub fn initialise(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
    

    // for now, we limit ticker sizes to 12 bytes 
    if input.ticker.len() > 12 {
        return Err(FairLaunchError::TickerTooLong.into());
    }

    if input.offchain_url.len() > OFFCHAIN_URL_LIMIT {
        return Err(FairLaunchError::TickerTooLong.into());
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
    deployment.max_number_of_tokens = input.max_number_of_tokens;
    deployment.number_of_tokens_issued = 0;
    deployment.decimals = input.decimals;
    deployment.ticker = input.ticker;
    deployment.deployment_template = input.deployment_template;
    deployment.mint_template = input.mint_template;
    deployment.offchain_url = input.offchain_url;
    deployment.escrow_non_fungible_count = 0;

    deployment.migrated_from_legacy = false;


    // test that total amount of mints fits within u64
    (input.limit_per_mint).checked_mul(input.max_number_of_tokens).unwrap().checked_mul(
        (10_u64).checked_pow(input.decimals as u32).unwrap()).unwrap();
    


    emit!(NewDeploymentEvent {
        creator: deployment.creator,
        limit_per_mint: deployment.limit_per_mint,
        max_number_of_tokens: deployment.max_number_of_tokens,
        ticker: deployment.ticker.clone(),
    });

    Ok(())
}