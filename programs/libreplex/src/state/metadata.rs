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
