use anchor_lang::prelude::*;



use crate::{Deployment, initialise_logic, InitialiseInput};



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
    pub use_inscriptions: bool
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

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}



pub fn initialise_v2(ctx: Context<InitialiseV2Ctx>, input: InitialiseInputV2) -> Result<()> {
    
    let deployment = &mut ctx.accounts.deployment;
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
        use_inscriptions} = input;

    deployment.require_creator_cosign = require_creator_cosign;
    deployment.use_inscriptions = use_inscriptions;

    if !deployment.require_creator_cosign  {
        // temporary message until we're confident that this works for trad deployments as well (without require_creator_cosign)
        panic!("Only called with require_creator_cosign")
    }

    initialise_logic(InitialiseInput {
        limit_per_mint, max_number_of_tokens, decimals, ticker, deployment_template, mint_template, offchain_url
    }, deployment, creator.key())

}