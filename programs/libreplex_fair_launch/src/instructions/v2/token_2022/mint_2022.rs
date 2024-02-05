

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022, token_interface::Mint
};


use libreplex_shared::SharedError;


use crate::{
    errors::FairLaunchError, Deployment, HashlistMarker, TOKEN2022_DEPLOYMENT_TYPE, HYBRID_DEPLOYMENT_TYPE, 
    mint_token2022_logic, DeploymentConfig,
};

#[derive(Accounts)]
pub struct MintToken2022Ctx<'info> {
    #[account(mut,
       

        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

    /// CHECK: checked in constraint
    #[account(mut,
        constraint = deployment_config.creator_fee_treasury == creator_fee_treasury.key())] 
    pub creator_fee_treasury: UncheckedAccount<'info>,



    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut, 
        
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = 8,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub fungible_mint: InterfaceAccount<'info, Mint>,


    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub minter: UncheckedAccount<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,
    



    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,




    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn mint_token2022<'info>(ctx: Context<'_, '_, '_, 'info, MintToken2022Ctx<'info>>) -> Result<()> {
    // let MintToken2022Ctx { 
      
    //     ..
    // } = &ctx.accounts;

    let payer = &ctx.accounts.payer; 
    let signer = &ctx.accounts.signer;
    let minter= &ctx.accounts.minter;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_token_account = &ctx.accounts.non_fungible_token_account;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let fungible_mint = &ctx.accounts.fungible_mint;

    // mutable borrows
    let deployment = &mut ctx.accounts.deployment;
    let deployment_config = &mut ctx.accounts.deployment_config;
    let creator_fee_treasury = &mut ctx.accounts.creator_fee_treasury;
    let hashlist = &mut ctx.accounts.hashlist;

    if !deployment.deployment_type.eq(&TOKEN2022_DEPLOYMENT_TYPE) && !deployment.deployment_type.eq(&HYBRID_DEPLOYMENT_TYPE){
        return Err(FairLaunchError::IncorrectMintType.into())
    }
  
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        return Err(FairLaunchError::MintedOut.into());
    }

    if deployment.migrated_from_legacy {
        return Err(FairLaunchError::LegacyMigrationsAreMintedOut.into());
    }



    if deployment.require_creator_cosign && !signer.key().eq(&deployment.creator.key()) {
        return Err(SharedError::InvalidCreatorCosigner.into());
    }

    mint_token2022_logic(
        deployment, 
        deployment_config,
        creator_fee_treasury,
        &fungible_mint.to_account_info(),
        non_fungible_mint, 
        system_program, 
        payer, 
        associated_token_program, 
        token_program, 
        minter, 
        non_fungible_token_account, 
        hashlist,
    ctx.bumps.deployment,
ctx.remaining_accounts)?;

    
    Ok(())
}
