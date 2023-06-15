
use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};


use crate::{PermissionType};



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
        2) Asset::JsonTemplate{ url_parameter: String} with
                url_parameter = <mintId>
                (on group) url_template = https://metadata.libreplex.io/api/offchaindata/{{mintId}}
        3) Asset::Image{ url: String} with
                url = <custom renderer image API>
        4) Asset::ChainRenderer {program_id: Pubkey} with
                program_id = <address of on-chain rendering program that generates image content>
    
*/
#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Asset {
    None,
    Json {
        url: String
    },
    JsonTemplate {
        url_parameter: String,
    },
    Image {
        url: String,
    },
    ChainRenderer {
        program_id: Pubkey
    },
    Inscription {
        account_id: Pubkey
    }

}


impl Asset {
    pub const BASE_SIZE: usize = 2;
    pub fn get_size(&self) -> usize {
        return Asset::BASE_SIZE + match self {
            Asset::None => 0,
            Asset::Json {
                url
            } => 4 + url.len(),
            Asset::JsonTemplate {
                url_parameter
            } => 4 + url_parameter.len(),
            Asset::Image {
                url
            } => 4 + url.len(),
            Asset::ChainRenderer {
                program_id: _
            } => 32,
            Asset::Inscription {
                account_id: _
            } => 32,

        } 
    }
}



#[repr(C)]
#[account]
pub struct Metadata {
    // the mint address of the token to which the metadata refers
    pub mint: Pubkey,

    pub creator: Pubkey,

    pub is_mutable: bool,

    pub name: String,

    pub symbol: String,

    pub asset: Asset,

    pub description: Option<String>
}

impl Metadata {
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1;

    pub fn get_size(&self) -> usize {
        let size = Metadata::BASE_SIZE
            + 4 + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
            + 1 + match &self.description {
                None=>0,
                Some(x)=>4 + x.len()
            };

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
pub struct CreateMetadataInput {
    pub name: String,
    pub symbol: String,
    pub asset: Asset,
    pub description: Option<String>,
}

impl CreateMetadataInput {
    pub fn get_size(&self) -> usize {
        let size = 
            4 + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
            + match &self.description {
                None =>0,
                Some(x) => 4 + x.len()
            };

        return size;
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMetadataInput {
    pub name: String,
    pub symbol: String,
    pub asset: Asset,
    pub description: Option<String>,
    pub invoked_permission: PermissionType,
}

impl UpdateMetadataInput {
    pub fn get_size(&self) -> usize {
        let size = 4
            + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.asset.get_size()
            + match &self.description {
                None=>0,
                Some(x) => x.len()
            };

        return size;
    }
}



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataEventType {
    Create,
    Update,
    Delete
}

#[event]
pub struct MetadataEvent {
    pub id: Pubkey,
    pub mint: Pubkey,
    pub event_type: MetadataEventType
}