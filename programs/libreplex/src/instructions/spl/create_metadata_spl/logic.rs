use anchor_lang::prelude::*;

use crate::{get_bump, CreateMetadataSpl, MetadataError};

pub fn handle_create_metadata(
    ctx: Context<CreateMetadataSpl>,
    name: String,
    offchain_url: String,
    is_mutable: bool
) -> Result<()> {
    let bumps = &ctx.bumps;
    let bump = get_bump(&"metadata".to_owned(), bumps)?;

    // increment reference counter on the collection.
    let collection = &mut ctx.accounts.collection;



    if !collection.nft_data.is_none() {
        return Err(MetadataError::NotSplCollection.into())
    }


    let mint = &mut ctx.accounts.mint;
    collection.item_count += 1;

    let metadata = &mut ctx.accounts.metadata;

    metadata.collection = collection.key();
    metadata.mint = mint.key();
    metadata.is_mutable = is_mutable;
    metadata.bump = bump;    
    metadata.offchain_url = offchain_url;
    metadata.name = name;

    Ok(())
}
