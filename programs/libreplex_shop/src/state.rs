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
    pub group: Pubkey,
    pub price: Price,
}

impl Listing{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 32 + 8 + 1 + 32; // price not included as this is added dynamically from list_input
   
}

#[account]
pub struct ListingGroup {
    pub admin: Pubkey,
    pub seed: Pubkey,
    pub listings_active: u32,
    pub listings_created: u32,
    pub listings_sold: u32,
    pub filter_count: u32,
    pub name: String,
}

impl ListingGroup{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 36 + 4 + 4 + 4 + 4; // price not included as this is added dynamically from list_input
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
    Group {
        pubkey: Pubkey
    }
}

#[account]
pub struct ListingFilter {
    pub listing_group: Pubkey,
    pub seed: Pubkey,
    pub filter_type: ListingFilterType,
    pub listings_active: u32,
    pub listings_created: u32,
    pub listings_sold: u32,
}

impl ListingFilter{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 4 + 4 + 4 + 2 + 32; // price not included as this is added dynamically from list_input
    
}


