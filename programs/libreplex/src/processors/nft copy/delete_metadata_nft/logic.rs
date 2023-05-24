use anchor_lang::prelude::*;

use crate::{check_bump, DeleteMetadataNft};

pub fn handle_delete_metadata_nft(
    ctx: Context<DeleteMetadataNft>
) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let metadata_nft = &mut ctx.accounts.metadata_nft;
    check_bump(&"metadata".to_owned(), &ctx.bumps, metadata.bump)?;

    check_bump(&"metadata_nft".to_owned(), &ctx.bumps, metadata_nft.bump)?;

    Ok(())
}
