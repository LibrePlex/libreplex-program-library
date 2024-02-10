use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
};

use libreplex_shared::{
    create_mint_metadata_and_masteredition::create_mint_with_metadata_and_masteredition,
    MintAccounts, SharedError,
};
use mpl_token_metadata::types::TokenStandard;

use crate::{Deployment, Hashlist};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
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
pub struct DeployMigratedCtx<'info> {
    #[account(
        mut,
        // deployment must be executed by the payer 
        seeds=["deployment".as_bytes(), deployment.ticker.as_bytes()],
        bump
    )]
    pub deployment: Account<'info, Deployment>,

    #[account(init_if_needed, seeds = ["hashlist".as_bytes(), 
    deployment.key().as_ref()],
    bump, payer = payer, space = 8 + 32 + 4)]
    pub hashlist: Account<'info, Hashlist>,

    /*
       These keys are needed while the original hashlists are imported from old validator systems.
       Will be removed once migrations are complete as they really do not belong here.

       However
    */
    #[account(mut,
        constraint = payer.key().to_string() == *"4aAifU9ck88koMhSK6fnUSQHMzpyuLzGa6q7nfvqA6vx".to_owned())]
    pub payer: Signer<'info>,

    /* INITIALISE FUNGIBLE ACCOUNTS */
    #[account(mut)]
    pub fungible_mint: Signer<'info>,

    /// CHECK: checked in code
    #[account(mut)]
    pub fungible_escrow_token_account: UncheckedAccount<'info>,

    /// CHECK: gets created, passed into libreplex_fair_launch via  CPI
    #[account(mut)]
    pub fungible_metadata: UncheckedAccount<'info>,

    /*
        We do not need any non-fungible token stuff.
        Deployed a migrated deployment does not generate
        any new inscriptions or mints
     */

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

pub fn deploy_migrated(ctx: Context<DeployMigratedCtx>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let hashlist = &mut ctx.accounts.hashlist;


    check_deploy_allowed(deployment);

    hashlist.deployment = deployment.key();
    deployment.require_creator_cosign = false;
    deployment.use_inscriptions = true;

    let deployment = &mut ctx.accounts.deployment;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let fungible_metadata = &ctx.accounts.fungible_metadata;
    let sysvar_instructions = &ctx.accounts.sysvar_instructions;
    let token_program = &ctx.accounts.token_program;
    let metadata_program = &ctx.accounts.metadata_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;

    deployment.fungible_mint = fungible_mint.key();


    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &deployment.key(), &fungible_mint.key());

    if expected_token_account != fungible_escrow_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }   


  
    /* create the fungible mint and escrow token account */
    create_mint_with_metadata_and_masteredition(
        MintAccounts {
            authority_pda: deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: deployment.to_account_info(),
            nft_mint: fungible_mint.to_account_info(),
            nft_mint_authority: deployment.to_account_info(),
            nft_metadata: fungible_metadata.to_account_info(),
            nft_master_edition: None,
            token: Some(fungible_escrow_token_account.to_account_info()), // do not mint anything
            token_metadata_program: metadata_program.to_account_info(),
            spl_token_program: token_program.to_account_info(),
            spl_ata_program: associated_token_program.to_account_info(),
            system_program: system_program.to_account_info(),
            sysvar_instructions: sysvar_instructions.to_account_info(),
        },
        deployment_seeds,
        deployment.ticker.clone(),
        deployment.ticker.clone(),
        0,
        deployment.offchain_url.clone(),
        None,
        0,
        false,
        0,
        deployment.decimals,
        TokenStandard::Fungible,
    )?;

    // we make the escrow token account as well if needed

    if fungible_escrow_token_account.to_account_info().data_is_empty() {

        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: fungible_escrow_token_account.to_account_info(),
                authority: deployment.to_account_info(),
                mint: fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    } else {
        panic!("Fungible escrow token account not empty.")
    }


    Ok(())
}
