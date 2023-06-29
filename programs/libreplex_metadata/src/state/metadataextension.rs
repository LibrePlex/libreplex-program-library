

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::{Royalties, Metadata};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum License {
    NoLicense,
    Custom {
        license_url: String
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataExtension {
    None,
    Nft {
        attributes: Vec<u8>, // base: 4
        signers: Vec<Pubkey>, // base: 4
        royalties: Option<Royalties>, // base: 4
    }
}


impl MetadataExtension {
    
    pub const BASE_SIZE: usize = 2;

    pub fn get_size(&self) -> usize {
        MetadataExtension::BASE_SIZE 
        + match self {
            MetadataExtension::None => 0,
            MetadataExtension::Nft {attributes, signers, royalties} =>  
            &attributes.len()
            + &signers.len() * 32
            + match &royalties {
                Some(x)=>x.get_size(),
                None=>0
            }
        }
           
         

        
    }
}
