

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, set_authority, SetAuthority, spl_token::instruction::AuthorityType},
};
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV3,
    cpi::accounts::MakeInscriptionImmutableV3,
    cpi::accounts::ResizeInscriptionV3,
    cpi::accounts::WriteToInscriptionV3,
    instructions::{SignerType, WriteToInscriptionInput},
};
use libreplex_shared::{SharedError, create_mint_metadata_and_masteredition::create_mint_with_metadata_and_masteredition, MintAccounts};
use mpl_token_metadata::types::{Creator, TokenStandard};


use crate::{
    add_to_hashlist, errors::FairLaunchError, Deployment, HashlistMarker, MintEvent, STANDARD_DEPLOYMENT_TYPE
};


pub fn mint_legacy_logic<'info>(
    deployment: &mut Account<'info, Deployment>, 
    inscriptions_program: &UncheckedAccount<'info>, 
    inscription_summary: &Account<'info, InscriptionSummary>,
     non_fungible_mint: &Account<'info, Mint>, 
     inscription_v3: &UncheckedAccount<'info>, 
     system_program: &Program<'info, System>, 
     payer: &Signer<'info>, 
     inscription_data: &UncheckedAccount<'info>, 
    fungible_mint: &Account<'info, Mint>,
       fungible_token_account_escrow: &UncheckedAccount<'info>, 
       associated_token_program: &Program<'info, AssociatedToken>, 
       token_program: &Program<'info, Token>, 
       inscriber: &UncheckedAccount<'info>, 
       non_fungible_token_account: &UncheckedAccount<'info>, 
       non_fungible_metadata: &UncheckedAccount<'info>, 
       non_fungible_masteredition: &UncheckedAccount<'info>,
       metadata_program: &UncheckedAccount<'info>, 
       sysvar_instructions_program: &UncheckedAccount<'info>, 
       hashlist: &mut UncheckedAccount<'info>,
       hashlist_marker: &HashlistMarker,
        bump_deployment: u8) ->
 Result<()> {
    if deployment.deployment_type != STANDARD_DEPLOYMENT_TYPE {
        return Err(FairLaunchError::IncorrectMintType.into())
    }

    
    deployment.number_of_tokens_issued += 1;
 
    if deployment.use_inscriptions {
        libreplex_inscriptions::cpi::create_inscription_v3(
            CpiContext::new(
                inscriptions_program.to_account_info(),
                CreateInscriptionV3 {
                    /* the inscription root is set to metaplex
                        inscription object.
                    */
                    inscription_summary: inscription_summary.to_account_info(),

                    root: non_fungible_mint.to_account_info(),
                    // since root in this case can sign (we are creating a brand new mint),
                    // it will sign
                    signer: non_fungible_mint.to_account_info(),
                    inscription_v3: inscription_v3.to_account_info(),

                    system_program: system_program.to_account_info(),
                    payer: payer.to_account_info(),
                    inscription_data: inscription_data.to_account_info(),
                },
            ),
            libreplex_inscriptions::instructions::CreateInscriptionInputV3 {
                authority: Some(payer.key()), // this includes update auth / holder, hence
                signer_type: SignerType::Root,
                validation_hash: None,
            },
        )?;
        let data_bytes = deployment.mint_template.clone().into_bytes();
        libreplex_inscriptions::cpi::resize_inscription_v3(
            CpiContext::new(
                inscriptions_program.to_account_info(),
                ResizeInscriptionV3 {
                    /* the inscription root is set to metaplex
                    inscription object.
                    */
                    authority: payer.to_account_info(),

                    system_program: system_program.to_account_info(),
                    payer: payer.to_account_info(),
                    inscription_data: inscription_data.to_account_info(),
                    inscription_v3: inscription_v3.to_account_info(),
                },
            ),
            libreplex_inscriptions::instructions::ResizeInscriptionInput {
                change: data_bytes.len() as i32 - 8,
                expected_start_size: 8,
                target_size: data_bytes.len() as u32,
            },
        )?;
        libreplex_inscriptions::cpi::write_to_inscription_v3(
            CpiContext::new(
                inscriptions_program.to_account_info(),
                WriteToInscriptionV3 {
                    authority: payer.to_account_info(),
                    payer: payer.to_account_info(),
                    inscription_v3: inscription_v3.to_account_info(),
                    system_program: system_program.to_account_info(),
                    inscription_data: inscription_data.to_account_info(),
                },
            ),
            WriteToInscriptionInput {
                data: data_bytes,
                start_pos: 0,
                media_type: Some("text/plain".to_owned()),
                encoding_type: Some("ascii".to_owned()),
            },
        )?;
        libreplex_inscriptions::cpi::make_inscription_immutable_v3(CpiContext::new(
            inscriptions_program.to_account_info(),
            MakeInscriptionImmutableV3 {
                payer: payer.to_account_info(),
                authority: payer.to_account_info(),
                inscription_summary: inscription_summary.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
            },
        ))?;
    }
    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[bump_deployment],
    ];
    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &deployment.key(), &fungible_mint.key());
    if expected_token_account != fungible_token_account_escrow.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    if fungible_token_account_escrow.to_account_info().data_is_empty() {

        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
                mint: fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }
    let expected_non_fungible_token_account = anchor_spl::associated_token::get_associated_token_address(
        &inscriber.key(), &non_fungible_mint.key());
    if expected_non_fungible_token_account != non_fungible_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    if non_fungible_token_account.to_account_info().data_is_empty() {

        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: non_fungible_token_account.to_account_info(),
                authority: inscriber.to_account_info(),
                mint: non_fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: fungible_mint.to_account_info(),
                // always mint spl tokens to the program escrow
                to: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
            },
            &[deployment_seeds],
        ),
        deployment.get_fungible_mint_amount(hashlist_marker)   )?;
    create_mint_with_metadata_and_masteredition(
        MintAccounts {
            authority_pda: deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: inscriber.to_account_info(),
            nft_mint: non_fungible_mint.to_account_info(),
            nft_mint_authority: deployment.to_account_info(),
            nft_metadata: non_fungible_metadata.to_account_info(),
            nft_master_edition: Some(non_fungible_masteredition.to_account_info()),
            token: Some(non_fungible_token_account.to_account_info()), // do not mint anything
            token_metadata_program: metadata_program.to_account_info(),
            spl_token_program: token_program.to_account_info(),
            spl_ata_program: associated_token_program.to_account_info(),
            system_program: system_program.to_account_info(),
            sysvar_instructions: sysvar_instructions_program.to_account_info(),
        },
        deployment_seeds,
        // rent.to_account_into(),
        deployment.ticker.clone(),
        "".to_owned(),
        0,
        deployment.offchain_url.clone(),
        Some(
            [Creator {
                address: deployment.key(),
                verified: true,
                share: 100,
            }]
            .to_vec(),
        ),
        0,
        false, // this is the supply of the editions. always 0
        1,
        0, 
        TokenStandard::NonFungible,
    )?;
    if deployment.number_of_tokens_issued == deployment.max_number_of_tokens {
        if fungible_mint.freeze_authority.is_some() {
            // ok we are at max mint
            set_authority(CpiContext::new_with_signer(
                token_program.to_account_info(),
                SetAuthority {
                    current_authority: deployment.to_account_info(),
                    account_or_mint: fungible_mint.to_account_info(),
                },
                &[deployment_seeds]
            ),
            AuthorityType::FreezeAccount,
            None
            )?;
        }

        if fungible_mint.mint_authority.is_some() {
            // ok we are at max mint
            set_authority(CpiContext::new_with_signer(
                token_program.to_account_info(),
                SetAuthority {
                    current_authority: deployment.to_account_info(),
                    account_or_mint: fungible_mint.to_account_info(),
                },
                &[deployment_seeds]
            ),
            AuthorityType::MintTokens,
            None
            )?;
        }
    }
    if deployment.number_of_tokens_issued <= 262144 {
        add_to_hashlist(deployment.number_of_tokens_issued as u32, hashlist, 
            payer, 
            system_program, 
            &non_fungible_mint.key(), 
            &deployment.key(),
            inscription_summary.inscription_count_total
        )?;
    }
    emit!(MintEvent{
        mint: non_fungible_mint.key(),
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });
    Ok(())
}

