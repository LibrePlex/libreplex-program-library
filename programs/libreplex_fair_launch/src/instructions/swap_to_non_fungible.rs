use crate::{move_fungible_into_escrow, HashlistMarker};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, self}, token_2022,
};
use libreplex_shared::operations::transfer_non_pnft;
// use libreplex_shared::operations::transfer_non_pnft;

use crate::Deployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct SwapFungibleToLegacyCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /* fungible accounts */
    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = payer, // coimes out of the payer account
    )]
    pub fungible_source_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checked in transfer logic
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* NON-FUNGIBLE COMES OUT OF THE ESCROW */
    #[account(mut)]
    pub non_fungible_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = deployment, // escrow is always owned by the deployment
    )]
    pub non_fungible_source_token_account: Account<'info, TokenAccount>,

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// CHECK: Checked in transfer logic
    #[account(mut)]
    pub non_fungible_target_token_account: UncheckedAccount<'info>,

     /// CHECK: Checked in constraint
     #[account(
        constraint = token_program.key() == token_2022::ID || token_program.key() == token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,
}

pub fn swap_to_nonfungible(ctx: Context<SwapFungibleToLegacyCtx>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;


    if deployment.require_creator_cosign {
        panic!("Only launches without creator cosign can currently use v1 methods")
    }

    // simples. two steps:
    // 1) move the fungible into the escrow

    move_fungible_into_escrow(
        token_program,
        fungible_source_token_account,
        fungible_target_token_account,
        source_wallet,
        fungible_mint,
        deployment,
        associated_token_program,
        system_program,
        payer,
        &ctx.accounts.hashlist_marker,
    )?;

    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // 2) move the non_fungible_mint out of the escrow

    transfer_non_pnft(
        &token_program.to_account_info(),
        &non_fungible_source_token_account.to_account_info(),
        &non_fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &source_wallet.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]), // payer signs
        &payer.to_account_info(),
        1,
    )?;

    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
