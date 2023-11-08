use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::{MetadataExtension, Royalties};

/*
    Asset replaces URL and provides both backwards compatibility and flexibility

    Examples of asset set-ups for different asset types

    1) SPL tokens:
        Asset::Image{ url: String}
        specifying only an image when JSON is not needed
        OR
        Asset::Json{ url: String}
        specifying a link to static JSON when JSON is needed


    2) 1/1 NFTs:

        Asset::Json{ url: String}
        specifying a static url image pointing to a https:// that hosts/serves the JSON

    2) Large collections:
        JsonTemplate: {url_param: String}
        Specifying a url_param when most of the URL address is repeated for every item in the collection, this optimises rent

    3) Dynamic applications (gaming etc) could be configured in many different ways such
        1) Asset::Json{ url: String} with
                url = https://metadata.libreplex.io/api/offchaindata/<mintId> (or any similar API)
        2) Asset::Image{ url: String} with
                url = <custom renderer image API>
        3) Asset::ChainRenderer {program_id: Pubkey} with
                program_id = <address of on-chain rendering program that generates image content>
        4) Asset::Inscriptions - fully on-chain image
*/
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Asset {
    None,
    Json { url: String },
    Image { url: String, description: Option<String> },
    ChainRenderer { 
        program_id: Pubkey },
    Inscription { 
        // base inscription account id. The base account (and the PDAs derived from it)
        // store the actual on-chain data
        base_data_account_id: Pubkey,
        // inscription stores the "config" of the inscription, such as 
        // authority, rank etc. The intention is to keep the base_data_account as "pure"
        // as possible 
        inscription_id: Pubkey,
        data_type: String,
        description: Option<String>,
           /* 
            Chunks tells you the number of inscription accounts.
            each inscription accounts can hold up to 10MB of data
            so for large media, you may need to split this into chunks.

            each chunk points to an InscriptionData struct which is a
            PDA owned by the inscription program with seed [base_data_account_id, <idx>]
            where idx runs from 0,1,2,3,4 ... N
            and account_id is the pointer to the inscription.

            We keep the total number of chunks here for ease of indexing.

            I.e. we should not need multiple hops to get all the data, just 
            use the chunks to calculate the PDAs off the metadata and then
            fetch all in fell swoop
        */  
        chunks: usize,
    },
}

impl Asset {
    pub const BASE_SIZE: usize = 2;
    pub fn get_size(&self) -> usize {
        Asset::BASE_SIZE
            + match self {
                Asset::None => 0,
                Asset::Json { url } => 4 + url.len(),
                Asset::Image { url , description} => 4 + url.len() + 1 + match &description {Some(x) => 4 + x.len(), None => 0},
                Asset::ChainRenderer { program_id: _} => 32 + 32,
                Asset::Inscription { 
                    base_data_account_id: _,
                    inscription_id: _,
                    data_type,
                    description,
                    chunks: _,
                } => 32 // base_data_account_id 
                + 32 // inscription_id
                + 4 + data_type.len()
                + 1 + match description {
                    Some(x) => 4 + x.len(),
                    None => 0 
                }
                + 4 // chunks
            }
    }
}

#[account]
pub struct Metadata {
    // the mint address of the token to which the metadata refers
    pub mint: Pubkey,

    pub update_authority: Pubkey,

    // First ever creator cannot be changed
    pub creator: Pubkey,

    pub is_mutable: bool,

    pub collection: Option<Pubkey>,

    pub name: String,

    pub symbol: String,

    pub asset: Asset,

    pub extensions: Vec<MetadataExtension>,
}

impl Metadata {
    pub const BASE_SIZE: usize = 8 
        // mint
        + 32 
        // ua
        + 32 
        // creator
        + 32 
        // is mutable
        + 1 
        // collection
        + 1 + 32;

    pub fn get_size(&self) -> usize {
        

        Metadata::BASE_SIZE
            + 4
            + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
            + 4
            + self.extensions.iter().map(|x|x.get_size()).sum::<usize>()
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
        

        4 + self.attributes.len()
    }
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MetadataExtensionInput {
    pub attributes: Vec<u8>, // base: 4
    pub royalties: Option<Royalties>,
}

impl MetadataExtensionInput {
    pub const BASE_SIZE: usize = 4 + 1 + 1;

    pub fn get_size(&self) -> usize {
        MetadataExtensionInput::BASE_SIZE
            + self.attributes.len()
            + match &self.royalties {
                Some(x) => x.get_size(),
                None => 0,
            }
    }
}

pub fn validate_extend_metadata_input(metadata_input: &MetadataExtensionInput) -> Result<()> {
    match &metadata_input.royalties {
        Some(royalties) => {
            let total_shares: u16 = royalties.shares.iter().map(|x| x.share).sum();
            if total_shares != 10000 {
                return Err(ErrorCode::RoyaltiesBadSum.into());
            }
        }
        None => {}
    }
    Ok(())
}



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateMetadataInput {
    pub name: String,
    pub symbol: String,
    pub asset: Asset,
    pub update_authority: Pubkey,
    pub extensions: Vec<MetadataExtension>,
}



impl CreateMetadataInput {
    pub fn get_size(&self) -> usize {
            4
            + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
            + self.extensions.iter().map(|x|x.get_size()).sum::<usize>()
            + 4 
            // sum of sizes of extensions
            + self.extensions.iter().map(|x|x.get_size())
                .sum::<usize>()
    }
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMetadataInput {
    pub name: String,
    pub symbol: String,
    pub asset: Asset,
}

impl UpdateMetadataInput {
    pub fn get_size(&self) -> usize {
        4
            + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
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
