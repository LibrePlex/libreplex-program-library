use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

pub const ROOT_TYPE_LIMIT: usize = 25;
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

    // Can be anything gives the consumer an idea of what type of root you have provided.
    // Usually SPL_MINT or SPL_MINT_2022
    #[max_len(ROOT_TYPE_LIMIT)]
    pub root_type: String,

    #[max_len(400)]
    pub padding: Vec<u8>,
}