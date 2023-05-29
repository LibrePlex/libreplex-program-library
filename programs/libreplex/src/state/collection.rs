use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum CollectionRenderMode {
    NONE,
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
    PROGRAM(Pubkey),
    URL(String),
}

impl CollectionRenderMode {
    pub fn get_size(&self) -> usize {
        1 + match self {
            CollectionRenderMode::URL(item_base_url) => 4 + item_base_url.len(),
            CollectionRenderMode::PROGRAM(program_id) => 32,
            _ => 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum MetadataRenderMode {
    NONE {},
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
    PROGRAM(Pubkey),
    URL(Option<BaseUrlConfiguration>),
}

impl MetadataRenderMode {
    pub fn get_size(&self) -> usize {
        1 + match self {
            MetadataRenderMode::URL(item_base_url) => {
                1 + match item_base_url {
                    Some(baseConfig) => 4 + 4 + baseConfig.prefix.len() + baseConfig.suffix.len(),
                    None => 0,
                }
            }
            MetadataRenderMode::PROGRAM(program_id) => 32,
            _ => 0,
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

    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    pub collection_render_mode: CollectionRenderMode,

    // the number of items in collection
    pub item_count: u64,

    // for NFT collections
    pub nft_collection_data: Option<NftCollectionData>,

    pub metadata_render_mode: MetadataRenderMode,
}

impl Collection {
    pub fn get_size(&self) -> usize {
        32 + 32
            + 4
            + self.name.len()
            + 4
            + self.symbol.len()
            + self.collection_render_mode.get_size()
            + 8
            + 1
            + match &self.nft_collection_data {
                Some(x) => x.get_size(),
                None => 0,
            }
            + self.metadata_render_mode.get_size()
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

    pub permitted_values: Vec<String>,

    pub deleted: bool,

    // pointer to the next attribute type in case of attribute value overflow (>255)
    pub continued_at_index: Option<usize>,

    // pointer to the previous attribute type in case of attribute value overflow (>255)
    pub continued_from_index: Option<usize>,
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
    pub collection_render_mode: CollectionRenderMode,
    pub metadata_render_mode: MetadataRenderMode,
    pub nft_collection_data: Option<NftCollectionData>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl CollectionInput {
    pub fn get_size(&self) -> usize {
        let name_length = self.name.len();
        let symbol_length = self.symbol.len();

        let nft_data_length = match self.nft_collection_data.as_ref() {
            Some(data) => data.get_size(),
            None => 0,
        };

        let size = 4
            + name_length
            + 4
            + symbol_length
            + 4
            + self.collection_render_mode.get_size()
            + self.metadata_render_mode.get_size()
            + 1
            + nft_data_length;

        return size;
    }
}
