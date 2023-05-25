use anchor_lang::prelude::*;

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct CollectionData {

    // the authority of the collection
    pub authority: Pubkey,

    // Seed address used to generate unique account PDA address
    pub collection_seed: Pubkey,

    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    // collection url
    pub collection_url: String,

    // the number of items in collection
    pub collection_count: u64,

    // for NFT collections
    pub nft_collection_data: Option<NftCollectionData>
}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct NftCollectionData {

    // the royalty amount in basis points (0-10,000)
    pub royalty_bps: u16,

    pub royalty_shares: Vec<RoyaltyShare>,

    pub permitted_signers: Vec<Pubkey>,
}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct RoyaltyShare {

    // royalty address and their share in basis points (0-10,000)
    pub royalty_address: Pubkey,

    pub royalty_share: u16,
}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct CollectionDataInput {

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

impl CollectionDataInput {

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
