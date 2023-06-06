use std::mem;

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use prog_common::errors::ErrorCode;

use crate::{Collection, MAX_NAME_LENGTH, PermissionType};

use crate::CollectionRenderMode;

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataRenderModeData {
    None,
    Program { program_id: Pubkey },
    Url { url: String },
}

impl MetadataRenderModeData {
    pub fn get_size(&self) -> usize {
        2 + match self {
            
            MetadataRenderModeData::Url { url } => 4 + url.len(),
            MetadataRenderModeData::Program{program_id:_}=>32,
            MetadataRenderModeData::None => 0
        }
    }

    pub fn is_compatible_with(&self, collection_render_mode: &CollectionRenderMode) -> bool {
        match self {
            MetadataRenderModeData::None {} => {
                return mem::discriminant(collection_render_mode) == mem::discriminant(&CollectionRenderMode::None{})
            },
            MetadataRenderModeData::Program { program_id: _ } => {
                return mem::discriminant(collection_render_mode)
                    == mem::discriminant(&CollectionRenderMode::Program {
                        program_id: Pubkey::default(),
                    })
            }
            MetadataRenderModeData::Url { url:_ } => {
                return mem::discriminant(collection_render_mode)
                    == mem::discriminant(&CollectionRenderMode::Url {
                        url: String::default(),
                    })
            }
        }
    }
}

#[repr(C)]
#[account]
pub struct Metadata {
    
    // the mint address of the token for which the metadata refers
    pub mint: Pubkey,

    pub creator: Pubkey,

    pub is_mutable: bool,

    /// from input - variable size
    pub name: String,

    /// from input - variable size
    pub symbol: String,

    /// from input - variable size
    pub url: String,

    pub description: Option<String>
}

impl Metadata {
    
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1;
    

    pub fn get_size(&self) -> usize {
        let size = Metadata::BASE_SIZE
            + 4 + self.name.len()
            + 4 + self.symbol.len()
            + 4 + self.url.len();

        return size;
    }
}


#[repr(C)]
#[account]
pub struct MetadataExtended {
    
    pub nft_metadata: Option<NftMetadata>,

    pub render_mode_data: Vec<MetadataRenderModeData>,
}

impl MetadataExtended {
    
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1 
    + 4  // because MetadataRanderModeData is wrapped in a vector
    + 4; // because nft_metadata.signers gets initialised to an empty vec

    pub fn get_size(&self) -> usize {
        let size = Metadata::BASE_SIZE
            + 1 + match &self.nft_metadata {
                Some(x) => x.get_size(),
                None => 0,
            }
            + 4 + self
                .render_mode_data
                .iter()
                .map(|x| x.get_size())
                .sum::<usize>();

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

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AttributesInput {
    pub attributes: Vec<u8>,

    /*
        we purposefully omit the signers from here
        as every metadata starts its life with no
        signers. signatures are added separately
        in accordance with permitted signers
    */

}

impl AttributesInput {
    pub fn get_size(&self) -> usize {
        let size = 4 + self.attributes.len();

        return size;
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MetadataInput {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub description: Option<String>,
    pub invoked_permission: PermissionType
}

impl MetadataInput {
    pub fn get_size(&self) -> usize {
        let size = 4 + self.name.len()
        + 4 + self.symbol.len()
        + 4 + self.url.len()
        + 1 + match &self.description {
            Some(x) => x.len(),
            None => 0
        }
        ;

        return size;
    }
}


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMetadataExtendedInput {
    pub render_mode_data: MetadataRenderModeData,
    pub nft_metadata: Option<AttributesInput>,
    pub invoked_permission: PermissionType,
}


pub fn validate_metadata_input(
    metadata_input: &UpdateMetadataExtendedInput,
    collection: &Collection,
) -> Result<()> {
    let UpdateMetadataExtendedInput {
        invoked_permission,
        render_mode_data,
        nft_metadata:_,
    } = metadata_input;

    /*
        ensure that the initial render mode of the metadata matches the
        currently active render mode of the collection.

        NB: It is possible to change the active render mode of the collection.
        If that happens, it is the responsibility of the update auth holder
        to add the appropriate render mode data to each metadata.

    */

    
    render_mode_data.is_compatible_with(&collection.collection_render_mode);

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    
    Ok(())
}



////////////////////////////////////////////////////////////////////////////////////////////////////

impl UpdateMetadataExtendedInput {

   
    pub fn get_size(&self) -> usize {
        let size = 4
            + self.render_mode_data.get_size()
            + 1
            + match self.nft_metadata.as_ref() {
                Some(data) => data.get_size(),
                None => 0,
            };

        return size;
    }
}