

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::{Royalties};

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
    
    pub const BASE_SIZE: usize = 2;

    pub fn get_size(&self) -> usize {
        1 
        + match self {
            MetadataExtension::Attributes {attributes} => 4 + attributes.len(),
            MetadataExtension::Signers {signers} => 4 + signers.len() * 32,
            MetadataExtension::Royalties {royalties} => royalties.get_size(),
            MetadataExtension::License {license} => license.get_size(),
            // MetadataExtension::Nft {attributes, signers, royalties, license} =>  
            //   4 + &attributes.len()
            // + 4 + &signers.len() * 32
            // + 1 + match &royalties {
            //     Some(x)=>x.get_size(),
            //     None=>0
            // } 
            // + 1 + match &license {
            //     Some(x) => x.get_size(),
            //     None => 0
            // }
        }
    }
}

// impl MetadataExtension {
    
//     pub const BASE_SIZE: usize = 2;

//     pub fn get_size(&self) -> usize {
//         MetadataExtension::BASE_SIZE 
//         + match self {
//             MetadataExtension::None => 0,
//             MetadataExtension::Nft {attributes, signers, royalties, license} =>  
//               4 + &attributes.len()
//             + 4 + &signers.len() * 32
//             + 1 + match &royalties {
//                 Some(x)=>x.get_size(),
//                 None=>0
//             } 
//             + 1 + match &license {
//                 Some(x) => x.get_size(),
//                 None => 0
//             }
//         }
//     }
// }
