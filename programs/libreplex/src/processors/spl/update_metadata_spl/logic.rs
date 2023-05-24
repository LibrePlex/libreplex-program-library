use anchor_lang::prelude::*;

use crate::{UpdateMetadata, MetadataError};

pub fn handle_update_metadata_spl(
    ctx: Context<UpdateMetadata>,
    name: Option<String>,
    offchain_url: Option<String>,
    is_mutable: Option<bool>

) -> Result<()> {

    // looking for symbol? This has been moved up to Collection

    let metadata = &mut ctx.accounts.metadata;

    if !metadata.is_mutable {
        return Err(MetadataError::MetadataIsNotMutable.into())
    }

    match name {
        Some(x) => {
            metadata.name = x;
        },
        None => {}
    }

    match offchain_url {
        Some(x) => {
            metadata.offchain_url = x;
        },
        None => {}
    }

    
    match is_mutable {
        Some(x) => {
            metadata.is_mutable = x;
        },
        None => {}
    }
    Ok(())
}
