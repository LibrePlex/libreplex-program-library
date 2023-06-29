use crate::{Inscription, InscriptionEvent, InscriptionEventType};
use anchor_lang::prelude::*;

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionInput {
    pub max_data_length: u32,
    pub authority: Option<Pubkey>,
}

impl CreateInscriptionInput {
    pub fn get_size(&self) -> u32 {
        return self.max_data_length + 1 + match self.authority {
            Some(_)=>32,
            None=>0
        }
    }
}


#[derive(Accounts)]
#[instruction(ordinal_input: CreateInscriptionInput)]
pub struct CreateInscription<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub root: Signer<'info>,

    /// CHECK: validated in logic
    #[account(zero)]
    pub ordinal: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateInscription>,
    inscription_input: CreateInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.ordinal;

    let inscription_account_info = inscription.to_account_info();
    msg!("Writing authority");

    let authority = match inscription_input.authority {
        Some(x) => x.to_owned(),
        None => ctx.accounts.payer.key(),
    };

    inscription.authority = authority;
    inscription.size = inscription_input.max_data_length;
   

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}