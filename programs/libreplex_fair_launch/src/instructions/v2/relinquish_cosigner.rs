use anchor_lang::prelude::*;

use crate::Deployment;

#[derive(Accounts)]
pub struct RelinquishCosignersCtx<'info> {
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        constraint = cosigner.key() == deployment.creator,
        constraint = deployment.require_creator_cosign)]
    pub cosigner: Signer<'info>,

}

pub fn relinquish_cosigner(ctx: Context<RelinquishCosignersCtx>) -> Result<()> {
    let deployment: &mut Account<'_, Deployment> = &mut ctx.accounts.deployment;
    deployment.require_creator_cosign = false;

    Ok(())
}
