use anchor_lang::prelude::*;

use crate::{check_bump, CreateMetadata, METADATA};

pub fn handle_create_metadata(
    ctx: Context<CreateMetadata>,
    name: String,
    symbol: String,
    image_url: String,
    is_mutable: bool,
    bump: u8,
) -> Result<()> {
    check_bump(&METADATA.to_owned(), &ctx.bumps, bump)?;

    let metadata = &mut ctx.accounts.metadata;
    metadata.name = name;
    metadata.symbol = symbol;
    metadata.image_url = image_url;
    metadata.is_mutable = is_mutable;
    metadata.bump = bump;
    Ok(())
}
