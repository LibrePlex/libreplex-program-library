use anchor_lang::prelude::*;

use crate::{check_bump, UpdateMetadata, METADATA};

pub fn handle_update_metadata(
    ctx: Context<UpdateMetadata>,
    name: Option<String>,
    symbol: Option<String>,
    offchain_url: Option<String>,
    is_mutable: Option<bool>

) -> Result<()> {

    let metadata = &mut ctx.accounts.metadata;


    match name {
        Some(x) => {
            metadata.name = x;
        },
        None => {}
    }

    match symbol {
        Some(x) => {
            metadata.symbol = x;
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
