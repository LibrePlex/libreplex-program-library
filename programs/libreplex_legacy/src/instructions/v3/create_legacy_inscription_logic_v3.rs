use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::cpi::accounts::CreateGhostRootInscription;
use libreplex_inscriptions::{
     instructions::SignerType, program::LibreplexInscriptions,
};

use libreplex_inscriptions::cpi::accounts::CreateInscriptionV3;


use crate::{legacy_inscription::LegacyInscription, LegacyType};


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

pub enum RootType<'a, 'b> {
    Mint(&'a Account<'b, Mint>),
    Ghost(&'a Pubkey),
}


pub fn create_legacy_inscription_logic_v3<'a>(
    root: RootType<'_, 'a>,
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
    match root {
        RootType::Mint(mint) => {
            let root_key = mint.key();
            legacy_inscription.authority_type = authority_type;
            legacy_inscription.mint = root_key;
            
            // inscription field on legacy_inscription could be either v1 or v3. 
            // within the contract it is information only.
            // However need to inform any downstream dependencies
            // if a client is relying on it. They shouldn't though
            // because both legacy inscription and inscriptions are
            // single-hop PDAs of the underlying mint. so both 
            // should be available
        
            legacy_inscription.inscription = inscription_v3.key(); 
            legacy_inscription.legacy_type = LegacyType::MetaplexMint;
            let inscription_auth_seeds: &[&[u8]] = &[root_key.as_ref(), &[expected_bump]];
        
        
            
            libreplex_inscriptions::cpi::create_inscription_v3(
                CpiContext::new_with_signer(
                    inscriptions_program.to_account_info(),
                    CreateInscriptionV3 {
                        /* the inscription root is set to metaplex
                         inscription object.
                        */
                        inscription_summary: inscription_summary.to_account_info(),
        
                        root: mint.to_account_info(),
                        // since root in this case can not sign,
                        // this legacy inscription must be the signer
                        // this is ok as the inscriptions guarantee uniqueness
                        // per mint.
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
            )
        },
        RootType::Ghost(root_key) => {
            legacy_inscription.authority_type = authority_type;
            legacy_inscription.mint = *root_key;
            
            // inscription field on legacy_inscription could be either v1 or v3. 
            // within the contract it is information only.
            // However need to inform any downstream dependencies
            // if a client is relying on it. They shouldn't though
            // because both legacy inscription and inscriptions are
            // single-hop PDAs of the underlying mint. so both 
            // should be available
        
            legacy_inscription.inscription = inscription_v3.key(); 
            legacy_inscription.legacy_type = LegacyType::MetaplexMint;
            let inscription_auth_seeds: &[&[u8]] = &[root_key.as_ref(), &[expected_bump]];
        
        
            
            libreplex_inscriptions::cpi::create_ghost_root_inscription(
                CpiContext::new_with_signer(
                    inscriptions_program.to_account_info(),
                    CreateGhostRootInscription {
                        /* the inscription root is set to metaplex
                         inscription object.
                        */
                        inscription_summary: inscription_summary.to_account_info(),
                        // since root in this case can not sign,
                        // this legacy inscription must be the signer
                        // this is ok as the inscriptions guarantee uniqueness
                        // per mint.
                        signer: legacy_signer.to_account_info(),
                        inscription_v3: inscription_v3.to_account_info(),
        
                        system_program: system_program.to_account_info(),
                        payer: payer.to_account_info(),
                        inscription_data: inscription_data.to_account_info(),
                    },
                    &[inscription_auth_seeds],
                ),
                libreplex_inscriptions::instructions::CreateGhostRootInscriptionInput {
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
                    root: *root_key,
                },
            )
        },
    }
}

