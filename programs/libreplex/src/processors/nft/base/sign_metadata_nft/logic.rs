use anchor_lang::prelude::*;

use crate::{MetadataError, MetadataNftOverride, SignMetadataNft};

pub fn handle_sign_metadata_nft(ctx: Context<SignMetadataNft>) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    let metadata_nft = &mut ctx.accounts.metadata_nft;
    let metadata_nft_override = &mut ctx.accounts.metadata_nft_override;
    let signer = &mut ctx.accounts.signer;

    /*
       A creator can only sign an NFT if the creator wallet
       is part of creators on NFT or the collection
    */

    match &collection.nft_data {
        Some(nft_data) => {
            let mut creator_found = false;

            // check the collection creators first

            if !metadata_nft_override.to_account_info().data_is_empty() {
                let metadata_nft_override_obj = Account::<MetadataNftOverride>::try_from(
                    &metadata_nft_override.to_account_info(),
                )?;
                match &metadata_nft_override_obj.permitted_signers {
                    Some(x) => {
                        if x.iter().find(|&x| x.key() == signer.key()).is_some() {
                            creator_found = true;
                        }
                    }
                    None => {}
                }
            } else {
                if nft_data
                    .permitted_signers
                    .iter()
                    .find(|&x| x.key() == signer.key())
                    .is_some()
                {
                    creator_found = true;
                }
            }

            if !creator_found {
                return Err(MetadataError::SignerNotInCreatorArray.into());
            }

            match &metadata_nft.nft_data {
                Some(metadata_nft_data) => {
                    // ensure we haven't already signed
                    if metadata_nft_data
                        .signers
                        .iter()
                        .find(|&x| x.key() == signer.key())
                        .is_some()
                    {
                        return Err(MetadataError::AlreadySigned.into());
                    }
                }
                None => return Err(MetadataError::CannotSignNonNftMetadata.into()),
            }
        }
        None => return Err(MetadataError::CannotSignItemInNonNftCollection.into()),
    }

    Ok(())
}
