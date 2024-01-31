use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use libreplex_fair_launch::Deployment;
use vault_proxy::program::Vault;

use crate::Liquidity;


#[derive(Accounts)]
pub struct BootstapPool<'info> {
    #[account(has_one = deployment, has_one = fungible_mint)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub deployment: Box<Account<'info, Deployment>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    #[account(associated_token::mint = fungible_mint, 
        associated_token::authority = liquidity)]
    pub fungible_escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = anchor_spl::token::spl_token::native_mint::id())]
    pub wrapped_sol_mint: Box<Account<'info, Mint>>,


    #[account(
        associated_token::mint = wrapped_sol_mint, 
        associated_token::authority = liquidity
    )]
    pub wrapped_sol_escrow: Box<Account<'info, TokenAccount>>,

  /// CHECK: I maybe should check, but checked in coi
  #[account(mut)]
  pub pool: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi
  #[account(mut)]
  pub wrapped_sol_vault: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi
  #[account(mut)]
  pub fungible_vault: UncheckedAccount<'info>,

  /// CHECK: Wrapped Sol Token account of the vault
  #[account(mut)]
  pub wrapped_sol_token_vault: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi, token account of the fungible vault
  #[account(mut)]
  pub fungible_token_vault: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi, the lp mint of the wrapped sol vault
  #[account(mut)]
  pub wrapped_sol_vault_lp_mint: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi
  #[account(mut)]
  pub fungible_vault_lp_mint: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi, a token account
  #[account(mut)]
  pub wrapped_sol_vault_lp: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi
  #[account(mut)]
  pub fungible_vault_lp: UncheckedAccount<'info>,

  /// CHECK: Checked in cpi
  #[account(mut)]
  pub lp_mint: Box<Account<'info, Mint>>,

  #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = lp_mint,
      associated_token::authority = liquidity,
  )]
  pub pool_lp_token_account: Box<Account<'info, TokenAccount>>,


    pub amm_program: Program<'info, amm_proxy::program::Amm>,

    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Program<'info, Vault>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>
}


#[derive(Accounts)]
pub struct PrepareNativeEscrow<'info> {
    #[account(mut)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, address = anchor_spl::token::spl_token::native_mint::id())]
    pub wrapped_sol_mint: Box<Account<'info, Mint>>,

    #[account(init_if_needed, payer = payer, associated_token::mint = wrapped_sol_mint, 
        associated_token::authority = liquidity)]
    pub escrow_wrapped_sol_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn prepare_native_escrow_handler(ctx: Context<PrepareNativeEscrow>) -> Result<()> {
    let liquidity: &AccountInfo = ctx.accounts.liquidity.as_ref().as_ref();
    let native_escrow: &AccountInfo = ctx.accounts.escrow_wrapped_sol_account.as_ref().as_ref();

    let rent = Rent::get()?;
    let excess_sol = liquidity.to_account_info().lamports().saturating_sub(rent.minimum_balance(liquidity.to_account_info().data_len()));

    if excess_sol > 0 {
        **(liquidity.try_borrow_mut_lamports()?) -= excess_sol;
        **(native_escrow.try_borrow_mut_lamports()?) += excess_sol;
    }

    Ok(())
}


pub fn bootstrap_pool_handler(ctx: Context<BootstapPool>) -> Result<()> {
    let rent = Rent::get()?;
    let liquidity = &mut ctx.accounts.liquidity;
    let excess_sol = liquidity.to_account_info().lamports().saturating_sub(rent.minimum_balance(liquidity.to_account_info().data_len()));
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
    
    if liquidity.pool_bootstrapped {
        return Ok(())
    }

    liquidity.pool_bootstrapped = true;

    if excess_sol > 0 {
        panic!("Call prepare native escrow before bootstrapping");
    } 

    let clock = Clock::get()?;
    let time = clock.unix_timestamp;
    if let Some(bootstrap_start_time) = liquidity.bootstrap_start_time {
        if time < bootstrap_start_time {
            panic!("Bootstrap time not reached yet");
        }
    }

    let deployment = &ctx.accounts.deployment;

    let sold_out = deployment.number_of_tokens_issued >= deployment.max_number_of_tokens;

    if liquidity.bootstrap_requires_sold_out && !sold_out {
        panic!("Deployment not sold out yet");
    }
 
    let wrapped_sol_escrow = &mut ctx.accounts.wrapped_sol_escrow;

    anchor_spl::token::sync_native(CpiContext::new(ctx.accounts.token_program.to_account_info(), 
    anchor_spl::token::SyncNative{ account: wrapped_sol_escrow.to_account_info() }))?;

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];

    let bootstrap_accs = amm_proxy::cpi::accounts::BootstrapLiquidity { 
        pool: ctx.accounts.pool.to_account_info(),
            lp_mint: ctx.accounts.lp_mint.to_account_info(),
            user_pool_lp: ctx.accounts.pool_lp_token_account.to_account_info(),
            a_vault_lp: ctx.accounts.wrapped_sol_vault_lp.to_account_info(), 
            b_vault_lp: ctx.accounts.fungible_vault_lp.to_account_info(), 
            a_vault: ctx.accounts.wrapped_sol_vault.to_account_info(), 
            b_vault: ctx.accounts.fungible_vault.to_account_info(), 
            a_vault_lp_mint: ctx.accounts.wrapped_sol_vault_lp_mint.to_account_info(), 
            b_vault_lp_mint: ctx.accounts.fungible_vault_lp_mint.to_account_info(), 
            a_token_vault: ctx.accounts.wrapped_sol_token_vault.to_account_info(),
            b_token_vault: ctx.accounts.fungible_token_vault.to_account_info(),
            user_a_token: wrapped_sol_escrow.to_account_info(),
            user_b_token: fungible_escrow_token_account.to_account_info(),
            user: liquidity.to_account_info(), 
            vault_program: ctx.accounts.vault_program.to_account_info(), 
            token_program: ctx.accounts.token_program.to_account_info() 
    };

    amm_proxy::cpi::bootstrap_liquidity( CpiContext::new_with_signer(ctx.accounts.amm_program.to_account_info(), 
        bootstrap_accs, &[seeds]), 
            wrapped_sol_escrow.amount, 
            fungible_escrow_token_account.amount)?;

    Ok(())
}