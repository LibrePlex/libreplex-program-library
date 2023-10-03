

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::Royalties;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum License {
    NoLicense,
    Custom {
        license_url: String
    }
}

impl License {
    pub fn get_size(&self)-> usize {
        2 + match &self {
            License::NoLicense => 0,
            License::Custom { license_url } => 4 + license_url.len()
        }
    }
}

// #[derive(Clone, AnchorDeserialize, AnchorSerialize)]
// pub enum MetadataExtension {
//     None,
//     Nft {
//         attributes: Vec<u8>, // base: 4
//         signers: Vec<Pubkey>, // base: 4
//         royalties: Option<Royalties>, // base: 4
//         license: Option<License>,
//     }
// }



#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataExtension {
    Attributes {attributes: Vec<u8>}, // base: 4
    Signers {signers: Vec<Pubkey>}, // base: 4
    Royalties {royalties: Royalties}, // base: 4
    License {license: License},
    /* can extend if needed */
}



impl MetadataExtension {
    
    pub fn get_size(&self) -> usize {
        2
        + match self {
            MetadataExtension::Attributes {attributes} => 4 + attributes.len(),
            MetadataExtension::Signers {signers} => 4 + signers.len() * 32,
            MetadataExtension::Royalties {royalties} => royalties.get_size(),
            MetadataExtension::License {license} => license.get_size(),
        }
    }
}

