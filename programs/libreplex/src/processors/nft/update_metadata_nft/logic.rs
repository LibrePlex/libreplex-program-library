use anchor_lang::prelude::*;

use crate::{validate_verified_creators, Attribute, Creator, UpdateMetadataNft};

pub fn handle_update_metadata_nft(
    ctx: Context<UpdateMetadataNft>,
    new_creators: Option<Vec<Creator>>,
    attributes: Option<Vec<Attribute>>,
    collection: Option<Pubkey>
) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let metadata_nft = &mut ctx.accounts.metadata_nft;
    let authority = &mut ctx.accounts.authority;

    match new_creators {
        Some(x) => {
            match &metadata_nft.creators {
                Some(existing_creators) => {
                    validate_verified_creators(
                        &existing_creators,
                        &x,
                        &authority.key(),
                    )?;
                }, None => {
                    validate_verified_creators(
                        &vec![],
                        &x,
                        &authority.key(),
                    )?;
                }
            }
            metadata_nft.creators = Some(x);
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
