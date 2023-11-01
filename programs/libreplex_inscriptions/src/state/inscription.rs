use std::cell::{Ref, RefMut};

use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum SummaryExtension {
    None,
}

#[account]
pub struct InscriptionRankPage {
    pub size: u32,
}

impl InscriptionRankPage {
    // discriminator + vector size
    pub const BASE_SIZE: usize = 8 + 4;

    pub fn add_inscription(
        &mut self,
        mut current_data: RefMut<&mut [u8]>,
        inscription: Pubkey
    ) -> Result<()> {
        let data_length_max = u32::from_le_bytes(current_data[8..12].try_into().unwrap()) as usize;
        println!("data length {}", data_length_max);
        let data_slice: &mut [u8] =
            &mut current_data[(12 + data_length_max * 32)..(12 + (data_length_max +1)* 32)];
        data_slice.copy_from_slice(inscription.key().as_ref());

        self.size += 1;

        Ok(())
    }

    pub fn get_inscriptions<'a>(current_data: &'a RefMut<'a, &mut [u8]>, start_pos: usize, end_pos: usize) -> impl Iterator<Item=Pubkey> + 'a {

        let effective_start_pos = std::cmp::min(12 + start_pos*32, current_data.len());
        let effective_end_pos = std::cmp::min(12 + end_pos*32, current_data.len());

        println!("start: {}, end: {}", effective_start_pos, effective_end_pos);
        let byte_slice = &current_data[(effective_start_pos)..(effective_end_pos)];
        byte_slice.chunks(32).map(|x| Pubkey::try_from_slice(x).unwrap())
    }
}

#[account]
pub struct InscriptionSummary {
    pub inscription_count_total: u64,
    pub inscription_count_immutables: u64,
    pub last_inscription: Pubkey,
    pub last_inscriber: Pubkey,
    pub last_inscription_create_time: i64,
    pub extension: SummaryExtension,
}

impl InscriptionSummary {
    pub const BASE_SIZE: usize = 8 + 8 + 8 + 32 + 32 + 8 + 2;
}

#[account]
pub struct InscriptionRank {
    // rank of the inscription, the earlier the inscription, the lower the rank
    pub rank: u64,
    // a pointer to the inscription at this rank
    pub inscription: Pubkey,
}

#[account]
pub struct Inscription {
    // no option to keep data easier to write into
    // set to 11111111.... or whatever to make this inscription immutable
    pub authority: Pubkey, // 8

    // root is the thing that the Inscription inscribes
    // could also be called inscribee but that would
    // be weird
    pub root: Pubkey, // 8 + 32 = 40

    // rank 0 - unranked. ranks 1,2,3,4,5,6 represent the rank of this inscription in the order they are made immutable
    // only immutable inscriptions are ranked.

    pub rank: u64, // 8 + 32 + 32 = 72
    pub size: u32,    // 8 + 32 + 32 + 8 = 80
                      // we do not mark the following field as being serialized at all. instead we
                      // write to it directly via append_data method
                      // pub data: Vec<u8>
}

impl Inscription {
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 8 + 4; // no need for vector padding as we write bytes directly onto the account

    pub fn get_size(&self) -> usize {
        Inscription::BASE_SIZE + self.size as usize
    }

    pub fn write_authority(mut current_data: RefMut<&mut [u8]>, authority: &Pubkey) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[8..40];
        current_position_slice.copy_from_slice(authority.as_ref());
        Ok(())
    }

    pub fn get_authority(current_data: Ref<&mut [u8]>) -> Result<Pubkey> {
        Ok(Pubkey::try_from_slice(&current_data[8..40])?)
    }

    pub fn get_counter(current_data: Ref<&mut [u8]>) -> Result<u64> {
        Ok(u64::try_from_slice(&current_data[72..76])?)
    }

    pub fn get_data_length(current_data: Ref<&mut [u8]>) -> Result<u32> {
        Ok(u32::try_from_slice(&current_data[80..84])?)
    }

    pub fn write_root(mut current_data: RefMut<&mut [u8]>, root: &Pubkey) -> Result<()> {
        let current_position_slice: &mut [u8] = &mut current_data[40..72];
        current_position_slice.copy_from_slice(root.as_ref());
        Ok(())
    }

    pub fn write_data_length_max(
        mut current_data: RefMut<&mut [u8]>,
        max_data_length: u32,
    ) -> Result<()> {
        let max_length_slice: &mut [u8] = &mut current_data[80..84];
        max_length_slice.copy_from_slice(&max_data_length.to_le_bytes());
        Ok(())
    }

    pub fn write_data(
        mut current_data: RefMut<&mut [u8]>,
        data_to_add: &Vec<u8>,
        start_pos: u32,
    ) -> Result<()> {
        let data_length_max = u32::from_le_bytes(current_data[80..84].try_into().unwrap());

        if start_pos + data_to_add.len() as u32 > data_length_max {
            return Err(ErrorCode::MaxSizeExceeded.into());
        }

        let current_index = Inscription::BASE_SIZE + start_pos as usize;
        let data_slice: &mut [u8] =
            &mut current_data[current_index..current_index + data_to_add.len()];
        data_slice.copy_from_slice(data_to_add);

        Ok(())
    }
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum InscriptionEventType {
    Create,
    Update,
    Resize,
}

#[event]
pub struct InscriptionEvent {
    pub id: Pubkey,
    pub event_type: InscriptionEventType,
}
