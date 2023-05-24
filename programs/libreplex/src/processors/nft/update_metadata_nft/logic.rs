use anchor_lang::prelude::*;

use crate::{check_bump, Attribute, Creator, UpdateMetadataNft, validate_verified_creators};

pub fn handle_update_metadata_nft(
    ctx: Context<UpdateMetadataNft>,
    creators: Option<Vec<Creator>>,
    attributes: Option<Vec<Attribute>>,
) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let metadata_nft = &mut ctx.accounts.metadata_nft;
    let authority = &mut ctx.accounts.authority;
    check_bump(&"metadata".to_owned(), &ctx.bumps, metadata.bump)?;

    check_bump(&"metadata_nft".to_owned(), &ctx.bumps, metadata_nft.bump)?;

    match creators {
        Some(x) => {
            validate_verified_creators(&x, metadata_nft, authority)?;
            metadata_nft.creators = x;
        }
        None => {}
    }
    //

    match attributes {
        Some(x) => {
            metadata_nft.attributes = x;
        }
        None => {}
    }

    Ok(())
}
