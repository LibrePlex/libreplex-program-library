
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Phase {
    pub start_time: i64,
    pub end_time: Option<i64>,
    // price of a phase is always denominated in terms of 
    // SPL tokens. This gives us complete flexibility
    // as the tokens can be generated
    pub price_mint: Pubkey, 
    pub price_quantity: u64,
    pub max_mints: u64,
    pub collection: Pubkey
}

impl Phase {
    // not an account so no need for 8 byte discriminator space
    pub const BASE_SIZE: usize =  8 + 1 + 32 + 8 + 8 + 32;

    pub fn get_size(&self) -> usize {
        return Phase::BASE_SIZE + match &self.end_time {
            None => 0,
            Some(_) => 8
        }
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Blueprint {
    // attribute indices
    pub attributes: Vec<u8>
}


#[repr(C)]
#[account]
pub struct Toybox {
    pub max_mints: u64,
    pub minted: u64,
    pub phases: Vec<Phase>,
    pub attributes: Vec<u8>
}

impl Toybox {
    pub const BASE_SIZE: usize = 8 + 8 + 8 + 4 + 4;

    pub fn get_size (&self) -> usize {
        return Toybox::BASE_SIZE + &self.phases.len() * Phase::BASE_SIZE + &self.attributes.len()
    }
}
