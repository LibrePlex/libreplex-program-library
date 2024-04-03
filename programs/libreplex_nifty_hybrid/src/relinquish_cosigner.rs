use anchor_lang::prelude::*;
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment};

use crate::NiftyHybrid;

#[derive(Accounts)]
pub struct RelinquishCosignersCtx<'info> {


    #[account(mut,
        constraint = deployment.creator.eq(&nifty_hybrid.key()))]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,

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

    let nifty_hybrid = &ctx.accounts.nifty_hybrid;
    let deployment = &ctx.accounts.deployment;
    let payer = &ctx.accounts.payer;
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;

    let seeds = &[
        b"nifty_hybrid",
        nifty_hybrid.seed.as_ref(),
        &[nifty_hybrid.bump],
    ];

    if deployment.number_of_tokens_issued < deployment.max_number_of_tokens {
        panic!("Cannot relinquish cosigner before mintout");
    }

    libreplex_fair_launch::cpi::relinquish_cosigner(CpiContext::new_with_signer(
        libreplex_fair_launch_program.to_account_info(),
        libreplex_fair_launch::cpi::accounts::RelinquishCosignersCtx {
            deployment: deployment.to_account_info(),
            payer: payer.to_account_info(),
            cosigner: nifty_hybrid.to_account_info(),
        },
        &[seeds],
    ))?;

    Ok(())
}
