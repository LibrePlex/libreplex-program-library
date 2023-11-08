use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::CreateInscription, instructions::SignerType, program::LibreplexInscriptions,
    EncodingType,
};
use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode, LegacyType};

#[derive(Clone, AnchorDeserialize, AnchorSerialize, PartialEq, Copy)]
pub enum AuthorityType {
    /*
       Holder-created inscription. Update authority holder
       can not touch the inscription. However they can
       remove the mint from their collection and airdrop
       a new mint with inscription to the holder in case
       they want to have a collection-wide inscription
       owned by the update authority.

       For mutable inscriptions, holder can resize / update
       if the underlying offchain image changes. holder can
       also close the inscription and reclaim the rent.

       For immutable inscriptions, nothing can be done to it.
       Rent from ommutable inscriptions CANNOT BE RECLAIMED.
    */
    Holder,

    /*
       Update-authority created inscription. If it is immutable,
       it is forever. If it is mutable, the update authority can
       resize / update the inscription.

       Holder cannot create a new inscription of a mint that
       already has an update authority inscription on it.

       For immutable inscriptions, nothing can be done to it.
       Rent from ommutable inscriptions CANNOT BE RECLAIMED.
    */
    UpdateAuthority,
}

pub fn create_legacy_inscription_logic<'a>(
    mint: &Account<'a, Mint>,
    legacy_inscription: &mut Account<'a, LegacyInscription>,
    authority_type: AuthorityType,
    inscription: &mut UncheckedAccount<'a>,
    expected_bump: u8,
    inscriptions_program: &Program<'a, LibreplexInscriptions>,
    inscription_summary: &mut UncheckedAccount<'a>,
    legacy_signer: &UncheckedAccount<'a>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    inscription_data: &mut UncheckedAccount<'a>,
    inscription_ranks_current_page: &UncheckedAccount<'a>,
    inscription_ranks_next_page: &UncheckedAccount<'a>,
    validation_hash: String,
    signer_type: SignerType,
    encoding_type: EncodingType,
    media_type: libreplex_inscriptions::MediaType,
) -> Result<()> {
    let mint_key = mint.key();
    legacy_inscription.authority_type = authority_type;
    legacy_inscription.mint = mint.key();
    legacy_inscription.inscription = inscription.key();
    legacy_inscription.legacy_type = LegacyType::MetaplexMint;
    let inscription_auth_seeds: &[&[u8]] = &[mint_key.as_ref(), &[expected_bump]];
    libreplex_inscriptions::cpi::create_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscription {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),

                root: mint.to_account_info(),
                /// since root in this case can not sign,
                /// this legacy inscription must be the signer
                /// this is ok as the inscriptions guarantee uniqueness
                /// per mint.
                signer: legacy_signer.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                inscription_ranks_current_page: inscription_ranks_current_page.to_account_info(),
                inscription_ranks_next_page: inscription_ranks_next_page.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInput {
            /*  set authority equal to the metaplex_inscription
            so that this program can later update it and
             make it immutable as long as the current holder
             of the metadata authorises it

             This delegation is needed to ensure that
             any authority associated with the inscription
             travels with the mint.

             the authority can (as of today) do any of the following
             1) resize the inscription
             2) upload data to the inscription (co-authorised by global signer
                    to ensure integrity of what's written )
             3) make the inscription immutable (and get a rank)
            */
            authority: Some(legacy_inscription.key()), // this includes update auth / holder, hence
            current_rank_page: 0,
            signer_type,
            encoding_type,
            media_type,
            // signer_type: SignerType::LegacyMetadataSigner,
            // encoding_type: EncodingType::Base64,
            // media_type: libreplex_inscriptions::MediaType::Image,
            validation_hash: Some(validation_hash),
        },
    )?;
    Ok(())
}

pub fn check_permissions_for_authority(
    legacy_metadata: &UncheckedAccount<'_>,
    mint: &Account<Mint>,
    auth_key: Pubkey,
) -> Result<()> {
    let mai = legacy_metadata.to_account_info().clone();
    let data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data.clone())?;
    if metadata_obj.mint != mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }
    if metadata_obj.update_authority != auth_key {
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
