use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};



use crate::{deploy_legacy_inscriptions, deploy_legacy_logic, Deployment, Hashlist};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct DeployV2Input {
    pub require_creator_cosign: bool,
    pub use_inscriptions: bool
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
pub struct DeployLegacyV2Ctx<'info> {
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

    /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub fungible_metadata: UncheckedAccount<'info>,

    /* INITIALISE NON_FUNGIBLE ACCOUNTS. NB: no token account neede until mint */
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub non_fungible_metadata: UncheckedAccount<'info>,

    /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub non_fungible_master_edition: UncheckedAccount<'info>,

    /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,

    /* INTERACT WITH INSCRIPTIONS PROGRAM  */
     /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: ID checked via constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = metadata_program.key() == mpl_token_metadata::ID
    )]
    #[account()]
    pub metadata_program: UncheckedAccount<'info>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    #[account()]
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn deploy_v2(ctx: Context<DeployLegacyV2Ctx>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;

    if !deployment.require_creator_cosign {
        panic!("Only creator cosign can currently use v2 methods")
    }

    let hashlist = &mut ctx.accounts.hashlist;

    hashlist.deployment = deployment.key();

    let deployment = &mut ctx.accounts.deployment;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &ctx.accounts.inscription_summary;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let fungible_metadata = &ctx.accounts.fungible_metadata;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_metadata = &ctx.accounts.non_fungible_metadata;
    let non_fungible_master_edition = &ctx.accounts.non_fungible_master_edition;
    let non_fungible_token_account = &ctx.accounts.non_fungible_token_account;
    let sysvar_instructions = &ctx.accounts.sysvar_instructions;
    let token_program = &ctx.accounts.token_program;
    let metadata_program = &ctx.accounts.metadata_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;

    deploy_legacy_logic(
        hashlist,
        deployment,
        fungible_mint,
        payer,
        fungible_metadata,
        fungible_escrow_token_account,
        metadata_program,
        token_program,
        associated_token_program,
        system_program,
        sysvar_instructions,
        non_fungible_mint,
        non_fungible_metadata,
        non_fungible_master_edition,
        non_fungible_token_account,
        ctx.bumps.deployment,
    )?;

    if deployment.use_inscriptions {
        deploy_legacy_inscriptions(
            inscriptions_program,
            inscription_summary,
            non_fungible_mint,
            inscription_v3,
            system_program,
            payer,
            inscription_data,
            deployment,
        )?;
    }

    Ok(())
}
