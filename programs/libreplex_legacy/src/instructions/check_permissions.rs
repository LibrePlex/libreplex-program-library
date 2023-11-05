use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};


use crate::LegacyInscriptionErrorCode;

use super::inscribe_legacy_metadata::AuthorityType;


/* 
    This signer is needed when holder inscribes their mints.
    In that case, we need a second signer to validate the
    inscription content.

    For update authority we don't care so much. It's their 
    collection and they can put whatever junk they want in the 
    inscription.

    However we do want to prevent scenarios where the holder
    buys a turbo-rug for 0.01 SOL and then uploads a mad lad 
    skull as an inscription in the hopes of flogging it off
    to the highest bidder.
*/
pub mod content_validator_signer {
    use anchor_lang::declare_id;
    declare_id!("S1GNkYN3NZxyKVZfaTecXbrb8tA8yDMEUYFxd9yuW22");
}


pub fn check_permissions(legacy_metadata: &UncheckedAccount<'_>, mint: &Account<Mint>, authority_type: AuthorityType, auth_key: Pubkey) 
    -> Result<()> {
    let mai = legacy_metadata.to_account_info().clone();
    let data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data.clone())?;
    if metadata_obj.mint != mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }
    if (authority_type == AuthorityType::UpdateAuthority
        && metadata_obj.update_authority != auth_key)
        || 
        // if authority == Holder, anybody can sign this. Because of the second signature
        // i.e. the content validator, we know that the request is bona fide and a hash 
        // will be set. only the holder can upload / resize at this point, but the upload
        // has to conform to the hash.
        (authority_type == AuthorityType::Holder
            && auth_key != content_validator_signer::id())
    {
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }
    match metadata_obj.token_standard {
        Some(x) => match &x {
            TokenStandard::Fungible => {
                return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
            }
            TokenStandard::FungibleAsset => {
                return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
            }
            _ => {}
        },
        None => {
            return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
        }
    }
   
    Ok(())
}
