use anchor_lang::prelude::*;

use crate::{get_bump, CreateCollection, RoyaltyShare};

pub fn handle_create_collection(
    ctx: Context<CreateCollection>,
    _seed: Pubkey,
    creators: Vec<RoyaltyShare>,
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
    
    // item_count counts the number of items added & VERIFIED in this collection. If item_count_verified hits 0,
    // then the collection can be deleted. TODO: Check that this is desired behaviour.

    collection.item_count = 0;
    
    Ok(())
}
