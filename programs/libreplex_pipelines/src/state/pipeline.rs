use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
pub const HASHLIST_URL_LIMIT: u32 = 300;
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Filter {
    MCC {
        collection_id: Pubkey
    },
    FirstCreatorId {
        first_creator_id: Pubkey
    },
    Hashlist {
        root: [u8; 32]
    }
}

impl anchor_lang::Space for Filter  {
    const INIT_SPACE: usize = 100;
}

#[account]
#[derive(InitSpace)]
pub struct Pipeline {
    pub fair_launch_deployment: Pubkey,
    pub liquidity: Pubkey,
    pub auth: Pubkey,
    pub processed_item_count: u64,
    pub creation_time: i64,
    pub bump: u8,  // for, you know, signing and stuff,
    pub filter: Filter,
    // paid to liquidity provider directly when they add liquidity
    pub liquidity_provider_amount_in_spl: u64,
    pub fungible_chunk_count: u64,
    // only consider amounts added by the pipelines program.
    // if somebody else sends tokens to this address,
    // we ignore it. net of amounts put into swaps
    pub fungible_amount_net: u64,
    // 
    pub fungible_amount_total: u64,
    // number of swaps created - each swap can have been utilised
    // multiple times but this program does not count those
    pub created_swap_count: u64,
    pub auth_program_id: Pubkey,  // system program for none
    // the amount of SPL that you receive when you first swap in an element
    // from the incoming collection. calculated upfront for convenience
    // and to make sure there are no rounding errors later
    pub spl_swap_amount_primary: u64,

    // the amount for secondary swaps. secondary swap amount is always
    // higher than the primary swap (accounting for the existence of the 
    // SPL tokens distributed to the liquidity providers)
    pub spl_swap_amount_secondary: u64,
    
    pub require_cosigner: bool,

    #[max_len(HASHLIST_URL_LIMIT)]
    pub hashlist_url: String, // pub padding: Vec<u8, EXCESS>
    
}





