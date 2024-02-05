

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use libreplex_fair_launch::Deployment;
use vault_proxy::program::Vault;

use crate::Liquidity;


#[derive(Accounts)]
pub struct BootstapPool<'info> {
    #[account(mut, has_one = deployment)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account(has_one = fungible_mint)]
    pub deployment: Box<Account<'info, Deployment>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init_if_needed, 
        associated_token::authority = payer,
        associated_token::mint = wrapped_sol_mint,
         payer = payer)]
    pub payer_wrapped_sol_account: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed, 
        associated_token::authority = payer,
        associated_token::mint = fungible_mint,
         payer = payer)]
    pub payer_fungible_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub fungible_mint: Box<Account<'info, Mint>>,

    #[account(mut, associated_token::mint = fungible_mint, 
        associated_token::authority = liquidity)]
    pub fungible_escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = anchor_spl::token::spl_token::native_mint::id())]
    pub wrapped_sol_mint: Box<Account<'info, Mint>>,


    #[account(
        mut,
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
    pub lp_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in program
    #[account(
        mut,
    )]
    pub payer_lp_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked by address
    #[account(
        mut,
        seeds = [system_program.key.as_ref(), anchor_spl::token::spl_token::ID.as_ref(), lp_mint.key.as_ref()], bump, 
        seeds::program = anchor_spl::associated_token::ID
    )]
    pub system_program_lp_token_account: UncheckedAccount<'info>,


    pub amm_program: Program<'info, amm_proxy::program::Amm>,

    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Program<'info, Vault>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub fungible_token_fee: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub wrapped_sol_token_fee: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub fee_owner: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub metadata_program: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub lp_mint_metadata: UncheckedAccount<'info>,
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

    let bal = liquidity.to_account_info().lamports();
    let required_bal = rent.minimum_balance(liquidity.to_account_info().data_len());
    let excess_sol = bal.saturating_sub(required_bal);

    
    msg!("{} {} {}", bal, required_bal, excess_sol);

    if excess_sol > 0 {
        **(liquidity.try_borrow_mut_lamports()?) -= excess_sol;
        **(native_escrow.try_borrow_mut_lamports()?) += excess_sol;
    }

    Ok(())
}


