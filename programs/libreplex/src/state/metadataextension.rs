

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::Royalties;

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum License {
    NoLicense,
    Custom {
        license_url: String
    }
}


#[repr(C)]
#[account]
pub struct MetadataExtension {
    
    // base: 8 (discriminator)

    pub group: Pubkey, // base: 32

    pub metadata: Pubkey, // base: 32

    pub attributes: Vec<u8>, // base: 4

    pub signers: Vec<Pubkey>, // base: 4

    pub royalties: Option<Royalties>, // base: 4

    pub license: Option<License>
 
}

impl MetadataExtension {
    
    pub const BASE_SIZE: usize = 8 + 32 + 32  
    + 4 // attributes 
    + 4 // signers
    + 1; // royalties

    pub fn get_size(&self) -> usize {
        MetadataExtension::BASE_SIZE
            + &self.attributes.len()
            + &self.signers.len() * 32
            + match &self.royalties {
                Some(x)=>x.get_size(),
                None=>0
            }
            + match &self.license {
                None => 0,
                Some(license) => 
                    match license {
                        License::NoLicense => 0,
                        License::Custom {license_url} => 4 + license_url.len()
            }
        }

        
    }
}
