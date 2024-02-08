use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment};

use crate::Liquidity;


#[derive(Accounts)]
pub struct SwapToFungible<'info> {
    #[account(has_one = deployment)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account(
        mut,
        has_one = fungible_mint
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub fungible_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut, seeds = ["hashlist_marker".as_bytes(), 
    deployment.key().as_ref(),
    non_fungible_mint.key().as_ref()],
    bump, seeds::program = libreplex_fair_launch::ID)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub fungible_source_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(
        mut,
    )]
    pub non_fungible_source_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub non_fungible_target_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub token_program_22:  UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub token_program:  UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi
    sysvar_instructions: UncheckedAccount<'info>,

    fair_launch_program: Program<'info, LibreplexFairLaunch>
}


pub fn swap_to_fungible_handler(ctx: Context<SwapToFungible>) -> Result<()> {
    let liqudity = &ctx.accounts.liquidity;

    
    if !liqudity.pool_bootstrapped {
        panic!("Cannot swap until pool has been bootstrapped");
    }

    let seeds = &[
        b"liquidity",
        ctx.accounts.liquidity.seed.as_ref(),
        &[ctx.accounts.liquidity.bump],
    ];


    libreplex_fair_launch::cpi::swap_to_fungible22(
        CpiContext::new_with_signer(
            ctx.accounts.fair_launch_program.to_account_info(),
            libreplex_fair_launch::cpi::accounts::SwapToFungible2022Ctx {
                deployment: ctx.accounts.deployment.to_account_info(),
                fungible_target_token_account_owner: ctx.accounts.payer.to_account_info(),
                non_fungible_source_account_owner: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: liqudity.to_account_info(),
                fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
                hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
                fungible_source_token_account: ctx.accounts.fungible_source_token_account.to_account_info(),
                fungible_target_token_account: ctx.accounts.fungible_target_token_account.to_account_info(),
                non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
                non_fungible_source_token_account: ctx.accounts.non_fungible_source_token_account.to_account_info(),
                non_fungible_target_token_account: ctx.accounts.non_fungible_target_token_account.to_account_info(),
                token_program_22: ctx.accounts.token_program_22.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                sysvar_instructions: ctx.accounts.sysvar_instructions.to_account_info(),
            },
            &[seeds])
    )?;

    Ok(())
}