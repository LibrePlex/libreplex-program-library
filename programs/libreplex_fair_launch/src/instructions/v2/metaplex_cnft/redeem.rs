
use anchor_lang::prelude::*;




use libreplex_shared::SharedError;

use crate::{
    errors::FairLaunchError, Deployment, HashlistMarker, MintEvent, Redeemable, redeem_cnft_logic,
};

#[derive(Accounts)]
pub struct InscribeCompressedCtx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Checked by seeds
    #[account(mut, 
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(mut, close = payer, has_one = deployment)]
    pub redeemable: Account<'info, Redeemable>,

    #[account(init, 
        space = 8,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        redeemable.asset.as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// CHECK: Checked by address
    #[account(
        mut,
        seeds = [redeemable.asset.as_ref()], 
        bump)]
    pub ghost_root_signer: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: sent via CPI to libreplex_inscriptions_program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    
    /// CHECK: Checked in constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn redeem(ctx: Context<InscribeCompressedCtx>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let signer = &ctx.accounts.signer;
    let payer = &ctx.accounts.payer;

    if deployment.require_creator_cosign && !signer.key().eq(&deployment.creator.key()) {
        return Err(SharedError::InvalidCreatorCosigner.into());
    }

    // to be discussed w/ everybody and feedback. Not strictly in line with BRC 20 thinking
    // but seems pointless to issue tokens if they can never be valid
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        return Err(FairLaunchError::MintedOut.into());
    }

    let hashlist = &mut ctx.accounts.hashlist;

    let inscription_summary = &ctx.accounts.inscription_summary;

    let inscriptions_program = &ctx.accounts.inscriptions_program;

    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    
    let asset_id = ctx.accounts.redeemable.asset;

    let ghost_root_signer = &ctx.accounts.ghost_root_signer;
    let ghost_root_seeds: &[&[u8]] = &[asset_id.as_ref(), &[ctx.bumps.ghost_root_signer]];

    redeem_cnft_logic(
        deployment, 
        inscriptions_program, 
        inscription_summary, 
        inscription_v3, 
        system_program,
        payer,
        inscription_data,
        ghost_root_signer,
        ghost_root_seeds,
        hashlist, 
        asset_id,
    )?;

    emit!(MintEvent{
        mint: asset_id,
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });

    Ok(())
}

