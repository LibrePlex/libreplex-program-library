use crate::state::{Metadata};
use crate::{ CreateMetadataInput, Permissions, PermissionType, MetadataEvent, MetadataEventType, Ordinal, OrdinalEventType, OrdinalEvent};
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
    pub signer: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, 
        payer = payer,
        // allocate the full ordinal size now
        // we can add an extend / reduce endpoint 
        // later as needed
        constraint = ordinal.key() == signer.key(), 
        space = Ordinal::BASE_SIZE + ordinal_input.max_data_length as usize)]
    pub ordinal: Box<Account<'info, Ordinal>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateOrdinal>, 
    ordinal_input: CreateOrdinalInput) -> Result<()> {
    let ordinal = &mut ctx.accounts.ordinal;

    let initial_data = ordinal_input.initial_data; 
  
    ordinal.data = initial_data.clone();
    ordinal.authority = ctx.accounts.signer.key();
    ordinal.data_length_max = ordinal_input.max_data_length;
    ordinal.data_length_current = initial_data.len() as u32;

    msg!("Data length at create {:?}", initial_data.len());

    emit!(OrdinalEvent {
        id: ordinal.key(),
        event_type: OrdinalEventType::Create
    });

    Ok(())
}
