use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, self}, token_2022,
};
use libreplex_shared::operations::transfer_non_pnft;

use crate::{Deployment, HashlistMarker, move_fungible_out_of_escrow};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct SwapLegacyToFungibleCtx<'info> {
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

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// this always exists so we can specify the account type explicitly
    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_source_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: derivation checked in Logic. May not exist so created as required.
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* non-fungible accounts */
    #[account(mut)]
    pub non_fungible_mint: Box<Account<'info, Mint>>,

    /// this always exists (otherwise we couldn't swap), so we can specify the account
    /// type explicitly
    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = payer,
    )]
    pub non_fungible_source_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: derivation checked in Logic. Will be created as needed
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

pub fn swap_metaplex_to_fungible(ctx: Context<SwapLegacyToFungibleCtx>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

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
    // 1) move the non_fungible into the escrow

    transfer_non_pnft(
        &token_program.to_account_info(),
        &non_fungible_source_token_account.to_account_info(),
        &non_fungible_target_token_account.to_account_info(),
        &payer.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &deployment.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        1,
    )?;

    let ticker = deployment.ticker.clone();

    let authority_seeds = &[
        "deployment".as_bytes(),
        ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // // 2) move the fungible_mint out of the escrow

    move_fungible_out_of_escrow(
        token_program,
        fungible_source_token_account,
        fungible_target_token_account,
        deployment,
        fungible_mint,
        payer,
        associated_token_program,
        system_program,
        authority_seeds,
        &ctx.accounts.hashlist_marker,
    )?;

    Ok(())
}
