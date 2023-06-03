use std::mem;

use anchor_lang::prelude::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::{CollectionRenderMode};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataRenderModeData {
    NONE {
    },
    PROGRAM {
    },
    URL {
        url: String
    },
}

impl MetadataRenderModeData {
    pub fn get_size(&self) -> usize {
        1 + match self {
            URL => 32,
            _ => 0
        }
    }
    pub fn is_compatible_with(&self,collection_render_mode: &CollectionRenderMode) -> bool {
        match self {
            MetadataRenderModeData::NONE {} => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::NONE) 
            },
            MetadataRenderModeData::PROGRAM {} => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::PROGRAM(Pubkey::default())) 
            },
            MetadataRenderModeData::URL {url} => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::URL(String::default())) 
            }
        }
    }
}


#[repr(C)]
#[account]
pub struct Metadata {

    // the collection to which this metadata belongs
    pub collection_data: Pubkey,

    // the mint address of the token for which the metadata refers
    pub mint: Pubkey,

    pub name: String,

    pub is_mutable: bool,

    pub nft_data: Option<NftMetadata>,

    pub render_mode_data: Vec<MetadataRenderModeData>

}


impl Metadata {

    pub fn get_size(&self) -> usize {

        let size = 8 + 32 + 32 + 36 + 1 + 1 + match &self.nft_data {
            Some(x)=>x.get_size(),
            None=>0
        } 
        + 4 + self.render_mode_data.iter().map(|x|x.get_size()).sum::<usize>();

        return size;
    }

}




#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct NftMetadata {

    pub attributes: Vec<u8>,

    pub signers: Vec<Pubkey>,

}



impl NftMetadata {

    pub fn get_size(&self) -> usize {

        let size = 4 + self.attributes.len() + 4 + self.signers.len();

        return size;
    }

}


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MetadataInput {

    pub name: String,
    pub symbol: String,
    pub render_mode_data: MetadataRenderModeData,
    pub nft_metadata: Option<NftMetadata>,

}

////////////////////////////////////////////////////////////////////////////////////////////////////



impl MetadataInput {

    pub fn get_size(&self) -> usize {

        let name_length = self.name.len();
        let symbol_length = self.symbol.len();
        
        let nft_metadata_length = match self.nft_metadata.as_ref()
        {
            Some (data) => data.get_size(),
            None => 0
        };

        let size = 4 + name_length + 4 + symbol_length + 4 + self.render_mode_data.get_size() + 1 + nft_metadata_length;

        return size;
    }
}
