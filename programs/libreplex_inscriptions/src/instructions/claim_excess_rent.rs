


use crate::Inscription;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;

#[derive(Accounts)]
pub struct ClaimExcessRent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut,
        constraint = inscription.authority == authority.key())]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: validated in logic
    #[account(
    seeds=[
        "inscription_data".as_bytes(),
        inscription.root.as_ref()
    ],bump)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ClaimExcessRent>,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let inscription_data = &mut ctx.accounts.inscription_data;

    let system_program = &mut ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;


    let rent = Rent::get()?;
    let minimum_balance = rent.minimum_balance(inscription.size as usize);

    
    let inscription_root = inscription.root.key();

    msg!("Decreasing to {}", minimum_balance);
    // reallocate first

    let auth_seeds = [
        "inscription_data".as_bytes(),
        inscription_root.as_ref(),
        &[ctx.bumps["inscription_data"]],
    ];

    let lamports_diff = inscription_data
        .lamports()
        .saturating_sub(minimum_balance);

    msg!("Lamports diff {}", lamports_diff);

    invoke_signed(
        &system_instruction::transfer(inscription_data.key, &payer.key(), lamports_diff),
        &[
            inscription_data.to_account_info(),
            payer.to_account_info(),
            system_program.to_account_info(),
        ],
        &[&auth_seeds],
    )?;
    

    Ok(())
}
