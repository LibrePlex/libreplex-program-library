


use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};


#[account]
pub struct AttributeConfig {
    pub authority: Pubkey,
    pub current: u32,
    // this is to ensure that each attributemapping has the same length
    // for accessing with AccountLoader / working with slices
    pub max_onchain_attribute_count: u32,
    // if we are using onchain description 
    
    // Inline data as slices
    //pub attributes: Vec<Vec<u8>>
}




