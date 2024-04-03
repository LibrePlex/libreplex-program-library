use anchor_lang::prelude::*;

use solana_program::pubkey::Pubkey;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
#[derive(InitSpace)]
enum CosignerType {
    Mint,
    SwapToNft,
    SwapToSpl
}


use crate::{OFFCHAIN_URL_LIMIT, TICKER_LIMIT};

/* 
    a stripped-down variant of the original deployment.  does the following
    1) manages hashlists
    2) allows escrow swaps 
    3) 
*/
#[account]
#[derive(InitSpace)]
pub struct DeploymentRaw {
    
    pub creator: Pubkey,

    pub limit_per_mint: u64,
    
    pub max_number_of_tokens: u64,

    pub number_of_tokens_issued: u64,
    
    
    // this is used to sanity check that
    // whenever swaps occur, to the maount
    // of fungible and non-fungible in the
    // escrow always remains equal to the total
    // supply.
    pub escrow_non_fungible_count: u64,

    #[max_len(TICKER_LIMIT)]
    pub ticker: String,

    pub fungible_mint: Pubkey, // starts as 111111111111...
  
    #[max_len(OFFCHAIN_URL_LIMIT)]
    pub offchain_url: String, // pub padding: Vec<u8, EXCESS>

    // information only to help route handling above the IDL
    pub proxy_program_id: Pubkey,

    // to allow modular custom logic around this contract.
    // 1111111111111... means no cosigner required
    // to control mint
    pub cosigner_mint: Pubkey,

    pub cosigner_swap_to_nft: Pubkey,

    pub cosigner_swap_to_spl: Pubkey,

    // just in case
    pub padding: [u8; 200]
}
