
use std::cell::{RefMut, Ref};

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use crate::{errors::ErrorCode};

#[account]
pub struct Inscription {
    
    // no option to keep data easier to write into
    // set to 11111111.... or whatever to make this ordinal immutable
    pub authority: Pubkey, 
    
    // root is the thing that the Inscription inscribes
    // could also be called inscribee but that would 
    // be weird 
    pub root: Pubkey, 
    pub size: u32,
    // we do not mark this field as being serialized at all. instead we 
    // right to it directly via append_data method
    // pub data: Vec<u8>
}

impl Inscription {
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 4; // no need for vector padding as we write bytes directly onto the account

    pub fn get_size(&self) -> usize {
        Inscription::BASE_SIZE + self.size as usize
    }

    pub fn write_authority(
        mut current_data: RefMut<&mut [u8]>,
        authority: &Pubkey) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[8..40];
        current_position_slice.copy_from_slice(authority.as_ref()); 
        Ok(())
    }

    pub fn get_authority(
        current_data: RefMut<&mut [u8]>
    ) -> Result<Pubkey> {
        Ok(Pubkey::try_from_slice(&current_data[8..40])?)
    }

    pub fn get_data_length_current(
        current_data: Ref<&mut [u8]>
    ) -> Result<u32> {
        Ok(u32::try_from_slice(&current_data[72..76])?)
    }

    pub fn write_root(
        mut current_data: RefMut<&mut [u8]>,
        root: &Pubkey) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[40..72];
        current_position_slice.copy_from_slice(root.as_ref()); 
        Ok(())
    }


    pub fn write_data_length_max(
        mut current_data: RefMut<&mut [u8]>,
        max_data_length: u32) -> Result<()> {
        let max_length_slice: &mut [u8] = &mut current_data[72..76];
        max_length_slice.copy_from_slice(&max_data_length.to_le_bytes()); 
        Ok(())
    }

    pub fn write_data(
        mut current_data: RefMut<&mut [u8]>,
        data_to_add: &Vec<u8>,
        start_pos: u32
    ) -> Result<()> {

        let data_length_max = u32::from_le_bytes(current_data[72..76].try_into().unwrap());

        msg!("{} {} {} ", start_pos,  data_to_add.len(),  data_length_max);
        if start_pos + data_to_add.len() as u32 > data_length_max {
            return Err(ErrorCode::MaxSizeExceeded.into());
        }
        msg!("LENGTH: {:?}", data_to_add.len());
       
        let current_index = Inscription::BASE_SIZE + start_pos as usize;
        msg!("current_index: {:?}", current_index);
        let data_slice: &mut [u8] = &mut current_data[current_index..current_index 
        + data_to_add.len()];
        data_slice.copy_from_slice(&data_to_add);

        Ok(())
    }
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum InscriptionEventType {
    Create,
    Update,
    Resize
}


#[event]
pub struct InscriptionEvent {
    pub id: Pubkey,
    pub event_type: InscriptionEventType
}