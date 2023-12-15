use anchor_lang::prelude::*;


use mpl_token_metadata::accounts::Metadata;

use crate::{LegacyInscriptionErrorCode};

use super::create_legacy_inscription_logic_v3::AuthorityType;



pub fn check_metadata_uauth(
    metaplex_metadata: &UncheckedAccount<'_>,
    mint: Pubkey,
    authority: Pubkey,
    authority_type: AuthorityType,
) -> Result<Metadata> {
    let mai = metaplex_metadata.to_account_info().clone();
    let mut data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data)?;
    if metadata_obj.mint != mint {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }
    if metadata_obj.update_authority != authority
        || authority_type != AuthorityType::UpdateAuthority
    {
        // return bad authority - only the owner of the mint / update authority can sign
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }
    Ok(metadata_obj)
}
