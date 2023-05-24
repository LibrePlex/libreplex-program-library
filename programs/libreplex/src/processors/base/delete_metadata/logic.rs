use anchor_lang::prelude::*;

use crate::{check_bump, METADATA, DeleteMetadata};

pub fn handle_delete_metadata(
    ctx: Context<DeleteMetadata>
) -> Result<()> {
    Ok(())
}
