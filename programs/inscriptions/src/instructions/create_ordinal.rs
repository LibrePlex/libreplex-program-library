use crate::{  Ordinal, OrdinalEventType, OrdinalEvent};
use anchor_lang::prelude::*;


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateOrdinalInput {
    pub max_data_length: u32,
    pub initial_data: Vec<u8>,
}




#[derive(Accounts)]
#[instruction(ordinal_input: CreateOrdinalInput)]
pub struct CreateOrdinal<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut)]
    pub ordinal: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateOrdinal>, 
    ordinal_input: CreateOrdinalInput) -> Result<()> {


    let ordinal = &mut ctx.accounts.ordinal;

    
    let initial_data = ordinal_input.initial_data; 

    let ordinal_account_info = ordinal.to_account_info();
  
    
    Ordinal::write_authority(
        ordinal_account_info.data.borrow_mut(),
        &ctx.accounts.payer.key())?;

    Ordinal::write_data_length_max(
        ordinal_account_info.data.borrow_mut(),
        ordinal_input.max_data_length)?;
    
    // ordinal.data_length_max = ;
    // ordinal.data_length_current = 0; //initial_data.len() as u32;

    

    // ordinal.data = initial_data.clone();

    Ordinal::append_data(ordinal_account_info.data.borrow_mut(), 
    &initial_data)?;

    msg!("Data length at create {:?}", initial_data.len());

    emit!(OrdinalEvent {
        id: ordinal.key(),
        event_type: OrdinalEventType::Create
    });

    Ok(())
}
