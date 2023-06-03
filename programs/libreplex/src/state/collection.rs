use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[repr(C)]
#[account]
<<<<<<< HEAD
#[derive(Debug)]
pub struct CollectionData {
=======
pub struct Collection {
>>>>>>> 551ed7b (Rename permissions to include is_admin and a light organise of field prefixes)
    // Seed address used to generate unique account PDA address
    pub seed: Pubkey,

    // Creator does not convey any privileges
    pub creator: Pubkey,

    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    // collection url
    pub url: String,

    // the number of items in collection
    pub item_count: u64,

    // for NFT collections
    pub nft_collection_data: Option<NftCollectionData>,
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


/* 
    #0 BACKGROUND - "blue", "red", ...., "green", "#3", // last entry is a pointer to the extension attribute type
    #1 FACE - "angry", "sad",
    #2 GLOVE - "metal", "riding"
    #3 BACKGROUND - "yellow", "black"
*/



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AttributeType {
    // royalty address and their share in basis points (0-10,000)
    pub name: String,

    pub permitted_values: Vec<String>,

    pub deleted: bool
}

impl AttributeType {
    pub fn get_size(&self) -> usize {
        let total_size: usize = self.permitted_values.iter().map(|x| 4 + x.len()).sum();

        return 4 + self.permitted_values.len() + 4 + total_size + 1;
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

    pub item_base_url: Option<BaseUrlConfiguration>,
}

impl NftCollectionData {
    pub fn get_size(&self) -> usize {

        let total_size_attribute_configurations: usize = 
            4
            + self.attribute_types.iter().map(|x|4+x.get_size()).sum::<usize>();

        let size = 2 + 4 + 34 * self.royalty_shares.len() + 4 + 32 * self.permitted_signers.len()
            + total_size_attribute_configurations
            + 1 as usize + match &self.item_base_url {
                Some(x) => x.get_size(),
                None => 0 as usize,
            };
            
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
    pub collection_url: String,
    pub nft_collection_data: Option<NftCollectionData>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl CollectionInput {
    pub fn get_size(&self) -> usize {
        let name_length = self.name.len();
        let symbol_length = self.symbol.len();
        let url_length = self.collection_url.len();

        let nft_data_length = match self.nft_collection_data.as_ref() {
            Some(data) => data.get_size(),
            None => 0,
        };

        let size = 4 + name_length + 4 + symbol_length + 4 + url_length + 1 + nft_data_length;

        return size;
    }
}
