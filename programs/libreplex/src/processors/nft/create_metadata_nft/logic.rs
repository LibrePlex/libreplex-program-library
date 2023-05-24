use anchor_lang::prelude::*;

use crate::{validate_verified_creators, Attribute, CollectionData, CreateMetadataNft, Creator};

pub fn handle_create_metadata_nft(
    ctx: Context<CreateMetadataNft>,
    creators: Option<Vec<Creator>>,
    attributes: Vec<Attribute>,
    collection: Option<Pubkey>,
) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let metadata_nft = &mut ctx.accounts.metadata_nft;

    metadata_nft.attributes = attributes;
    match creators {
        Some(x) => {
            validate_verified_creators(&vec![], &x, &authority.key())?;
            metadata_nft.creators = Some(x);
        }
        None => {
            metadata_nft.creators = None;
        }
    }

    match collection {
        Some(x) => {
            metadata_nft.collection = Some(CollectionData {
                address: x.key(),
                verified: false,
            });
        }
        None => {
            metadata_nft.collection = None;
        }
    }

    Ok(())
}
