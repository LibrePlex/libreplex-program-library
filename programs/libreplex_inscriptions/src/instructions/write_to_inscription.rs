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
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WriteToInscription>,
    inscription_input: WriteToInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let authority = &ctx.accounts.authority;

    let inscription_account_info = inscription.to_account_info();
    // check that the authority matches

    let inscription_authority = Inscription::get_authority(inscription_account_info.data.borrow())?;

    if inscription_authority != authority.key() {
        return Err(ErrorCode::BadAuthority.into());
    }

    Inscription::write_data(
        inscription_account_info.data.borrow_mut(),
        &inscription_input.data,
        inscription_input.start_pos,
    )?;

    emit!(InscriptionWriteEvent {
        id: inscription.key(),
    });
    
    Ok(())
}