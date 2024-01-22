use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use solana_program::{program::invoke, system_instruction};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_metadata_interface::state::TokenMetadata;

// use libreplex_shared::sysvar_instructions_program;

use libreplex_shared::{create_token_2022_and_metadata, MintAccounts2022, TokenMemberInput};

use crate::{
    create_fair_launch_inscriptions, mint_non_fungible_2022_logic, update_deployment_and_hashlist,
    Deployment, DeploymentConfig,
};

pub fn mint_token2022_logic<'info>(
    deployment: &mut Account<'info, Deployment>,
    deployment_config: &mut Account<'info, DeploymentConfig>,
    fee_treasury: &UncheckedAccount<'info>,
    inscriptions_program: &UncheckedAccount<'info>,
    inscription_summary: &UncheckedAccount<'info>,
    fungible_mint: &AccountInfo<'info>,
    non_fungible_mint: &Signer<'info>,
    inscription_v3: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    inscription_data: &UncheckedAccount<'info>,
    associated_token_program: &Program<'info, AssociatedToken>,
    token_program: &UncheckedAccount<'info>,
    minter: &UncheckedAccount<'info>,
    non_fungible_token_account: &UncheckedAccount<'info>,
    hashlist: &mut UncheckedAccount<'info>,
    bump_deployment: u8,
) -> Result<()> {
    let update_authority =
        OptionalNonZeroPubkey::try_from(Some(deployment.key())).expect("Bad update auth");
 

        
    deployment.number_of_tokens_issued += 1;

    let ticker = deployment.ticker.clone();
    let deployment_seeds: &[&[u8]] =
        &["deployment".as_bytes(), ticker.as_ref(), &[bump_deployment]];

    msg!("Create token 2022 w/ metadata");
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
            additional_metadata: vec![],
        }),
        None,
        Some(TokenMemberInput {
            group_mint: fungible_mint.to_account_info(),
        }),
        Some(deployment_seeds),
    )?;

    msg!("Minting 2022");
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

    if deployment.use_inscriptions {
        msg!("Creating inscriptions");
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
    msg!("Update deployment and hashlist");

    update_deployment_and_hashlist(
        deployment,
        hashlist,
        payer,
        system_program,
        non_fungible_mint.key(),
        inscription_summary,
    )?;

    // finally send a fee to the creator if a fee is specified

    if deployment_config.creator_fee_per_mint_lamports > 0 {
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
