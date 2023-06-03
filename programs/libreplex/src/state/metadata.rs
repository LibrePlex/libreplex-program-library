
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
    // this can we made dynamic w/ realloc
    pub image_url: String, 
    // could we reduce the size to say max 8 characters?
    pub symbol: String,
    // this can we made dynamic w/ realloc and avoid the annoying padding
    pub name: String,
}

impl Metadata {
    // fixed size for now. TODO: replace with base size 
    pub const SIZE: usize = 8 + 32 + 32 + 1 + 1 + 36 + 36 + 36;
}

/* 
    MetadataNFT address creators and attributes to the base
    Metadata struct. These are not needed for all mints (such as SPL tokens)
    and hence we are keeping under a separate PDA
 */

pub const MAX_CREATOR_COUNT: usize = 5; // conform to the existing standard for now.

#[account]
pub struct MetadataNft {
    // need to agree on best way to handle creators
    // creators should be searchable via PDA
    // so the position of each creator should be known
    // and the max size of the vector should be capped
    pub creators: Vec<Creator>,
    pub collection: Option<Pubkey>,
    pub bump: u8,

    // there can be any number of attributes.
    // Should therefore be dynamically sized.
    pub attributes: Vec<Attribute>,
}

impl MetadataNft {
    pub const BASE_SIZE: usize = 8 
        + MAX_CREATOR_COUNT * Creator::SIZE 
        + 1 + 32
        + 1;
}
