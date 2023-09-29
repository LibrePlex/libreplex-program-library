use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Price {
    Native {
        lamports: u64,
    },
    Spl {
        mint: Pubkey,
        amount: u64
    }
}

#[account]
pub struct Listing {
    pub mint: Pubkey,
    pub lister: Pubkey, 
    pub amount: u64, 
    pub listing_bump: u8, 
    pub collection: Option<Pubkey>,// set to the group of the mint for indexing,
    pub price: Price,
}

impl Listing{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 8 + 1 + 32; // price not included as this is added dynamically from list_input
   
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum ListingFilterType {
    // to allow listings of items from a particular creator
    Creator {
        pubkey: Pubkey
    },
    // to allow any listings as long as they're listed by a particular lister
    Lister {
        pubkey: Pubkey
    },
    // to allow listings of items belonging to a group
    Collection {
        pubkey: Pubkey
    }
}

#[account]
pub struct ListingFilter {
    pub listing_collection: Pubkey,
    pub seed: Pubkey,
    pub filter_type: ListingFilterType,
    pub listings_active: u32,
    pub listings_created: u32,
    pub listings_sold: u32,
}

impl ListingFilter{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 4 + 4 + 4 + 2 + 32; // price not included as this is added dynamically from list_input
    
}


