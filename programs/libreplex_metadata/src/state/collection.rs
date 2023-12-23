use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[account]
pub struct Collection {
    // Seed address used to generate unique account PDA address
    pub seed: Pubkey,

    pub update_authority: Pubkey,

    // the number of items in collection
    pub item_count: u32,

    /* variable length fields = match 1-1 with CollectionInput */
    // name and symbol of the collection
    pub name: String,

    pub symbol: String,

    // yes this could be a little bit expensive but it's only 
    // done once for each collection. better to keep this on the chain
    pub description: String,
    // and forget about messing around with off-chain jsons

    /* 
        why no url?? because collection is not a mint 
        and it does not need to be ERC 721 compliant.

        collections are never traded, nor should they 
        be accidentally burned
    */
}

impl Collection {

    pub const BASE_SIZE: usize  = 8 + 32 + 32 + 32 + 4; // anchor + seed + creator + item count

    pub fn get_size(&self) -> usize {
        Collection::BASE_SIZE
        + 4 + self.name.len() // name
        + 4 + self.symbol.len() // symbol
        + 4 + self.symbol.len() // symbol
    }
}
