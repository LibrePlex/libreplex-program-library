use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use libreplex_shared::operations::transfer_non_pnft;

use crate::TokenDeployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

/*

    Swaps a non
*/

#[derive(Accounts)]
pub struct SwapToNonFungibleCtx<'info> {
    #[account(
        constraint = deployment.collection_mint == non_fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Account<'info, TokenDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /* fungible accounts */
    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    /* FUNGIBLE GOES INTO THE ESCROW */
    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment, // escrow is always owned by deployment
    )]
    pub fungible_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = payer, // coimes out of the payer account
    )]
    pub fungible_source_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = fungible_mint,
        token::authority = deployment, // and into the escrow
    )]
    pub fungible_target_token_account: Account<'info, TokenAccount>,

    /* NON-FUNGIBLE COMES OUT OF THE ESCROW */
    #[account(mut)]
    pub non_fungible_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = fungible_mint,
        token::authority = deployment, // escrow is always owned by the deployment
    )]
    pub non_fungible_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = deployment, // comes out of the escrow account
    )]
    pub non_fungible_source_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = fungible_mint,
        token::authority = payer, // and to the payer
    )]
    pub non_fungible_target_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Passed in via CPI
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

   /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,
}

pub fn swap_to_nonfungible(ctx: Context<SwapToNonFungibleCtx>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    // simples. two steps:
    // 1) move the non_fungible out of the escrow

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

    let deployment_key = deployment.key();
    let authority_seeds = &[
        deployment_key.as_ref(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // 2) move the fungible_mint into the escrow
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

    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
