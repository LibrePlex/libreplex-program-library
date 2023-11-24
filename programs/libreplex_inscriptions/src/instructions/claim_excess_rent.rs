use std::cmp;

use crate::{Inscription, MINIMUM_INSCRIPTION_LAMPORTS};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimExcessRent<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(
        constraint = inscription.authority == authority.key())]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: validated in logic
    #[account(
        mut,
    seeds=[
        "inscription_data".as_bytes(),
        inscription.root.as_ref()
    ],bump)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimExcessRent>) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let inscription_data = &mut ctx.accounts.inscription_data;

    let payer = &mut ctx.accounts.payer;

    let rent = Rent::get()?;

    let minimum_balance_for_rent = rent.minimum_balance(inscription.size as usize);

    let minimum_balance = cmp::max(
        minimum_balance_for_rent,
        MINIMUM_INSCRIPTION_LAMPORTS,
    );
    
    let lamports_diff = inscription_data.lamports().saturating_sub(minimum_balance);

    msg!("lamports {}", lamports_diff);
    if lamports_diff > 0 {
        inscription_data.sub_lamports(lamports_diff)?;
        payer.add_lamports(lamports_diff)?;
    }

    Ok(())
}
