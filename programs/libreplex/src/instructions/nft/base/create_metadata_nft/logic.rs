use anchor_lang::prelude::*;

use crate::{Attribute, CreateMetadataNft, get_bump, MetadataNft, MetadataError};

pub fn handle_create_metadata_nft(
    ctx: Context<CreateMetadataNft>,
    name: String,
    offchain_url: String,
    is_mutable: bool,
    attributes: Vec<Attribute>,
) -> Result<()> {
    
    // increment reference counter on the collection.
    let collection = &mut ctx.accounts.collection;

    if collection.nft_data.is_none() {
        return Err(MetadataError::NotNftCollection.into())
    }

    let mint = &ctx.accounts.mint.key();
    collection.item_count += 1;

    let metadata = &mut ctx.accounts.metadata;

    let bumps = &ctx.bumps;

    let bump = get_bump(&"metadata".to_owned(), bumps)?;

    metadata.collection = collection.key();
    metadata.mint = mint.key();
    metadata.is_mutable = is_mutable;
    metadata.bump = bump;
    metadata.offchain_url = offchain_url;
    metadata.name = name;

    metadata.nft_data = Some(MetadataNft {
        attributes,
        signers: vec![]
    });
    
    Ok(())
}
