use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

// to allow for expansion of the summary later as needed

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum SummaryExtension {
    None,
}

#[account]
pub struct MetadataSummary {
    pub metadata_count_total: u64,
    pub last_metadata_mint: Pubkey,
    pub last_metadata_creator: Pubkey,
    pub last_metadata_create_time: i64,
    pub extension: SummaryExtension,
}

impl MetadataSummary {
    pub const BASE_SIZE: usize = 8 + 8 + 32 + 32 + 8 + 2;
}