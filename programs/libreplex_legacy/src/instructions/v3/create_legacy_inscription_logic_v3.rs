use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
     instructions::SignerType, program::LibreplexInscriptions,
};

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV3, 
};


use crate::{legacy_inscription::LegacyInscription, LegacyType, instructions::AuthorityType};



pub fn create_legacy_inscription_logic_v3<'a>(
    mint: &Account<'a, Mint>,
    legacy_inscription: &mut Account<'a, LegacyInscription>,
    authority_type: AuthorityType,
    inscription_v3: &mut UncheckedAccount<'a>,
    expected_bump: u8,
    inscriptions_program: &Program<'a, LibreplexInscriptions>,
    inscription_summary: &mut UncheckedAccount<'a>,
    legacy_signer: &UncheckedAccount<'a>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    inscription_data: &mut UncheckedAccount<'a>,
    validation_hash: String,
    signer_type: SignerType,
) -> Result<()> {
    let mint_key = mint.key();
    legacy_inscription.authority_type = authority_type;
    legacy_inscription.mint = mint.key();
    
    // inscription field on legacy_inscription could be either v1 or v3. 
    // within the contract it is information only.
    // However need to inform any downstream dependencies
    // if a client is relying on it. They shouldn't though
    // because both legacy inscription and inscriptions are
    // single-hop PDAs of the underlying mint. so both 
    // should be available

    legacy_inscription.inscription = inscription_v3.key(); 
    legacy_inscription.legacy_type = LegacyType::MetaplexMint;
    let inscription_auth_seeds: &[&[u8]] = &[mint_key.as_ref(), &[expected_bump]];
    libreplex_inscriptions::cpi::create_inscription_v3(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscriptionV3 {
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
                inscription_v3: inscription_v3.to_account_info(),

                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInputV3 {
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
            signer_type,
            // signer_type: SignerType::LegacyMetadataSigner,
            // encoding_type: EncodingType::Base64,
            // media_type: libreplex_inscriptions::MediaType::Image,
            validation_hash: Some(validation_hash),
        },
    )?;
    Ok(())
}

