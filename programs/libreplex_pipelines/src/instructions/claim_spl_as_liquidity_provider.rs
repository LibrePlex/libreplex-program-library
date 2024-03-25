use crate::Pipeline;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount}};
use libreplex_fair_launch::Deployment;
use libreplex_liquidity::Liquidity;
use libreplex_shared::operations::transfer_generic_spl;

#[derive(Accounts)]
pub struct ClaimSplAsLiquidityProviderCtx<'info> {
    #[account(constraint = pipeline.liquidity == liquidity.key())]
    pub pipeline: Account<'info, Pipeline>,

    /// CHECK: Checked via CPI
    #[account(constraint = liquidity.deployment == deployment.key())]
    pub liquidity: Account<'info, Liquidity>,

    /// CHECK: Checked via CPI
    #[account()]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut,
        token::authority = liquidity_provider_escrow,
        token::mint = fungible_mint)]
    pub liquidity_provider_escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = deployment.fungible_mint == fungible_mint.key()
    )]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Checked in logic.
    #[account(seeds=[b"liq_provider_escrow",pipeline.key().as_ref(), recipient.key().as_ref()], bump)]
    pub liquidity_provider_escrow: UncheckedAccount<'info>,

    /// CHECK: Derivation checked in logic 
    #[account(mut)]
    pub recipient_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in PDA for liquidity_provider_escrow 
    pub recipient: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

      /// CHECK: Only check address
    #[account(
        constraint = token_program.key().eq(fungible_mint.to_account_info().owner))]
    pub token_program: AccountInfo<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn claim_spl_as_liquidity_provider(
    ctx: Context<ClaimSplAsLiquidityProviderCtx>
) -> Result<()> {

    
    let fungible_mint = &ctx.accounts.fungible_mint;
    let token_program = &ctx.accounts.token_program;
    let liquidity_provider_escrow_token_account = &ctx.accounts.liquidity_provider_escrow_token_account;
    let liquidity_provider_escrow = &ctx.accounts.liquidity_provider_escrow;
    let recipient_token_account = &ctx.accounts.recipient_token_account;
    let recipient = &ctx.accounts.recipient;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let liquidity = &ctx.accounts.liquidity;
    let pipeline_key = &ctx.accounts.pipeline.key();
    let liquidity_provider_key = &ctx.accounts.recipient.key();

    if !liquidity.pool_bootstrapped {
        panic!("Cannot claim liquidity before pool has been bootstrapped.")
    }


    let liquidity_provider_escrow_seeds: &[&[u8]] = &[
        "liq_provider_escrow".as_bytes(),
        pipeline_key.as_ref(), liquidity_provider_key.as_ref(),
        &[ctx.bumps.liquidity_provider_escrow],
    ];

    transfer_generic_spl(
        &token_program.to_account_info(),
        &liquidity_provider_escrow_token_account.to_account_info(),
        &recipient_token_account.to_account_info(),
        &liquidity_provider_escrow.to_account_info(),
        &fungible_mint.to_account_info(),
        &recipient.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[liquidity_provider_escrow_seeds]),
        payer,
        fungible_mint.decimals,
        liquidity_provider_escrow_token_account.amount,
        &[]
    )?;

    Ok(())
}
