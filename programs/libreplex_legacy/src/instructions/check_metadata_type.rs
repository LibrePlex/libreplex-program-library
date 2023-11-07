use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};


use crate::LegacyInscriptionErrorCode;


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

pub fn check_metadata_type(
    legacy_metadata: &UncheckedAccount<'_>,
    mint: &Account<Mint>,
) -> Result<()> {
    let mai = legacy_metadata.to_account_info().clone();
    let data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data.clone())?;
    if metadata_obj.mint != mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
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
            // no token standard, let's check supply on the mint.
            // that's the best we can do for now
            if mint.decimals != 0 || mint.supply != 1 {
                return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
            }
        }
    }

    Ok(())
}
