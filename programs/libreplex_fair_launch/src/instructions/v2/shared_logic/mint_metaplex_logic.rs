use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

// use libreplex_shared::sysvar_instructions_program;


use libreplex_shared::{
    create_mint_metadata_and_masteredition::create_mint_with_metadata_and_masteredition,
    MintAccounts,
};
use mpl_token_metadata::types::{Creator, TokenStandard};

use crate::{
    errors::FairLaunchError, Deployment, STANDARD_DEPLOYMENT_TYPE, update_deployment_and_hashlist, create_fair_launch_inscriptions,
};

pub fn mint_metaplex_logic<'info>(
    deployment: &mut Account<'info, Deployment>,
    inscriptions_program: &UncheckedAccount<'info>,
    inscription_summary: &UncheckedAccount<'info>,
    non_fungible_mint: &Signer<'info>,
    inscription_v3: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    inscription_data: &UncheckedAccount<'info>,
    associated_token_program: &Program<'info, AssociatedToken>,
    token_program: &UncheckedAccount<'info>,
    inscriber: &UncheckedAccount<'info>,
    non_fungible_token_account: &UncheckedAccount<'info>,
    non_fungible_metadata: &UncheckedAccount<'info>,
    non_fungible_masteredition: &UncheckedAccount<'info>,
    metadata_program: &UncheckedAccount<'info>,
    sysvar_instructions_program: &UncheckedAccount<'info>,
    hashlist: &mut UncheckedAccount<'info>,
    bump_deployment: u8,
) -> Result<()> {
    if deployment.deployment_type != STANDARD_DEPLOYMENT_TYPE {
        return Err(FairLaunchError::IncorrectMintType.into());
    }

    let ticker = deployment.ticker.clone();
    let deployment_seeds: &[&[u8]] =
        &["deployment".as_bytes(), ticker.as_ref(), &[bump_deployment]];

    let ticker = deployment.ticker.clone();

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
        ticker,
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
    update_deployment_and_hashlist(
        deployment,
        hashlist,
        payer,
        system_program,
        non_fungible_mint.key(),
        inscription_summary,
    )?;

    if deployment.use_inscriptions {
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
    }
    Ok(())
}
