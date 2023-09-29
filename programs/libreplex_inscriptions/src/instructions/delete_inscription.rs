use crate::{Inscription};
use anchor_lang::prelude::*;


#[event]
pub struct InscriptionEventDelete {
    pub id: Pubkey
}

#[derive(Accounts)]
pub struct DeleteInscription<'info> {
    #[account(mut,
        constraint = inscription.authority == payer.key())]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut, close = payer)]
    pub inscription: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DeleteInscription>
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    emit!(InscriptionEventDelete {
        id: inscription.key(),
    });

    Ok(())
}