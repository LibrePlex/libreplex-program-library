use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;
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

    // responsibility of the deployer to set this correctly.
    // also, responsibility of the deployer to ensure that 
    // the associated token account (owned by deployment)
    // contains sufficient tokens for the swap
    pub fungible_mint: Pubkey,
  
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

impl DeploymentRaw {
    pub fn get_base_amount_per_mint(&self, fungible_mint: &Mint) -> u64 {
        self.limit_per_mint
        .checked_mul(10_u64.checked_pow(fungible_mint.decimals as u32).unwrap())
        .unwrap()
    }
}
