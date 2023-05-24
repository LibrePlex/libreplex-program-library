use anchor_lang::prelude::*;

use crate::{DeleteMetadata};

pub fn handle_delete_metadata_nft(
    ctx: Context<DeleteMetadata>
) -> Result<()> {

    // decrement reference counter on the collection.
    let collection = &mut ctx.accounts.collection;
    collection.item_count -= 1;

    
    Ok(())
}
