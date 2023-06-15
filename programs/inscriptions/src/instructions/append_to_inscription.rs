
use anchor_lang::{prelude::*, accounts::signer};
use prog_common::errors::ErrorCode;

use crate::{Inscription, InscriptionEvent, InscriptionEventType};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AppendToInscriptionInput {
    pub append_data: Vec<u8>,
}

impl AppendToInscriptionInput {
    pub fn get_size(&self) -> usize {
        return 4 + self.append_data.len();
    }
}

#[derive(Accounts)]
#[instruction(ordinal_input: AppendToInscriptionInput)]
pub struct AppendToInscription<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(zero)]
    pub inscription: AccountLoader<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AppendToInscription>,
    append_to_ordinal_input: AppendToInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let signer = &ctx.accounts.signer;

    let ordinal_account_info = inscription.to_account_info();
    // check that the authority matches

    let authority = Inscription::get_authority(ordinal_account_info.data.borrow_mut())?;

    if  authority != signer.key() {
        return Err(ErrorCode::BadAuthority.into())
    }




    Inscription::append_data(ordinal_account_info.data.borrow_mut(), 
        &append_to_ordinal_input.append_data)?;

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}
