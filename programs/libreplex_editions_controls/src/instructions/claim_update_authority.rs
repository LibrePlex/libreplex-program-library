use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token_2022;
use libreplex_editions::cpi::accounts::ClaimUpdateAuthorityCtx as CpiClaimUpdateAuthorityCtx;
use libreplex_editions::program::LibreplexEditions;
use crate::EditionsControls;
use crate::errors::EditionsError;
use spl_token_metadata_interface::instruction::update_authority;
use spl_pod::optional_keys::OptionalNonZeroPubkey; 
#[derive(Accounts)]
pub struct ClaimUpdateAuthorityCtx<'info> {

    #[account(mut,
        seeds = [
            b"editions_controls", editions_deployment.key().as_ref()
            ], bump)]
    pub editions_controls: Account<'info, EditionsControls>,

    #[account(mut)]
    pub editions_deployment: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub mint: AccountInfo<'info>,
    
    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    pub libreplex_editions_program: Program<'info, LibreplexEditions>

}

pub fn claim_update_authority<'info>(ctx: Context<'_, '_, '_, 'info, ClaimUpdateAuthorityCtx<'info>>) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let editions_controls = &ctx.accounts.editions_controls;
    let editions_deployment = &ctx.accounts.editions_deployment;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let libreplex_editions_program = &ctx.accounts.libreplex_editions_program;
    let creator = &ctx.accounts.creator;

    require!(editions_controls.creator.key() == creator.key(), EditionsError::InvalidCreator);
    let editions_deployment_key = editions_deployment.key();

    let seeds = &[
        b"editions_controls",
        editions_deployment_key.as_ref(),
        &[ctx.bumps.editions_controls],
    ];

    libreplex_editions::cpi::claim_update_authority(
        CpiContext::new_with_signer(
            libreplex_editions_program.to_account_info(),
            CpiClaimUpdateAuthorityCtx {
                editions_deployment: editions_deployment.to_account_info(),
                mint: mint.to_account_info(),
                payer: payer.to_account_info(),
                creator: editions_controls.to_account_info(),
                token_program: token_program.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[seeds]
        ))?;

    let account_infos = [
        editions_controls.to_account_info(),
        mint.to_account_info(), 
        creator.to_account_info(),
        token_program.to_account_info(),
    ];
    
    let creator_key: OptionalNonZeroPubkey = OptionalNonZeroPubkey::try_from(Some(creator.to_account_info().key()))?;
    
    let update_authority_ix = update_authority(
        &spl_token_2022::ID,
        &mint.key(),
        &editions_controls.key(),
        creator_key
    );

    invoke_signed(&update_authority_ix, &account_infos, &[seeds])?;

    Ok(())
}