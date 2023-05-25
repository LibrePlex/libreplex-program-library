use anchor_lang::prelude::*;

use crate::{DeleteMetadata, MetadataError};

pub fn handle_delete_metadata(ctx: Context<DeleteMetadata>) -> Result<()> {
    let metadata_override = &ctx.accounts.metadata_override;

    if !metadata_override.data_is_empty() {
        return Err(MetadataError::MustDeleteOverrideFirst.into());
    }

    // decrement reference counter on the collection.
    let collection = &mut ctx.accounts.collection;
    collection.item_count -= 1;

    Ok(())
}
