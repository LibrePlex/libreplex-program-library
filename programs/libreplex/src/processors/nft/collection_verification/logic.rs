use anchor_lang::prelude::*;

use crate::{MetadataError, VerifyCollection};


pub fn handle_collection_verification(
    ctx: Context<VerifyCollection>,
    verify: bool,
) -> Result<()> {
    let metadata_nft = &mut ctx.accounts.metadata_nft;

    let ref mut collection = metadata_nft.collection;
    match &collection {
        Some(mut x) => {
            if x.address != ctx.accounts.collection_authority.key() {
                return Err(MetadataError::IncorrectCollectionAuthority.into());
            }
            x.verified = verify;
            
        }
        None => return Err(MetadataError::NoCollectionSet.into()),
    }
    Ok(())
}
