use anchor_lang::prelude::*;

use crate::{get_bump, CreateCollection, RoyaltyShare, CollectionNftData};

pub fn handle_create_collection_nft(
    ctx: Context<CreateCollection>,
    _seed: Pubkey,
    royalty_bps: u16,
    royalty_shares: Vec<RoyaltyShare>,
    permitted_signers: Vec<Pubkey>,
    name: String,
    symbol: String,
    url: String,
) -> Result<()> {
    let bumps = &ctx.bumps;
    let bump = get_bump(&"collection".to_owned(), bumps)?;
    let collection = &mut ctx.accounts.collection;
    collection.name = name;
    collection.symbol = symbol;
    collection.url = url;
    collection.authority = ctx.accounts.authority.key();
    collection.bump = bump;
    
    // item_count counts the number of items added & VERIFIED in this collection. If item_count_verified hits 0,
    // then the collection can be deleted. TODO: Check that this is desired behaviour.

    collection.item_count = 0;

    collection.nft_data = Some(CollectionNftData {
        royalty_bps,
        royalty_shares,
        permitted_signers
    });
    
    Ok(())
}
