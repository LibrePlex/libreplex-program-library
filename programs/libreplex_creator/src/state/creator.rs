
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum AssetUrl {
    JsonPrefix {
        url: String,
    },
    ChainRenderer {
        program_id: Pubkey,
    },
}

// Case off chain meta/image template from a base
// Case on chain draw from a 
// Case renderer


#[account]
pub struct Creator {
    pub update_authority: Pubkey,
    // Only this key can mint
    pub creator_authority: Pubkey,
    pub seed: Pubkey,
    pub supply: u32,
    pub symbol: String,
    pub asset_url: AssetUrl,
    pub minted: u32,
    pub collection: Pubkey, // has available attributes as well if appropriate
    pub bump: u8,
    pub description: Option<String>,
    pub attribute_mappings: Option<Pubkey>,
    pub is_ordered: bool,
    pub name: String,
    pub minter_numbers: Option<Pubkey>,
}



impl Creator {
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 32 + 4 
    // Lets say 14 for max symbol
    + 14
    + 1 + 32
    + 4 + 32 + 1
    // Desc
    + 1 + 40
    + 1 + 32
    + 1
    // name
    + 20 
    + 1 + 32;

    pub fn get_size (&self) -> usize {
        Creator::BASE_SIZE
    }
}

// Keeping it for now. Stores available/used mint numbers.
#[account]
pub struct MintNumbers {
    pub creator: Pubkey,
}
