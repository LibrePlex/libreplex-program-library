use anchor_lang::prelude::*;

#[repr(C)]
#[account]
#[derive(Debug)]
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
#[account]
#[derive(Debug)]
pub struct NftMetadata {

    pub attributes: Vec<Attribute>,

    pub signers: Vec<Pubkey>,

}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct Attribute {

    pub trait_type: String,

    pub attribute: String,

}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct MetadataInput {

    pub name: String,
    pub symbol: String,
    pub metadata_url: String,
    pub nft_metadata: Option<NftMetadata>,

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Attribute {

    pub fn get_size(&self) -> usize {

        let trait_type_length = self.trait_type.len();
        let attribute_length = self.attribute.len();

        let size = 4 + trait_type_length + 4 + attribute_length;

        return size;
    }
}

impl NftMetadata {

    pub fn get_size(&self) -> usize {

        let mut total_attribute_size = 0;

        for attribute in self.attributes.iter() {

            let attribute_size = attribute.get_size();
            total_attribute_size += attribute_size;
        }

        let size = 4 + total_attribute_size + 4 + self.signers.len();

        return size;
    }

}

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
