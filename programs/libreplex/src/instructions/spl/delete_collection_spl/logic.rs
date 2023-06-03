use anchor_lang::prelude::*;


use crate::{get_bump, RoyaltyShare, MetadataError, DeleteCollectionSpl};

pub fn handle_delete_collection_spl(
    ctx: Context<DeleteCollectionSpl>,
    creators: Vec<RoyaltyShare>,
    name: String,
    symbol: String,
    url: String
) -> Result<()> {
    let bumps = &ctx.bumps;
    let bump = get_bump(&"collection".to_owned(), bumps)?;
    let collection = &mut ctx.accounts.collection;

    if collection.item_count > 0 {
        return Err(MetadataError::CannotDeleteCollectionWithVerifiedItems.into())
    }

    Ok(())
}
