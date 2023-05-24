use anchor_lang::prelude::*;

use crate::{get_bump, CreateMetadata};

pub fn handle_create_metadata(
    ctx: Context<CreateMetadata>,
    name: String,
    symbol: String,
    offchain_url: String,
    is_mutable: bool
) -> Result<()> {
    let bumps = &ctx.bumps;
    let bump = get_bump(&"metadata".to_owned(), bumps)?;
    let metadata = &mut ctx.accounts.metadata;
    metadata.name = name;
    metadata.symbol = symbol;
    metadata.offchain_url = offchain_url;
    metadata.is_mutable = is_mutable;
    Ok(())
}
