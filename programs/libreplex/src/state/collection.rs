use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum CollectionRenderMode {
    /*
        Pubkey here is the address of the rendering program
        * BETA functionality (to be validated against
            validator resourcing limitations) *

        the id of the external COLLECTION rendering program
        that implements the LibrePlex rendering
        interface standard
        the interface will have the following
        signature (roughly, still in discussions):

        to obtain a rendering, it is enough
        to simulate the transaction.

    input accounts:
        1) collection

        output:
        1) JSON
        2) image URL (including ordinal / base64)
        3) something else
    */
    None,
    Program{program_id: Pubkey},
    Url{url: String},
}

impl CollectionRenderMode {
    pub fn get_size(&self) -> usize {
        let _size = 2 // discriminator size
        // + 32 // max variant size (from Pubkey)
        + match self {
            CollectionRenderMode::None => 0,
            CollectionRenderMode::Url{url} => 4 + url.len(),
            CollectionRenderMode::Program{program_id:_} => 32
        };

        msg!("Collection render mode size {}", _size);
        return _size;
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataRenderMode {
    None,
    /*
        Pubkey here is the address of the rendering program
        * BETA functionality (to be validated against
            validator resourcing limitations) *

        the id of the external METADATA rendering program
        that implements the LibrePlex rendering
        interface standard
        the interface will have the following
        signature (roughly, still in discussions):

        to obtain a rendering, it is enough
        to simulate the transaction.

        input accounts:
        1) collection
        2) metadata
        3) mint
        4) (optional) token account

        output:
        1) JSON
        2) image URL (including ordinal / base64)
        3) something else
    */
    Program {
        program_id: Pubkey,
    },
    Url {
        base_url_configuration: Option<BaseUrlConfiguration>,
    },
}

impl MetadataRenderMode {
    pub fn get_size(&self) -> usize {
        2 + match self {
            MetadataRenderMode::None => 1,
            MetadataRenderMode::Url {
                base_url_configuration,
            } => {
                1 + match base_url_configuration {
                    Some(base_config) => {
                        4 + 4 + base_config.prefix.len() + base_config.suffix.len()
                    }
                    None => 0,
                }
            }
            MetadataRenderMode::Program { program_id: _ } => 32,
        }
    }
}

#[repr(C)]
#[account]
pub struct Collection {
    // Seed address used to generate unique account PDA address
    pub seed: Pubkey,

    // Creator does not convey any privileges
    pub creator: Pubkey,

    // the number of items in collection
    pub item_count: u32,


    /* variable length fields = match 1-1 with CollectionInput */
    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    pub description: String,

    pub collection_render_mode: CollectionRenderMode,

    pub metadata_render_mode: MetadataRenderMode,

    // for NFT collections
    
    pub nft_collection_data: Option<NftCollectionData>,

    
}



impl Collection {

    pub const BASE_SIZE: usize  = 8 + 32 + 32 + 4; // anchor + seed + creator + item count

    pub fn get_size(&self) -> usize {
        Collection::BASE_SIZE
        + 4 + self.name.len() // name
        + 4 + self.symbol.len() // symbol
        + 4 + self.description.len() // symbol
        + self.collection_render_mode.get_size()
        + self.metadata_render_mode.get_size()
        + match &self.nft_collection_data {
            Some(x) => x.get_size(),
            None => 0,
        }
    }
}


// collection input: 8 + 

// pub name: String, 
// pub symbol: String,
// pub collection_render_mode: CollectionRenderMode,
// pub metadata_render_mode: MetadataRenderMode,
// pub nft_collection_data: Option<NftCollectionData>,



// impl CollectionInput {
//     pub fn get_size(&self) -> usize {
//         let name_length = self.name.len();
//         let symbol_length = self.symbol.len();

//         let nft_data_length = match self.nft_collection_data.as_ref() {
//             Some(data) => data.get_size(),
//             None => 0,
//         };

//         let size = 4
//             + name_length
//             + 4
//             + symbol_length
//             + 4
//             + self.collection_render_mode.get_size()
//             + self.metadata_render_mode.get_size()
//             + 1
//             + nft_data_length;

//         return size;
//     }
// }


/*

   Attribute type is the normalisation of attributes so
   that individual items only contain vectors of indices
   that point to the human-readable names on the collection.

   The basic assumption is that
   most attribute types will have no more than 256 permitted
   values.

   if an attribute type has more than 256 values,
   then we add a new attribute type to the attribute type vector
   on NftCollectionData *WITH THE SAME NAME* as the first attribute
   type of the same kind.

   This allows for virtually infinite number of values per
   attribute type in those special cases that should require it.

   If an attribute type is extended in this way, then the attribute
   value index at position 255 points to the next index of the attribute
   **TYPE** on the collection.

   (The last point needs better documentation)

*/

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum AttributeValue {
    None,
    String {value: String},
    U8 {value: u8},
    U16 {value: u16},
    U32 {value: u32},
    U64 {value: u64},
    I8 {value: i8},
    I16 {value: i16},
    I32 {value: i32},
    I64 {value: i64},
    
    //HedgeHog{ value: HedgeHog} new custom types can be added as needed
}

impl AttributeValue {
    pub fn get_size(&self) -> usize {
        2 + match &self {
            AttributeValue::None => 0,
            AttributeValue::U8 {value: _}=> 1,
            AttributeValue::I8 {value: _}=> 1,
            AttributeValue::U16{value: _} => 2,
            AttributeValue::I16 {value: _}=> 2,
            AttributeValue::U32{value: _} => 4,
            AttributeValue::I32 {value: _}=> 4,
            AttributeValue::U64{value: _} => 8,
            AttributeValue::I64 {value: _}=> 8,
            AttributeValue::String{value} => 4 + value.len()
        }
    }
}

/*
    #0 BACKGROUND - "blue", "red", ...., "green"
    #1 FACE - "angry", "sad",
    #2 GLOVE - "metal", "riding"
    #3 BACKGROUND - "yellow", "black"
*/

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AttributeType {
    // royalty address and their share in basis points (0-10,000)
    pub name: String,

    pub permitted_values: Vec<AttributeValue>,

    pub deleted: bool,

    // pointer to the next attribute type in case of attribute value overflow (>255)
    pub continued_at_index: Option<u32>,

    // pointer to the previous attribute type in case of attribute value overflow (>255)
    pub continued_from_index: Option<u32>,
}

impl AttributeType {
    pub fn get_size(&self) -> usize {
        let total_size: usize = self.permitted_values.iter().map(|x| 4 + x.get_size()).sum();

        return 4 + 32  // name
            +  4 + total_size 
            + 1 // deleted
            + 1 
            + match self.continued_at_index { // continued_at_index
                Some(_)=>4,
                None => 0
            }
            + match self.continued_from_index { // continued_at_index
                Some(_)=>4,
                None => 0
            }
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct BaseUrlConfiguration {
    // royalty address and their share in basis points (0-10,000)
    pub prefix: String,

    pub suffix: String,
}

impl BaseUrlConfiguration {
    pub fn get_size(&self) -> usize {
        return 4 + self.prefix.len() + 4 + self.suffix.len();
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct NftCollectionData {
    // the royalty amount in basis points (0-10,000)
    pub royalty_bps: u16,

    pub royalty_shares: Vec<RoyaltyShare>,

    pub permitted_signers: Vec<Pubkey>,

    pub attribute_types: Vec<AttributeType>,
}

impl NftCollectionData {
    pub fn get_size(&self) -> usize {
        let total_size_attribute_configurations: usize = 4 + self
            .attribute_types
            .iter()
            .map(|x| 4 + x.get_size())
            .sum::<usize>();

        let size = 2
            + 4
            + 34 * self.royalty_shares.len()
            + 4
            + 32 * self.permitted_signers.len()
            + total_size_attribute_configurations
            + 1 as usize;

        return size;
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct RoyaltyShare {
    // royalty address and their share in basis points (0-10,000)
    pub recipient: Pubkey,

    pub share: u16,
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CollectionInput {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub collection_render_mode: CollectionRenderMode,
    pub metadata_render_mode: MetadataRenderMode,
    pub nft_collection_data: Option<NftCollectionData>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl CollectionInput {
    pub fn get_size(&self) -> usize {
        let size 
            = 4 + self.name.len()
            + 4 + self.symbol.len()
            + 4 + self.description.len()
            + self.collection_render_mode.get_size()
            + self.metadata_render_mode.get_size()
            + 1 + match self.nft_collection_data.as_ref() {
                Some(data) => data.get_size(),
                None => 0,
            };

        return size;
    }
}