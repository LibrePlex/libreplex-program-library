
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use libreplex::AttributeType;



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Phase {
    pub start_time: i64,
    pub end_time: Option<i64>,
    // price of a phase is always denominated in terms of 
    // SPL tokens. This gives us complete flexibility
    // as the tokens can be generated
    pub price_mint: Option<Pubkey>, 
    pub price_quantity: u64,
    pub max_mints: u64,
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
pub enum BaseUrl {
    Json {
        url: String,
    },
    Image {
        url: String,
    }
}

#[repr(C)]
#[account]
pub struct Creator {
    pub owner: Pubkey,
    pub seed: Pubkey,
    pub minted_count: u32,
    pub max_mints: u64,
    pub symbol: String,
    pub base_name: BaseUrl,
    pub minted: u64,
    pub collection: Pubkey, // has available attributes as well if appropriate
    pub bump: u8,
    pub description: Option<String>,
    pub phases: Vec<Phase>,
    pub attribute_mappings: Option<Pubkey>,
}

impl Creator {
    pub const BASE_SIZE: usize = 8 + 8 + 8 + 4 + 4;

    pub fn get_size (&self) -> usize {
        return Creator::BASE_SIZE + &self.phases.len() * Phase::BASE_SIZE 
    }
}
