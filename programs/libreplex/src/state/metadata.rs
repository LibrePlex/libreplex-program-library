use std::mem;

use anchor_lang::prelude::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use prog_common::{errors::ErrorCode};

use crate::{MAX_NAME_LENGTH, Collection};

use crate::{CollectionRenderMode};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataRenderModeData {
    Program {
        program_id: Pubkey
    },
    Url {
        url: String
    },
}

impl MetadataRenderModeData {
    pub fn get_size(&self) -> usize {
        1 + match self {
            MetadataRenderModeData::Url {url: _} => 32,
            _ => 0
        }
    }
    pub fn is_compatible_with(&self,collection_render_mode: &CollectionRenderMode) -> bool {
        match self {
            // MetadataRenderModeData::None {} => {
            //     return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::None{}) 
            // },
            MetadataRenderModeData::Program {
                program_id: _
            } => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::Program{program_id: Pubkey::default()}) 
            },
            MetadataRenderModeData::Url {url} => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::Url{collection_url: String::default()}) 
            }
        }
    }
}


#[repr(C)]
#[account]
pub struct Metadata {

    // the collection to which this metadata belongs
    pub collection: Pubkey,

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
    pub render_mode_data: MetadataRenderModeData,
    pub nft_metadata: Option<NftMetadata>,
}


pub fn validate_metadata_input(metadata_input: &MetadataInput, collection: &Collection) -> Result<()> {
    let MetadataInput {name, render_mode_data, nft_metadata: _} = metadata_input;



    /* 
        ensure that the initial render mode of the metadata matches the 
        currently active render mode of the collection.

        NB: It is possible to change the active render mode of the collection.
        If that happens, it is the responsibility of the update auth holder
        to add the appropriate render mode data to each metadata.

    */ 
    
    render_mode_data.is_compatible_with(&collection.collection_render_mode);

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    
    if name_length > MAX_NAME_LENGTH  {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////



impl MetadataInput {

    pub fn get_size(&self) -> usize {

        let name_length = self.name.len();


        let nft_metadata_length = match self.nft_metadata.as_ref()
        {
            Some (data) => data.get_size(),
            None => 0
        };

        let size = 4 + name_length + 4 + self.render_mode_data.get_size() + 1 + nft_metadata_length;

        return size;
    }
}
