use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

pub const TICKER_LIMIT: usize = 200;
pub const ROOT_TYPE_LIMIT: usize = 25;
// pub const EXCESS: usize = 7000;
pub const TEMPLATE_LIMIT: usize = 1200;

#[account]
#[derive(InitSpace)]
pub struct TokenDeployment {
    pub creator: Pubkey,
    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
    pub collection_mint: Pubkey,
    pub number_of_tokens_issued: u64,

    // Can be anything gives the consumer an idea of what type of root you have provided.
    // Usually SPL_MINT or SPL_MINT_2022
    #[max_len(ROOT_TYPE_LIMIT)]
    pub root_type: String,

    #[max_len(TICKER_LIMIT)]
    pub ticker: String, 

    #[max_len(TEMPLATE_LIMIT)]
    pub deployment_template: String,
    
    #[max_len(TEMPLATE_LIMIT)]
    pub mint_template: String,

    // pub padding: Vec<u8, EXCESS>
    
}  






