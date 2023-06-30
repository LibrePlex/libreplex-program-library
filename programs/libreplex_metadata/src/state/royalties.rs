use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct RoyaltyShare {
    // royalty address and their share in basis points (0-10,000)
    pub recipient: Pubkey,

    pub share: u16,
}

impl RoyaltyShare {
    pub const SIZE: usize = 32 + 2;
}


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Royalties {
    pub bps: u16,
    pub shares: Vec<RoyaltyShare>,
}

impl Royalties {
    pub const BASE_SIZE: usize = 2 + 4;
    pub fn get_size(&self) -> usize {
        return Royalties::BASE_SIZE + self.shares.len() * RoyaltyShare::SIZE;
    }
}