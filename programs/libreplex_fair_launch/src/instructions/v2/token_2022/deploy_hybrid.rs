use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{spl_token, Mint, TokenAccount}};
use solana_program::system_program;



use crate::{
    deploy_hybrid_logic, Deployment, DeploymentActive, DeploymentConfig, Hashlist, HYBRID_DEPLOYMENT_TYPE
};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct DeployV2Input {
    pub require_creator_cosign: bool,
    pub use_inscriptions: bool,
}

/*

    Deploy takes no input parameters as all of the
    string parameter + decimals have already been set by
    initialise.

    Deploy creates all on-chain objects (inscriptions,
    mints + any metadata) that are required to keep track of the
    launch lifecycle.
*/
#[derive(Accounts)]
pub struct DeployHybridCtx<'info> {
    #[account(
        mut,
        seeds=["deployment".as_bytes(), deployment.ticker.as_bytes()],
        bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump
    )]
    pub deployment_config: Box<Account<'info, DeploymentConfig>>,

    #[account(init_if_needed, seeds = ["hashlist".as_bytes(), 
    deployment.key().as_ref()],
    bump, payer = payer, space = 8 + 32 + 4)]
    pub hashlist: Box<Account<'info, Hashlist>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // this signer is no longer used but retained for backwards compatibility.
    // reason being, the deploy call can be permissionless - it conveys no 
    // special privileges to the creator that initialised the mint.
    #[account(mut)]
    pub creator: Signer<'info>,

    /* INITIALISE FUNGIBLE ACCOUNTS */
    #[account(init, payer = payer, 
        mint::freeze_authority = deployment,
        mint::authority = deployment, 
        mint::decimals = deployment.decimals)]
    pub fungible_mint: Box<Account<'info, Mint>>,
    
    /// CHECK: Passed via CPI
    #[account(mut)]
    pub fungible_master_edition: UncheckedAccount<'info>,

    /// CHECK: Passed via CPI
    #[account(mut)]
    pub fungible_metadata: UncheckedAccount<'info>,

    /// CHECK: checked in code
    #[account(init, 
        associated_token::mint = fungible_mint, 
        payer = payer, associated_token::authority = deployment)]
    pub fungible_escrow_token_account: Box<Account<'info, TokenAccount>>,

    /* INITIALISE NON_FUNGIBLE ACCOUNTS. NB: no token account neede until mint */
    // #[account(mut)]
    // pub non_fungible_mint: Signer<'info>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(
        constraint = token_program.key() == spl_token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

    #[account()]
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    #[account()]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// CHECK: address checked
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
}

pub fn deploy_hybrid(ctx: Context<DeployHybridCtx>) -> Result<()> {
    let hashlist = &mut ctx.accounts.hashlist;
    let deployment = &mut ctx.accounts.deployment;

    let payer = &ctx.accounts.payer;
    let fungible_mint= &ctx.accounts.fungible_mint;
    let fungible_metadata= &ctx.accounts.fungible_metadata;
    let fungible_master_edition = &ctx.accounts.fungible_master_edition;    
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
    // let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let rent = &ctx.accounts.rent;
    let metadata_program = &ctx.accounts.metadata_program;
    

    check_deploy_allowed(deployment);

    let sysvar_instructions = &ctx.accounts.sysvar_instructions;
    if deployment.deployment_type != HYBRID_DEPLOYMENT_TYPE {
        panic!("Wrong deployment type")
    }

    deploy_hybrid_logic(
        hashlist,
        deployment,
        fungible_mint.as_ref().as_ref(),
        fungible_metadata,
        fungible_master_edition,
        payer,
        fungible_escrow_token_account.as_ref().as_ref(),
        token_program,
        associated_token_program,
        system_program,
        // non_fungible_mint,
        rent,
        sysvar_instructions,
        metadata_program,
        ctx.bumps.deployment,
        &ctx.accounts.deployment_config
    )?;

    Ok(())
}


pub fn check_deploy_allowed (deployment: &Account<Deployment>){

    // only allow redeploys if forced by a wrapping program and there are no mints
    if deployment.fungible_mint != system_program::ID {
        // already deployed
        if deployment.require_creator_cosign {
            // ok we may be able to force a redeploy
            if deployment.number_of_tokens_issued > 0 {
                panic!("Cannot force redeploy after tokens have been issued")
            }
        } else {
            panic!("Cannot redeploy without creator cosign")
        }   
    }

    // fungible mint = system program id, this has not been deployed yet at all
}


// Raw deployments do not enforce as many restrictions on the mint
#[derive(Accounts)]
pub struct DeployHybridUncheckedCtx<'info> {
    #[account(
        mut,
        seeds=["deployment".as_bytes(), deployment.ticker.as_bytes()],
        bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(
        mut,
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump
    )]
    pub deployment_config: Box<Account<'info, DeploymentConfig>>,

    #[account(init_if_needed, seeds = ["hashlist".as_bytes(), 
    deployment.key().as_ref()],
    bump, payer = payer, space = 8 + 32 + 4)]
    pub hashlist: Box<Account<'info, Hashlist>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // this signer is no longer used but retained for backwards compatibility.
    // reason being, the deploy call can be permissionless - it conveys no 
    // special privileges to the creator that initialised the mint.
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mint::decimals = deployment.decimals)]
    pub fungible_mint: Box<Account<'info, Mint>>,
    

    /// CHECK: checked in code
    #[account(init, 
        associated_token::mint = fungible_mint, 
        payer = payer, associated_token::authority = deployment)]
    pub fungible_escrow_token_account: Box<Account<'info, TokenAccount>>,

    /* INITIALISE NON_FUNGIBLE ACCOUNTS. NB: no token account neede until mint */
    // #[account(mut)]
    // pub non_fungible_mint: Signer<'info>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(
        constraint = token_program.key() == spl_token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn deploy_hybrid_unchecked_handler(ctx: Context<DeployHybridUncheckedCtx>) -> Result<()> {
    let deployment = ctx.accounts.deployment.as_mut();
    check_deploy_allowed(deployment);

    if deployment.deployment_type != HYBRID_DEPLOYMENT_TYPE {
        panic!("Wrong deployment type")
    }

    let hashlist = ctx.accounts.hashlist.as_mut();
    let fungible_mint = ctx.accounts.fungible_mint.as_ref();

    hashlist.deployment = deployment.key();
    deployment.fungible_mint = fungible_mint.key();

    ctx.accounts.deployment_config.unchecked_fungible = true;

    emit!(DeploymentActive { 
        ticker: deployment.ticker.clone(),
        fungible_mint: fungible_mint.key(),
    });

    Ok(())
}