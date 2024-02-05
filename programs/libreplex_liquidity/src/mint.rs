use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::TokenAccount};

use crate::Liquidity;
use libreplex_fair_launch::program::LibreplexFairLaunch;

#[derive(Accounts)]
pub struct MintCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked by has one
    pub authority: UncheckedAccount<'info>,

    #[account(mut, has_one = deployment, has_one = authority)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_config: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub creator_fee_treasury: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_hashlist_market: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub fungible_mint: UncheckedAccount<'info>,

    #[account(mut, 
        associated_token::authority = liquidity,
         associated_token::mint = fungible_mint)]
    pub liquidity_fungible_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_token_account: UncheckedAccount<'info>,


    /// CHECK: Checked in cpi.
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    pub token_program_22: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,

    /// CHECK: Checked in cpi.
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn mint_handler(ctx: Context<MintCtx>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;

    let seeds = &[
        b"liquidity",
        ctx.accounts.liquidity.seed.as_ref(),
        &[ctx.accounts.liquidity.bump],
    ];

    libreplex_fair_launch::cpi::mint_token22(CpiContext::new_with_signer(
        fair_launch.to_account_info(),
        libreplex_fair_launch::cpi::accounts::MintToken2022Ctx {
            deployment: ctx.accounts.deployment.to_account_info(),
            deployment_config: ctx.accounts.deployment_config.to_account_info(),
            creator_fee_treasury: ctx.accounts.creator_fee_treasury.to_account_info(),
            hashlist: ctx.accounts.hashlist.to_account_info(),
            hashlist_marker: ctx.accounts.pooled_hashlist_market.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            signer: ctx.accounts.liquidity.to_account_info(),
            fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
            minter: ctx.accounts.liquidity.to_account_info(),
            non_fungible_mint: ctx.accounts.pooled_non_fungible_mint.to_account_info(),
            non_fungible_token_account: ctx
                .accounts
                .pooled_non_fungible_token_account
                .to_account_info(),
            token_program: ctx.accounts.token_program_22.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[seeds],
    ))?;


    let pooled_non_fungible_token_account = &ctx.accounts.pooled_non_fungible_token_account;

    msg!("{:?}", pooled_non_fungible_token_account.owner);

    libreplex_fair_launch::cpi::swap_to_fungible22(
        CpiContext::new_with_signer(
            ctx.accounts.fair_launch.to_account_info(),
             libreplex_fair_launch::cpi::accounts::SwapToFungible2022Ctx {
                non_fungible_source_account_owner: ctx.accounts.liquidity.to_account_info(),
                fungible_target_token_account_owner: ctx.accounts.liquidity.to_account_info(),
                deployment: ctx.accounts.deployment.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: ctx.accounts.liquidity.to_account_info(),
                fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
                hashlist_marker: ctx.accounts.pooled_hashlist_market.to_account_info(),
                fungible_source_token_account: ctx.accounts.deployment_fungible_token_account.to_account_info(),
                fungible_target_token_account: ctx.accounts.liquidity_fungible_token_account.to_account_info(),
                non_fungible_mint: ctx.accounts.pooled_non_fungible_mint.to_account_info(),
                non_fungible_source_token_account: ctx.accounts.pooled_non_fungible_token_account.to_account_info(),
                non_fungible_target_token_account: ctx.accounts.deployment_non_fungible_token_account.to_account_info(),
                token_program_22: ctx.accounts.token_program_22.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                sysvar_instructions: ctx.accounts.sysvar_instructions.to_account_info(),
            }, 
             &[seeds]
        )
    )?;

    libreplex_fair_launch::cpi::mint_token22(CpiContext::new_with_signer(
        fair_launch.to_account_info(),
        libreplex_fair_launch::cpi::accounts::MintToken2022Ctx {
            deployment: ctx.accounts.deployment.to_account_info(),
            deployment_config: ctx.accounts.deployment_config.to_account_info(),
            creator_fee_treasury: ctx.accounts.creator_fee_treasury.to_account_info(),
            hashlist: ctx.accounts.hashlist.to_account_info(),
            hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            signer: ctx.accounts.liquidity.to_account_info(),
            fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
            minter: ctx.accounts.receiver.to_account_info(),
            non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
            non_fungible_token_account: ctx.accounts.non_fungible_token_account.to_account_info(),
            token_program: ctx.accounts.token_program_22.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[seeds],
    ))?;


    Ok(())
}
