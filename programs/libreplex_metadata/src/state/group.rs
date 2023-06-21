use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::Royalties;


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum TemplateConfiguration {
    None,
    Template {
        name: String,
        image_url: String,
        description: String
    },
}

impl TemplateConfiguration {
    pub fn get_size(&self) -> usize {
        2 + match self {
            TemplateConfiguration::None => 1,
            TemplateConfiguration::Template {
                name,
                image_url,
                description
            } => name.len() + image_url.len() + description.len(),
        }
    }
}

#[account]
pub struct Group {
    // Seed address used to generate unique account PDA address
    pub seed: Pubkey,

    pub update_authority: Pubkey,

    // Creator does not convey any privileges
    pub creator: Pubkey,

    // the number of items in collection
    pub item_count: u32,


    /* variable length fields = match 1-1 with CollectionInput */
    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    pub url: String,

    pub description: String,

    pub template_configuration: TemplateConfiguration,

    pub royalties: Option<Royalties>,
    
    pub permitted_signers: Vec<Pubkey>,

    pub attribute_types: Vec<AttributeType>,
    
}



impl Group {

    pub const BASE_SIZE: usize  = 8 + 32 + 32 + 32 + 4; // anchor + seed + creator + item count

    pub fn get_size(&self) -> usize {
        Group::BASE_SIZE
        + 4 + self.name.len() // name
        + 4 + self.symbol.len() // symbol
        + 4 + self.url.len() // symbol
        + 4 + self.description.len() // symbol
        // + self.collection_render_mode.get_size()
        + self.template_configuration.get_size()
        + 1 + match &self.royalties {
            Some(x)=>x.get_size(),
            None=>0
        }
        + 4 + self.permitted_signers.len() * 32 
        + 4 + self.attribute_types.iter().map(|x|x.get_size()).sum::<usize>()
    }
}

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
    Word {value: String},
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
            AttributeValue::Word{value} => 4 + value.len()
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
pub struct GroupInput {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub description: String,
    pub metadata_render_mode: TemplateConfiguration,
    pub royalties: Option<Royalties>,
    pub attribute_types: Vec<AttributeType>,
    pub permitted_signers: Vec<Pubkey>

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl GroupInput {
    pub fn get_size(&self) -> usize {
        let size 
            = 4 + self.name.len()
            + 4 + self.symbol.len()
            + 4 + self.url.len()
            + 4 + self.description.len()
            // + self.collection_render_mode.get_size()
            + self.metadata_render_mode.get_size()
            + 1 + match self.royalties.as_ref() {
                Some(data) => data.get_size(),
                None => 0,
            }
            + 4 + self.attribute_types.iter().map(|x|x.get_size()).sum::<usize>()
            + 4 + self.permitted_signers.len() * 32;

        return size;
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum GroupEventType {
    Create,
    Update,
    Delete
}

#[event]
pub struct GroupEvent {
    pub event_type: GroupEventType,
    pub authority: Pubkey,
    pub name: String,
    pub id: Pubkey,    
}