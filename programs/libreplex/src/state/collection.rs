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

    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    // collection url
    pub url: String,

    // the number of items in collection
    pub item_count: u64,

    // for NFT collections
    pub nft_collection_data: Option<NftCollectionData>
}

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct NftCollectionData {

    // the royalty amount in basis points (0-10,000)
    pub royalty_bps: u16,

    pub royalty_shares: Vec<RoyaltyShare>,

    pub permitted_signers: Vec<Pubkey>,
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

impl NftCollectionData {

    pub fn get_size(&self) -> usize {

        let size = 2 + 4 + 34*self.royalty_shares.len() + 4 + 32*self.permitted_signers.len();

        return size;
    }

}

impl CollectionInput {

    pub fn get_size(&self) -> usize {

        let name_length = self.name.len();
        let symbol_length = self.symbol.len();
        let url_length = self.collection_url.len();

        let nft_data_length = match self.nft_collection_data.as_ref()
        {
            Some (data) => data.get_size(),
            None => 0
        };

        let size = 4 + name_length + 4 + symbol_length + 4 + url_length + 1 + nft_data_length;

        return size;
    }

}
