use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;

use crate::{DeploymentV2, FungibleType, NonFungibleType};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Fungible {

}



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseRawInput {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub ticker: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub proxy_program_id: Option<Pubkey>,
    pub cosigner_mint: Option<Pubkey>,
    pub cosigner_swap_to_spl: Option<Pubkey>,
    pub cosigner_swap_to_nft: Option<Pubkey>,
    // these are for information only -
    // there are no restrictions on mint types when using
    // join or fungible types on initialise
    pub fungible_type: FungibleType,
    pub non_fungible_type: NonFungibleType
   
}

#[derive(Accounts)]
#[instruction(input: InitialiseRawInput)]
pub struct InitialiseRawCtx<'info>  {
    #[account(init, payer = payer, space = 8 + DeploymentV2::INIT_SPACE, 
        seeds = ["deployment".as_ref(), input.ticker.as_ref()], bump)]
    pub deployment: Account<'info, DeploymentV2>,

    #[account(
        constraint = fungible_mint.decimals > 0,
        constraint = fungible_mint.supply > 1
    )]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn initialise_raw(ctx: Context<InitialiseRawCtx>, input: InitialiseRawInput) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let creator = &ctx.accounts.creator;
    let fungible_mint = &ctx.accounts.fungible_mint;
    deployment.set_inner(DeploymentV2 { 
        creator: creator.key(), 
        deployed: false,
        limit_per_mint: input.limit_per_mint, 
        max_number_of_tokens: input.max_number_of_tokens, 
        number_of_tokens_issued: 0, 
        fungible_type: input.fungible_type,
        non_fungible_type: input.non_fungible_type,
        fungible_mint: fungible_mint.key(),
        escrow_non_fungible_count: 0, 
        ticker: input.ticker, 
        offchain_url: input.offchain_url, 
        fungible_decimals: fungible_mint.decimals,
        proxy_program_id: match &input.proxy_program_id {
            Some(x) => *x,
            _ => system_program::ID
        },
        cosigner_mint: match &input.cosigner_mint {
            Some(x) => *x,
            _ => system_program::ID
        },
        cosigner_swap_to_spl: match &input.cosigner_swap_to_spl {
            Some(x) => *x,
            _ => system_program::ID
        },
        cosigner_swap_to_nft: match &input.cosigner_swap_to_nft {
            Some(x) => *x,
            _ => system_program::ID
        }, 
        padding: [0; 200] 
    });
    
    Ok(())
}