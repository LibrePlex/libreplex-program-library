use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

use crate::Inscription;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct WriteToInscriptionInput {
    pub data: Vec<u8>,
    pub start_pos: u32,
}

#[event]
pub struct InscriptionWriteEvent {
    pub id: Pubkey
}

#[derive(Accounts)]
#[instruction(inscription_input: WriteToInscriptionInput)]
pub struct WriteToInscription<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Authority checked in logic
    #[account(mut,
        constraint = inscription.authority == authority.key(),
        constraint = inscription.inscription_data == inscription_data.key())]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: we never want anchor to handle this. It's just a data blob 
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WriteToInscription>,
    inscription_input: WriteToInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let authority = &ctx.accounts.authority;

    let inscription_data = &ctx.accounts.inscription_data;

    let inscription_data_account_info = inscription_data.to_account_info();
    // check that the authority matches

    if inscription.authority != authority.key() {
        return Err(ErrorCode::BadAuthority.into());
    }

    if inscription.inscription_data != inscription_data.key() {
        return Err(ErrorCode::IncorrectInscriptionDataAccount.into());
    }

    inscription.write_data(
        inscription_data_account_info.data.borrow_mut(),
        &inscription_input.data,
        inscription_input.start_pos,
    )?;

    emit!(InscriptionWriteEvent {
        id: inscription.key(),
    });
    
    Ok(())
}