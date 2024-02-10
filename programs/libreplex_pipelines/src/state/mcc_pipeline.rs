use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};



#[account]
pub struct MccPipeline {
    pub fair_launch_deployment: Pubkey,
    pub liquidity: Pubkey,
    pub processed_item_count: u64,
    pub collection: Pubkey,
    pub creation_time: i64,
    pub bump: u8 // for, you know, signing and stuff
    
}

impl MccPipeline {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 1 + 100;
}

