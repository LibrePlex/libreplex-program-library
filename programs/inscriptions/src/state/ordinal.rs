
use std::cell::RefMut;

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use prog_common::{errors::ErrorCode};


#[repr(C)]
#[account(zero_copy)]
pub struct Ordinal {
    // no option to keep data easier to write into
    // set to 11111111.... or whatever to make this ordinal immutable
    pub authority: Pubkey, 
    pub data_length_current: u32,
    pub data_length_max: u32,
    // we do not mark this field as being serialized at all. instead we 
    // right to it directly via append_data method
    // pub data: Vec<u8>
}

impl Ordinal {
    pub const BASE_SIZE: usize = 8 + 32 + 4 + 4; // no need for vector padding as we write bytes directly onto the account

    pub fn get_size(&self) -> usize {
        Ordinal::BASE_SIZE + self.data_length_max as usize
    }

    pub fn write_authority(
        mut current_data: RefMut<&mut [u8]>,
        authority: &Pubkey) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[8..40];
        current_position_slice.copy_from_slice(authority.as_ref()); 
        Ok(())
    }

    pub fn write_data_length_max(
        mut current_data: RefMut<&mut [u8]>,
        max_data_length: u32) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[44..48];
        current_position_slice.copy_from_slice(&max_data_length.to_le_bytes()); 
        Ok(())
    }

    pub fn append_data(
        mut current_data: RefMut<&mut [u8]>,
        data_to_add: &Vec<u8>,
    ) -> Result<()> {

        let data_length_current = u32::from_le_bytes(current_data[40..44].try_into().unwrap());
        let data_length_max = u32::from_le_bytes(current_data[44..48].try_into().unwrap());

        if data_length_current + data_to_add.len() as u32 >= data_length_max {
            return Err(ErrorCode::MaxSizeExceeded.into());
        }
        msg!("LENGTH: {:?}", data_to_add.len());
       
        let current_index = Ordinal::BASE_SIZE + data_length_current as usize;
        msg!("current_index: {:?}", current_index);
        let data_slice: &mut [u8] = &mut current_data[current_index..current_index 
        + data_to_add.len()];
        data_slice.copy_from_slice(&data_to_add);

        let current_position_slice: &mut [u8] = &mut current_data[40..44];
        current_position_slice.copy_from_slice(&(data_length_current + data_to_add.len() as u32).to_le_bytes());
        // self.data_length_current += data_to_add.len() as u32;

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
