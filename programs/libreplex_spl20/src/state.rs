use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

pub const TICKER_LIMIT: usize = 50;
pub const EXCESS: usize = 7000;

#[account]
#[derive(InitSpace)]
pub struct TokenDeployment {
    pub creator: Pubkey,
    pub limit: u64,
    pub max: u64,
    pub collection: Pubkey,

    #[max_len(TICKER_LIMIT)]
    pub ticker: String,

    pub root: Pubkey,

    #[max_len(400)]
    pub padding: Vec<u8>,
}