use anchor_lang::prelude::*;
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment};

use crate::Liquidity;

#[derive(Accounts)]
pub struct RelinquishCosignersCtx<'info> {


    #[account(mut,
        constraint = deployment.creator.eq(&liquidity.key()))]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub libreplex_fair_launch_program: Program<'info, LibreplexFairLaunch>,

}

pub fn relinquish_cosigner_handler(ctx: Context<RelinquishCosignersCtx>) -> Result<()> {
    // give up cosigner as it's no longer needed
    // the only reason liquidity grabs it is to prevent swaps
    // until minted out

    let liquidity = &ctx.accounts.liquidity;
    let deployment = &ctx.accounts.deployment;
    let payer = &ctx.accounts.payer;
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];

    if !liquidity.pool_bootstrapped {
        panic!("Cannot relinquish cosigner before bootstrap");
    }

    libreplex_fair_launch::cpi::relinquish_cosigner(CpiContext::new_with_signer(
        libreplex_fair_launch_program.to_account_info(),
        libreplex_fair_launch::cpi::accounts::RelinquishCosignersCtx {
            deployment: deployment.to_account_info(),
            payer: payer.to_account_info(),
            cosigner: liquidity.to_account_info(),
        },
        &[seeds],
    ))?;

    Ok(())
}
