use crate::Inscription;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

#[event]
pub struct InscriptionEventDelete {
    pub id: Pubkey,
}

#[derive(Accounts)]
pub struct DeleteInscription<'info> {
    #[account(mut,
        constraint = inscription.authority == payer.key())]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut, close = payer)]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: validated in logic
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteInscription>) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;
    let payer = &mut ctx.accounts.payer;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &mut ctx.accounts.system_program;
    inscription_data.realloc(0, false)?;

    // transfer funds out now that the account has been reallocated

    invoke(
        &system_instruction::transfer(
            &payer.key(),
            inscription_data.key,
            inscription_data.lamports() * 1000,
        ),
        &[
            inscription_data.to_account_info().clone(),
            payer.to_account_info().clone(),
            system_program.to_account_info().clone(),
        ],
    )?;

    emit!(InscriptionEventDelete {
        id: inscription.key(),
    });

    Ok(())
}
