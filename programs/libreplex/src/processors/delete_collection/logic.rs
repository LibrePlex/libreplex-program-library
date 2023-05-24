use anchor_lang::prelude::*;

use crate::{MetadataError, DeleteCollection};

pub fn handle_delete_collection_nft(
    ctx: Context<DeleteCollection>
) -> Result<()> {
    let collection = &mut ctx.accounts.collection;

    if collection.item_count > 0 {
        return Err(MetadataError::CannotDeleteCollectionWithVerifiedItems.into())
    }

    Ok(())
}
