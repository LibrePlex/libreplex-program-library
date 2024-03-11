use anchor_lang::prelude::*;
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment, ReduceMintCountInput};

use crate::Liquidity;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct ReduceMintCountInputLiquidity {
    pub max_number_of_tokens: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in
}

#[derive(Accounts)]
#[instruction(input: ReduceMintCountInputLiquidity)]
pub struct ReduceMintCountCtx<'info> {
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        constraint = liquidity.authority == creator.key(),
        constraint = liquidity.deployment == deployment.key() )]
    pub liquidity: Account<'info, Liquidity>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Can be anyone.
    pub creator: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    #[account()]
    pub libreplex_fair_launch_program: Program<'info, LibreplexFairLaunch>,
}

pub fn handle_reduce_mint_count(
    ctx: Context<ReduceMintCountCtx>,
    input: ReduceMintCountInputLiquidity,
) -> Result<()> {
    let deployment = &ctx.accounts.deployment;

    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;
    let payer = &ctx.accounts.payer;
    let liquidity = &ctx.accounts.liquidity;
    let system_program = &ctx.accounts.system_program;

    let seeds = &[b"liquidity", liquidity.seed.as_ref(), &[liquidity.bump]];

    libreplex_fair_launch::cpi::reduce_mint_count(
        CpiContext::new_with_signer(
            libreplex_fair_launch_program.to_account_info(),
            libreplex_fair_launch::cpi::accounts::ReduceMintCountCtx {
                deployment: deployment.to_account_info(),
                payer: payer.to_account_info(),
                creator: liquidity.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[seeds],
        ),
        ReduceMintCountInput {
            max_number_of_tokens: input.max_number_of_tokens,
        },
    )?;

    Ok(())
}
