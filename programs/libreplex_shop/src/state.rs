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
    pub price: Price,
    pub amount: u64,
    pub listing_bump: u8
}

impl Listing{
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 8; // price not included as this is added dynamically from list_input
}