use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use libreplex_shared::operations::transfer_non_pnft;

use crate::Deployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

/*

    Swaps a non
*/

#[derive(Accounts)]
pub struct SwapToFungibleCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /* fungible accounts */
    #[account(mut)]
    pub fungible_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_token_account_escrow: Box<Account<'info, TokenAccount>>,

    /// CHECK: derivation checked in Logic
    #[account(
        mut
    )]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* non-fungible accounts */
    #[account(mut)]
    pub non_fungible_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = payer,
    )]
    pub non_fungible_source_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: derivation checked in Logic
    #[account(mut
    )]
    pub non_fungible_target_token_account: UncheckedAccount<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

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

pub fn swap_to_fungible(ctx: Context<SwapToFungibleCtx>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_token_account_escrow;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    // simples. two steps:
    // 1) move the non_fungible into the escrow

    transfer_non_pnft(
        &token_program.to_account_info(),
        &non_fungible_source_token_account.to_account_info(),
        &non_fungible_target_token_account.to_account_info(),
        &source_wallet.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &deployment.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        1,
    )?;

    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // // 2) move the fungible_mint out of the escrow
    transfer_non_pnft(
        &token_program.to_account_info(),
        &fungible_source_token_account.to_account_info(),
        &fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &fungible_mint.to_account_info(),
        &source_wallet.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]),
        &payer.to_account_info(),
        deployment.limit_per_mint,
    )?;


    deployment.escrow_non_fungible_count += 1;
    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
