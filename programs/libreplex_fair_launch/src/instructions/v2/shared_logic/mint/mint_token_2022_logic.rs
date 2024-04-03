use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use solana_program::{program::invoke, system_instruction};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_metadata_interface::state::TokenMetadata;

// use libreplex_shared::sysvar_instructions_program;

use libreplex_shared::{
    create_token_2022_and_metadata, operations::mint_non_fungible_2022_logic, MintAccounts2022,
    SharedError, TokenMemberInput,
};

use crate::{
    create_fair_launch_inscriptions, errors::FairLaunchError, update_deployment_and_hashlist,
    Deployment, DeploymentConfig, HashlistMarker, MintInput, HYBRID_DEPLOYMENT_TYPE,
    TOKEN2022_DEPLOYMENT_TYPE,
};

pub fn validate_new_multiplier(
    mint_input: &MintInput,
    config: &DeploymentConfig,
    deployment: &Deployment,
) -> Result<()> {
    if (mint_input.multiplier_denominator != 1 || mint_input.multiplier_numerator != 1)
        && !deployment.require_creator_cosign
    {
        return Err(FairLaunchError::MultiplierMissMatch.into());
    }

    if mint_input.multiplier_denominator != 1 || mint_input.multiplier_numerator != 1 {
        if let Some(limit) = config.multiplier_limits.as_ref() {
            if mint_input.multiplier_denominator < limit.min_denominator
                || mint_input.multiplier_numerator > limit.max_numerator
            {
                return Err(FairLaunchError::MultiplierMissMatch.into());
            }
        } else {
            return Err(FairLaunchError::MultiplierMissMatch.into());
        }
    }

    Ok(())
}

pub fn mint_token2022_logic<'info>(
    deployment: &mut Account<'info, Deployment>,
    deployment_config: &mut Account<'info, DeploymentConfig>,
    fee_treasury: &UncheckedAccount<'info>,
    fungible_mint: &AccountInfo<'info>,
    non_fungible_mint: &Signer<'info>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    associated_token_program: &Program<'info, AssociatedToken>,
    token_program: &UncheckedAccount<'info>,
    minter: &AccountInfo<'info>,
    non_fungible_token_account: &AccountInfo<'info>,
    hashlist: &mut UncheckedAccount<'info>,
    hashlist_marker: &mut HashlistMarker,
    bump_deployment: u8,
    remaining_accounts: &[AccountInfo<'info>],
    co_signer: &Signer<'info>,
    create_the_nft: bool,
    mint_input: MintInput,
) -> Result<()> {
    validate_new_multiplier(&mint_input, deployment_config, deployment)?;

    hashlist_marker.multiplier_denominator = mint_input.multiplier_denominator;
    hashlist_marker.multiplier_numerator = mint_input.multiplier_numerator;

    if !deployment.deployment_type.eq(&TOKEN2022_DEPLOYMENT_TYPE)
        && !deployment.deployment_type.eq(&HYBRID_DEPLOYMENT_TYPE)
    {
        return Err(FairLaunchError::IncorrectMintType.into());
    }

    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        return Err(FairLaunchError::MintedOut.into());
    }

    if deployment.migrated_from_legacy {
        return Err(FairLaunchError::LegacyMigrationsAreMintedOut.into());
    }

    if deployment.require_creator_cosign && !co_signer.key().eq(&deployment.creator.key()) {
        return Err(SharedError::InvalidCreatorCosigner.into());
    }

    let update_authority =
        OptionalNonZeroPubkey::try_from(Some(deployment.key())).expect("Bad update auth");

    deployment.number_of_tokens_issued += 1;

    deployment_config.total_spl_equivalent_minted +=
        deployment.get_fungible_mint_amount(hashlist_marker);

    if deployment_config.allow_burn {
        deployment_config.spl_excess_in_escrow += deployment
            .get_max_fungible_mint_amount_per_token(&deployment_config.multiplier_limits)
            .checked_sub(deployment.get_fungible_mint_amount(hashlist_marker))
            .unwrap();
    }

    let ticker = deployment.ticker.clone();
    let deployment_seeds: &[&[u8]] =
        &["deployment".as_bytes(), ticker.as_ref(), &[bump_deployment]];

    if create_the_nft {
        // msg!("Create token 2022 w/ metadata");
        create_token_2022_and_metadata(
            MintAccounts2022 {
                authority: deployment.to_account_info(),
                payer: payer.to_account_info(),
                nft_owner: minter.to_account_info(),
                nft_mint: non_fungible_mint.to_account_info(),
                spl_token_program: token_program.to_account_info(),
            },
            0,
            Some(TokenMetadata {
                name: deployment.ticker.clone(),
                symbol: deployment.ticker.clone(),
                uri: deployment.offchain_url.clone(),
                update_authority,
                mint: non_fungible_mint.key(),
                additional_metadata: vec![
                    ("fld".to_string(), deployment.key().to_string()),
                    (
                        "pos".to_string(),
                        deployment.number_of_tokens_issued.to_string(),
                    ),
                ],
            }),
            None,
            Some(TokenMemberInput {
                group_mint: fungible_mint.to_account_info(),
            }),
            Some(deployment_seeds),
            None,
        )?;

        // msg!("Minting 2022");
        mint_non_fungible_2022_logic(
            &non_fungible_mint.to_account_info(),
            non_fungible_token_account,
            associated_token_program,
            payer,
            &minter.to_account_info(),
            system_program,
            token_program,
            &deployment.to_account_info(),
            deployment_seeds,
        )?;
    }

    if deployment.use_inscriptions {
        if remaining_accounts.len() != 4 {
            panic!(
                "Incorrect number of remaining accounts. with use_inscriptions, you must provide 4"
            );
        }

        let inscriptions_program = &remaining_accounts[0];
        let inscription_summary = &remaining_accounts[1];
        let inscription_v3 = &remaining_accounts[2];
        let inscription_data = &remaining_accounts[3];

        // msg!("Creating inscriptions");
        create_fair_launch_inscriptions(
            inscriptions_program,
            inscription_summary,
            non_fungible_mint,
            inscription_v3,
            system_program,
            payer,
            inscription_data,
            deployment,
        )?;
        update_deployment_and_hashlist(
            deployment,
            hashlist,
            payer,
            system_program,
            non_fungible_mint.key(),
            Some(inscription_summary),
        )?;
    } else {
        update_deployment_and_hashlist(
            deployment,
            hashlist,
            payer,
            system_program,
            non_fungible_mint.key(),
            None,
        )?;
    }

    // finally send a fee to the creator if a fee is specified
    msg!(
        "Creator fee: {}",
        deployment_config.creator_fee_per_mint_lamports
    );
    if deployment_config.creator_fee_per_mint_lamports > 0 {
        msg!(
            "{} {}",
            payer.key(),
            deployment_config.creator_fee_treasury.key()
        );
        invoke(
            &system_instruction::transfer(
                &payer.key(),
                &deployment_config.creator_fee_treasury,
                deployment_config.creator_fee_per_mint_lamports,
            ),
            &[
                payer.to_account_info(),
                fee_treasury.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;
    }
    Ok(())
}
