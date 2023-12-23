
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

/*
    basic metadata struct. describes the minimal usecase that is usually
    enough for spl tokens.  any extensions (nfts / royalties etc) will be
    tagged on as PDAs. because there are no protocol fees and the rent is
    otherwise minimised, this solution is still extremely rent-effective.

    this base struct is also enough to produce an erc-721 compliant
    representation of the metadata
 */
#[account]
pub struct Metadata {
    // the mint address of the token to which the metadata refers
    pub mint: Pubkey,

    // 1111111111111111.... means immutable in keeping with inscriptions
    pub update_authority: Pubkey,

    // this may not be needed with token 22 as mint groups will be
    // catered for. Unfortunately this is a key field for indexers 
    // so we need to place it before name, symbol etc variable-length strings.
    // set to 11111111111111.... for no collection
    pub collection: Pubkey,

    pub name: String,

    // because it's a URL (not a uri) and points to an ERC-721 compliant JSON
    pub url_json: String,

    pub symbol: String,

    // plus padding

    // any future extensions are PDAs off the MINT - never the Metadata.
    // to eliminate any double hops.

    // examples of extensions:
    // 1) nft (royalties, creators, on-chain attributes etc)
    // 2) licensing 
    // 3) dynamic rendering + other asset types


}

impl Metadata {
    pub const BASE_SIZE: usize = 8 
        // mint
        + 32 
        // ua
        + 32 
        // collection
        + 32;
        
    pub fn get_size(&self) -> usize {
        

        Metadata::BASE_SIZE
            + 4
            + self.name.len()
            + 4
            + self.url_json.len()
            + 4
            + self.symbol.len()
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateMetadataInput {
    pub update_authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub url_json: String
}



impl CreateMetadataInput {
    pub fn get_size(&self) -> usize {
            4
            + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.url_json.len()
    }
}


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataEventType {
    Create,
    Update,
    Delete,
}

#[event]
pub struct MetadataEvent {
    pub id: Pubkey,
    pub mint: Pubkey,
    pub event_type: MetadataEventType,
}
