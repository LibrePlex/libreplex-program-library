use anchor_lang::prelude::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use prog_common::{errors::ErrorCode};

use crate::{MAX_NAME_LENGTH, MAX_URL_LENGTH};

#[repr(C)]
#[account]
pub struct Metadata {

    // the collection to which this metadata belongs
    pub collection_data: Pubkey,

    // the mint address of the token for which the metadata refers
    pub mint: Pubkey,

    pub name: String,

    pub url: String,

    pub is_mutable: bool,

    pub nft_data: Option<NftMetadata>,
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
    pub metadata_url: String,
    pub nft_metadata: Option<NftMetadata>,
}


pub fn validate_metadata_input(metadata_input: &MetadataInput) -> Result<()> {
    let MetadataInput {name, metadata_url, nft_metadata: _} = metadata_input;

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let url_length = metadata_url.len();

    if name_length > MAX_NAME_LENGTH  || url_length > MAX_URL_LENGTH {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////



impl MetadataInput {

    pub fn get_size(&self) -> usize {

        let name_length = self.name.len();

        let url_length = self.metadata_url.len();

        let nft_metadata_length = match self.nft_metadata.as_ref()
        {
            Some (data) => data.get_size(),
            None => 0
        };

        let size = 4 + name_length + 4 + url_length + 1 + nft_metadata_length;

        return size;
    }
}
