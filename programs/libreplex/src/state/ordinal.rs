
use std::cell::RefMut;

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use prog_common::{errors::ErrorCode};


#[repr(C)]
#[account]
pub struct Ordinal {
    // no option to keep data easier to write into
    // set to 11111111.... or whatever to make this ordinal immutable
    pub authority: Pubkey, 
    pub data_length_current: u32,
    pub data_length_max: u32,
    pub data: Vec<u8>
}

impl Ordinal {
    pub const BASE_SIZE: usize = 8 + 32 + 4 + 4 + 4;

    pub fn get_size(&self) -> usize {
        Ordinal::BASE_SIZE + self.data.len()
    }

    pub fn append_data(
        &mut self,
        mut current_data: RefMut<&mut [u8]>,
        data_to_add: &Vec<u8>,
    ) -> Result<()> {
        if self.data_length_current + data_to_add.len() as u32 >= self.data_length_max {
            return Err(ErrorCode::MaxSizeExceeded.into());
        }
        msg!("LENGTH: {:?}", data_to_add.len());
       
        let current_index = Ordinal::BASE_SIZE + self.data_length_current as usize;
        msg!("current_index: {:?}", current_index);
        let data_slice: &mut [u8] = &mut current_data[current_index..current_index 
        + data_to_add.len()];
        data_slice.copy_from_slice(&data_to_add);
        self.data_length_current += data_to_add.len() as u32;

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum OrdinalEventType {
    Create,
    Append
}


#[event]
pub struct OrdinalEvent {
    pub id: Pubkey,
    pub event_type: OrdinalEventType
}