pub fn bootstrap_pool_handler(ctx: Context<BootstapPool>) -> Result<()> {
    msg!("Handler: Bootstrap pool");
    let rent = Rent::get()?;
    let liquidity = ctx.accounts.liquidity.as_mut();
    let payer_fungible_token_account = &ctx.accounts.payer_fungible_mint_token_account;
    let payer_wrapped_sol_account = &mut ctx.accounts.payer_wrapped_sol_account;



    if liquidity.pool_bootstrapped {
        if payer_wrapped_sol_account.amount == 0 {
            anchor_spl::token::close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), 
            anchor_spl::token::CloseAccount {
                account: payer_wrapped_sol_account.to_account_info(),
                destination: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            }))?;
        }

        if payer_fungible_token_account.amount == 0 {
            anchor_spl::token::close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), 
            anchor_spl::token::CloseAccount {
                account: payer_fungible_token_account.to_account_info(),
                destination: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            }))?;
        }

        return Ok(())
    }

    let excess_sol = liquidity.to_account_info().lamports().saturating_sub(rent.minimum_balance(liquidity.to_account_info().data_len()));
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
    
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

    wrapped_sol_escrow.reload()?;

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];

    let wrapped_sol_vault = &ctx.accounts.wrapped_sol_vault;

    if wrapped_sol_vault.data_is_empty() {
        let init_a_vault_accounts = vault_proxy::cpi::accounts::Initialize {
            vault: wrapped_sol_vault.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            token_vault: ctx.accounts.wrapped_sol_token_vault.to_account_info(),
            token_mint: ctx.accounts.wrapped_sol_mint.to_account_info(),
            lp_mint: ctx.accounts.wrapped_sol_vault_lp_mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        vault_proxy::cpi::initialize(CpiContext::new(
            ctx.accounts.vault_program.to_account_info(), 
            init_a_vault_accounts))?;
    }

    let init_b_vault_accounts = vault_proxy::cpi::accounts::Initialize {
        vault: ctx.accounts.fungible_vault.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        token_vault: ctx.accounts.fungible_token_vault.to_account_info(),
        token_mint: ctx.accounts.fungible_mint.to_account_info(),
        lp_mint: ctx.accounts.fungible_vault_lp_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    vault_proxy::cpi::initialize(CpiContext::new(
        ctx.accounts.vault_program.to_account_info(), 
        init_b_vault_accounts))?;



    // WE need to transfer the wrapped SOL and fungible mint pooled amt to the payer temprorarily.
    let wrapped_sol_to_pool = wrapped_sol_escrow.amount;

    anchor_spl::token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
     anchor_spl::token::Transfer {
        from: wrapped_sol_escrow.to_account_info(),
        to: payer_wrapped_sol_account.to_account_info(),
        authority: liquidity.to_account_info(),
    }, &[seeds]), wrapped_sol_to_pool)?;

 
    anchor_spl::token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
    anchor_spl::token::Transfer {
       from: fungible_escrow_token_account.to_account_info(),
       to: payer_fungible_token_account.to_account_info(),
       authority: liquidity.to_account_info(),
   }, &[seeds]), fungible_escrow_token_account.amount)?;

    let init_pool_accs = amm_proxy::cpi::accounts::InitializePermissionlessPoolWithFeeTier {
        metadata_program: ctx.accounts.metadata_program.to_account_info(),
        mint_metadata: ctx.accounts.lp_mint_metadata.to_account_info(),
        pool: ctx.accounts.pool.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        token_a_mint: ctx.accounts.wrapped_sol_mint.to_account_info(),
        token_b_mint: ctx.accounts.fungible_mint.to_account_info(),
        a_vault: ctx.accounts.wrapped_sol_vault.to_account_info(),
        b_vault: ctx.accounts.fungible_vault.to_account_info(),
        a_token_vault: ctx.accounts.wrapped_sol_token_vault.to_account_info(),
        b_token_vault: ctx.accounts.fungible_token_vault.to_account_info(),
        a_vault_lp_mint: ctx.accounts.wrapped_sol_vault_lp_mint.to_account_info(),
        b_vault_lp_mint: ctx.accounts.fungible_vault_lp_mint.to_account_info(),
        a_vault_lp: ctx.accounts.wrapped_sol_vault_lp.to_account_info(),
        b_vault_lp: ctx.accounts.fungible_vault_lp.to_account_info(),
        payer_token_a: payer_wrapped_sol_account.to_account_info(),
        payer_token_b: payer_fungible_token_account.to_account_info(),
        payer_pool_lp: ctx.accounts.payer_lp_token_account.to_account_info(),
        admin_token_a_fee: ctx.accounts.wrapped_sol_token_fee.to_account_info(),
        admin_token_b_fee: ctx.accounts.fungible_token_fee.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        fee_owner: ctx.accounts.fee_owner.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        vault_program: ctx.accounts.vault_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };


    msg!("{} {}", wrapped_sol_escrow.amount, fungible_escrow_token_account.amount);

    amm_proxy::cpi::initialize_permissionless_pool_with_fee_tier(
        CpiContext::new(ctx.accounts.amm_program.to_account_info(), init_pool_accs),
         amm_proxy::CurveType::ConstantProduct, 100, 
         wrapped_sol_escrow.amount, 
         fungible_escrow_token_account.amount)?;

    payer_wrapped_sol_account.reload()?;

    if payer_wrapped_sol_account.amount == 0 {
        anchor_spl::token::close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), 
        anchor_spl::token::CloseAccount {
            account: payer_wrapped_sol_account.to_account_info(),
            destination: ctx.accounts.payer.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        }))?;
    }

    anchor_spl::token::close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), 
    anchor_spl::token::CloseAccount {
        account: payer_fungible_token_account.to_account_info(),
        destination: ctx.accounts.payer.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    }))?;

    let payer_lp_token_account = &ctx.accounts.payer_lp_token_account;

    let lp_supply = {
        let mut payer_lp_token_account_data: &[u8] = &payer_lp_token_account.try_borrow_data()?;

        anchor_spl::token::TokenAccount::try_deserialize(&mut payer_lp_token_account_data)?.amount
    };

    // Burn the lp token
    anchor_spl::associated_token::create(CpiContext::new(ctx.accounts.associated_token_program.to_account_info(),
     anchor_spl::associated_token::Create {
        payer: ctx.accounts.payer.to_account_info(),
        associated_token: ctx.accounts.system_program_lp_token_account.to_account_info(),
        authority: ctx.accounts.system_program.to_account_info(),
        mint: ctx.accounts.lp_mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    }
    ))?;

    anchor_spl::token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), 
            anchor_spl::token::Transfer {
                from: payer_lp_token_account.to_account_info(),
                to: ctx.accounts.system_program_lp_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            }), 
        lp_supply)?;

    liquidity.pool_bootstrapped = true;

    Ok(())
}