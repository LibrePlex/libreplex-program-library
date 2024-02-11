use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};



// tells you if a mint has already been processed for a given pipeline
#[account]
pub struct PipelineSwapMarker {
    pub pipeline: Pubkey,
    pub incoming_mint: Pubkey,
}

impl PipelineSwapMarker {
    pub const SIZE: usize = 8 + 32 + 32;
}

