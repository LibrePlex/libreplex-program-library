use anchor_lang::{accounts::signer, prelude::*};
use crate::errors::ErrorCode;

use crate::{Inscription, InscriptionEvent, InscriptionEventType};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct WriteToInscriptionInput {
    pub data: Vec<u8>,
    pub start_pos: u32,
}

#[derive(Accounts)]
#[instruction(ordinal_input: WriteToInscriptionInput)]
pub struct WriteToInscription<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Authority checked in logic
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WriteToInscription>,
    append_to_ordinal_input: WriteToInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let signer = &ctx.accounts.signer;

    let ordinal_account_info = inscription.to_account_info();
    // check that the authority matches

    let authority = Inscription::get_authority(ordinal_account_info.data.borrow_mut())?;

    if authority != signer.key() {
        return Err(ErrorCode::BadAuthority.into());
    }

    Inscription::write_data(
        ordinal_account_info.data.borrow_mut(),
        &append_to_ordinal_input.data,
        append_to_ordinal_input.start_pos,
    )?;

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}