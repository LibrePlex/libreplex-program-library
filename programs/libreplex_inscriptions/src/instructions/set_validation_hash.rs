use crate::Inscription;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(validation_hash: Option<String>)]
pub struct SetValidationHash<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // this must be either the root or else a PDA
    // that is accepted as a proxy
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: validated in logic
    #[account(mut,
        realloc = Inscription::BASE_SIZE + match &validation_hash {
            Some(x)=> 4 + x.len(),
            None => 0
        },
        realloc::payer = payer,
        realloc::zero = true,
        constraint = inscription.authority == signer.key())]
    pub inscription: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetValidationHash>, validation_hash: Option<String>) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;
    inscription.validation_hash = validation_hash;

    Ok(())
}
