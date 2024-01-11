use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;



use crate::{
    deploy_token_2022_logic, Deployment, Hashlist,
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
pub struct DeployToken2022Ctx<'info> {
    #[account(
        mut,
        // deployment must be executed by the payer 
        constraint = deployment.creator == payer.key(),
        seeds=["deployment".as_bytes(), deployment.ticker.as_bytes()],
        bump
    )]
    pub deployment: Account<'info, Deployment>,

  

    #[account(init, seeds = ["hashlist".as_bytes(), 
    deployment.key().as_ref()],
    bump, payer = payer, space = 8 + 32 + 4)]
    pub hashlist: Account<'info, Hashlist>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // can be different from the payer. typically used with PDA
    #[account(mut)]
    pub creator: Signer<'info>,

    /* INITIALISE FUNGIBLE ACCOUNTS */
    #[account(mut)]
    pub fungible_mint: Signer<'info>,

    /// CHECK: checked in code
    #[account(mut)]
    pub fungible_escrow_token_account: UncheckedAccount<'info>,

    /* INITIALISE NON_FUNGIBLE ACCOUNTS. NB: no token account neede until mint */
    // #[account(mut)]
    // pub non_fungible_mint: Signer<'info>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(
        constraint = token_program.key() == spl_token_2022::ID
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
}

pub fn deploy_token_2022(ctx: Context<DeployToken2022Ctx>) -> Result<()> {

    let hashlist = &mut ctx.accounts.hashlist;
    let deployment = &mut ctx.accounts.deployment;

    let payer = &ctx.accounts.payer;
    let fungible_mint= &ctx.accounts.fungible_mint;
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
    // let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    
    // msg!("Set fungible mint to {}", fungible_mint.key());
    // deployment.fungible_mint = fungible_mint.key();
   


    deploy_token_2022_logic(
        hashlist,
        deployment,
        fungible_mint,
        payer,
        fungible_escrow_token_account,
        token_program,
        associated_token_program,
        system_program,
        // non_fungible_mint,
        ctx.bumps.deployment,
    )?;

    Ok(())
}
