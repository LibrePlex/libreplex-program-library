use anchor_lang::prelude::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[repr(C)]
#[account]
pub struct Metadata {

    // the collection to which this metadata belongs
    pub collection: Pubkey,

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
    pub symbol: String,
    pub metadata_url: String,
    pub nft_metadata: Option<NftMetadata>,

}

////////////////////////////////////////////////////////////////////////////////////////////////////



impl MetadataInput {

    pub fn get_size(&self) -> usize {

        let name_length = self.name.len();
        let symbol_length = self.symbol.len();
        let url_length = self.metadata_url.len();

        let nft_metadata_length = match self.nft_metadata.as_ref()
        {
            Some (data) => data.get_size(),
            None => 0
        };

        let size = 4 + name_length + 4 + symbol_length + 4 + url_length + 1 + nft_metadata_length;

        return size;
    }
}
