use anchor_lang::prelude::*;

#[repr(C)]
#[account]
#[derive(Debug)]
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
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NftMetadata {

    pub attributes: Vec<Attribute>,

    pub signers: Vec<Pubkey>,

}

#[repr(C)]
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Attribute {

    pub trait_type: String,

    pub attribute: String,

}

#[repr(C)]
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
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

        let size = (4 + trait_type_length) + (4 + attribute_length);

        return size;
    }
}

impl NftMetadata {

    pub fn get_size(&self) -> usize {

        let signers_length = self.signers.len();
        let total_attribute_size: usize = self.attributes.iter().map(|x| x.get_size()).sum();

        let size = (4 + total_attribute_size) + (4 + 32*signers_length);

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

        let size = (4 + name_length) + (4 + symbol_length) + (4 + url_length) + (1 + nft_metadata_length);

        return size;
    }
}
