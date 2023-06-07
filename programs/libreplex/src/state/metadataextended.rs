

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::instructions::Royalties;




#[repr(C)]
#[account]
pub struct MetadataExtended {
    
    // base: 8 (discriminator)

    pub group: Pubkey, // base: 32

    pub metadata: Pubkey, // base: 32

    pub attributes: Vec<u8>, // base: 4

    pub signers: Vec<Pubkey>, // base: 4

    pub royalties: Option<Royalties>, // base: 4
 
}

impl MetadataExtended {
    
    pub const BASE_SIZE: usize = 8 + 32 + 32  
    + 4 // attributes 
    + 4 // signers
    + 1; // royalties

    pub fn get_size(&self) -> usize {
        MetadataExtended::BASE_SIZE
            + &self.attributes.len()
            + &self.signers.len() * 32
            + match &self.royalties {
                Some(x)=>x.get_size(),
                None=>0
            }

        
    }
}
