use anchor_lang::prelude::*;

use crate::{check_bump, CreateMetadataNft, NFT, Creator, Attribute};

pub fn handle_create_metadata_nft(ctx: Context<CreateMetadataNft>, 
    creators: Vec<Creator>,
    attributes: Vec<Attribute>,
    collection: Option<Pubkey>,
    bump: u8) -> Result<()> {

    check_bump(&NFT.to_owned(), &ctx.bumps, bump)?;

    let metadata_nft = &mut ctx.accounts.metadata_nft;
    
    metadata_nft.creators = creators;
    metadata_nft.attributes = attributes;
    metadata_nft.collection = collection;

    Ok(())
}
