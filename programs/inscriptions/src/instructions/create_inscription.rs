use crate::{  Inscription, InscriptionEventType, InscriptionEvent};
use anchor_lang::prelude::*;


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionInput {
    pub max_data_length: u32,
    pub initial_data: Vec<u8>,
}




#[derive(Accounts)]
#[instruction(ordinal_input: CreateInscriptionInput)]
pub struct CreateInscription<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub root: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut)]
    pub ordinal: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateInscription>, 
    inscription_input: CreateInscriptionInput) -> Result<()> {


    let inscription = &mut ctx.accounts.ordinal;

    
    let initial_data = inscription_input.initial_data; 

    let inscription_account_info = inscription.to_account_info();
  
    
    Inscription::write_authority(
        inscription_account_info.data.borrow_mut(),
        &ctx.accounts.payer.key())?;

    Inscription::write_data_length_max(
        inscription_account_info.data.borrow_mut(),
        inscription_input.max_data_length)?;
    
    // ordinal.data_length_max = ;
    // ordinal.data_length_current = 0; //initial_data.len() as u32;

    

    // ordinal.data = initial_data.clone();

    Inscription::append_data(inscription_account_info.data.borrow_mut(), 
    &initial_data)?;

    msg!("Data length at create {:?}", initial_data.len());

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}
