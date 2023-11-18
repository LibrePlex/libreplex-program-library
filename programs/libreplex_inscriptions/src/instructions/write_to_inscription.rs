use crate::{errors::ErrorCode, InscriptionV3};
use anchor_lang::prelude::*;

use crate::{EncodingType, Inscription, MediaType};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct WriteToInscriptionInput {
    pub data: Vec<u8>,
    pub start_pos: u32,
    // when provided, will toggle the media type of the inscription
    pub media_type: Option<MediaType>,
    pub encoding_type: Option<EncodingType>,
}


#[event]
pub struct InscriptionWriteEvent {
    pub id: Pubkey,
}

#[derive(Accounts)]
#[instruction(inscription_input: WriteToInscriptionInput)]
pub struct WriteToInscription<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // required for the realloc if inscription size changes due
    // to media_type / encoding_type
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Authority checked in logic
    #[account(mut,
        realloc = Inscription::BASE_SIZE + inscription.get_new_size(&inscription_input),
        realloc::payer = payer,
        realloc::zero = false,
        constraint = inscription.authority == authority.key(),
        constraint = inscription.inscription_data == inscription_data.key())]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: Authority checked in logic
    #[account(mut,
        realloc = Inscription::BASE_SIZE + inscription.get_new_size(&inscription_input),
        realloc::payer = payer,
        realloc::zero = false,
        constraint = inscription.authority == authority.key(),
        constraint = inscription.inscription_data == inscription_data.key())]
    pub inscription2: Option<Account<'info, InscriptionV3>>,

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
    let inscription_v2 = &mut ctx.accounts.inscription2;

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

    if let Some(x) =inscription_input.media_type {
        inscription.media_type = x.to_owned();
        match inscription_v2 {
            Some(y)=>{ 
                y.content_type = x.convert_to_string();
            },
            None => {

            }
            
        }
        
    }

    if let Some(x) =inscription_input.encoding_type {
        inscription.encoding_type = x.to_owned();
        match inscription_v2 {
            Some(y)=>{ 
                y.encoding = x.convert_to_string();
            },
            None => {

            }
            
        }

    }
    if !inscription_input.data.is_empty() {
        inscription.write_data(
            inscription_data_account_info.data.borrow_mut(),
            &inscription_input.data,
            inscription_input.start_pos,
        )?;
    }

    emit!(InscriptionWriteEvent {
        id: inscription.key(),
    });

    Ok(())
}
