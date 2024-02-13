use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

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

#[account]
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
    pub auth_program_id: Pubkey  // system program for none
    
}

impl Pipeline {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 2 + 32 + 8 + 8 + 100;
}




