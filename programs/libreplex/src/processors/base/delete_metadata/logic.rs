use anchor_lang::prelude::*;

use crate::{check_bump, METADATA, DeleteMetadata};

pub fn handle_delete_metadata(
    ctx: Context<DeleteMetadata>
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    check_bump(&METADATA.to_owned(), &ctx.bumps, metadata.bump)?;
    Ok(())
}
