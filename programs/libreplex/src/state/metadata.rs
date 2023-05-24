
use anchor_lang::prelude::*;


#[account]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8, // percentage 
}

impl Creator {
    pub const SIZE: usize = 8 + 32 + 1 + 1;
}


#[account]
pub struct Verification {
    bump: u8
}

impl Verification {
    pub const SIZE: usize = 8 + 32 + 1 + 1;
}


#[account]
#[derive(Copy)]
pub struct CollectionData {
    pub address: Pubkey,
    pub verified: bool,
}

impl CollectionData {
    pub const SIZE: usize = 8 + 32 + 1;
}

#[account]
pub struct Attribute {
    pub trait_type: String,
    pub attribute: String,
}

#[account]
pub struct Metadata {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub is_mutable: bool,
    pub bump: u8,
    // first of the string fields as this 
    // could be used for search
    pub symbol: String,
    pub offchain_url: String, 
    pub name: String,
}

impl Metadata {
    // base size only, the size is set at init / reallocated at update based on
    // the lengths of the strings
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1 + 1;
}

/* 
    MetadataNFT address creators and attributes to the base
    Metadata struct. These are not needed for all mints (such as SPL tokens)
    and hence we are keeping under a separate PDA
*/
#[account]
pub struct MetadataNft {
    // Set to 11111111111111111111111111111111 for no collection
    // this saves 1 byte over using an Option and still keeps
    // things nice and searchable
    pub collection: Option<CollectionData>,

    // need to agree on best way to handle creators
    // creators should be searchable via PDA
    // so the position of each creator should be known
    // and the max size of the vector should be capped
    // dictate royalty payments
    pub creators: Option<Vec<Creator>>, 
    
    pub bump: u8,

    // there can be any number of attributes.
    // Should therefore be dynamically sized.
    pub attributes: Vec<Attribute>,
}

impl MetadataNft {
    pub const BASE_SIZE: usize = 8 
        + 1 + 32
        + 1;
}

/* Rationale for collection object:
    1) creators are typically set on a per collection basis
    2) collection is not actually mint in that it should not be movable
        between wallets, have mastereditions / creators / attributes etc.    
        hence leaving name + symbol + image url here as strings only
    3) dynamically resizes according to the length of the fields
    4) place creators at the beginning so they can be fetched via gPA 
        (if really needed :D otherwise use helios.xyz or alchemy etc)
*/
#[account]
pub struct Collection {
    pub authority: Pubkey,
    pub creators: Vec<Creator>,
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub bump: u8
}
