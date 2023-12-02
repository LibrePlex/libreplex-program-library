use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::HashlistMarker;
use libreplex_shared::{operations::transfer_non_pnft, SharedError};
// use libreplex_shared::operations::transfer_non_pnft;

use crate::Deployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct SwapToNonFungibleCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == non_fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Account<'info, Deployment>,

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

    /// CHECK: Checked in transfer logic
    #[account(
       mut
    )]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* NON-FUNGIBLE COMES OUT OF THE ESCROW */
    #[account(mut)]
    pub non_fungible_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment, // escrow is always owned by the deployment
    )]
    pub non_fungible_source_acount_escrow: Account<'info, TokenAccount>,


    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(mut, 
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// CHECK: Checked in transfer logic
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
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_acount_escrow;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    if !deployment.allow_spl_conversion {
        return Err(SharedError::SplConversionNotAllowed.into());
    }


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
        deployment.get_fungible_mint_amount(),
    )?;


    // mark one of the non fungibles as moving out of the contract
    deployment.escrow_non_fungible_count -= 1;

    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
