use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};



#[account]
pub struct SwapMarker {
    pub pipeline: Pubkey,
    pub incoming_mint: Pubkey,
}

impl SwapMarker {
    pub const SIZE: usize = 8 + 32 + 32;
}

