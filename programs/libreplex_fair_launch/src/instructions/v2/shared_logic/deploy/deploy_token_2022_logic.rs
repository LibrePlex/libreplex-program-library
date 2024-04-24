use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use libreplex_shared::{
    create_token_2022_and_metadata, MintAccounts2022, TransferFeeParams,
};

use spl_token_metadata_interface::state::TokenMetadata;

use crate::{mint_all_fungibles, Deployment, DeploymentActive, DeploymentConfig, Hashlist};
use spl_pod::optional_keys::OptionalNonZeroPubkey;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

pub fn deploy_token_2022_logic<'f>(
    hashlist: &mut Account<'f, Hashlist>,
    deployment: &mut Account<'f, Deployment>,
    deployment_config: &Account<'f, DeploymentConfig>,
    fungible_mint: &Signer<'f>,
    payer: &Signer<'f>,
    fungible_escrow_token_account: &UncheckedAccount<'f>,
    token_program: &UncheckedAccount<'f>,
    associated_token_program: &Program<'f, AssociatedToken>,
    system_program: &Program<'f, System>,
    // non_fungible_mint: &Signer<'f>,
    deployment_bump: u8,
) -> Result<()> {
    hashlist.deployment = deployment.key();
    msg!("Set fungible mint to {}", fungible_mint.key());
    deployment.fungible_mint = fungible_mint.key();
    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[deployment_bump],
    ];

    let update_authority =
        OptionalNonZeroPubkey::try_from(Some(deployment.key())).expect("Bad update auth");

    msg!("Create token 2022 w/ metadata and group");
    create_token_2022_and_metadata(
        MintAccounts2022 {
            authority: deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: deployment.to_account_info(),
            nft_mint: fungible_mint.to_account_info(),
            spl_token_program: token_program.to_account_info(),
        },
        deployment.decimals,
        // None,
        Some(TokenMetadata {
            update_authority,
            mint: fungible_mint.key(),
            name: deployment.ticker.clone(),
            symbol: deployment.ticker.clone(),
            uri: deployment.offchain_url.clone(),
            additional_metadata: vec![],
        }),
        None,
        // Some(TokenGroupInput {
        //     max_size: deployment.max_number_of_tokens as u32,
        // }),
        None,
        Some(deployment_seeds),
        match deployment_config.transfer_fee_in_basis_points {
            0 => None,
            x => Some(TransferFeeParams {
                transfer_fee_bps: x,
                withdraw_fee_authority: match deployment_config.transfer_fee_withdraw_authority {
                    Some(y) => y,
                    _ => deployment.key()
                },
            }),
            
        },
    )?;

    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[deployment_bump],
    ];

    mint_all_fungibles(
        deployment,
        &fungible_mint.to_account_info(),
        fungible_escrow_token_account,
        associated_token_program,
        payer,
        system_program,
        token_program,
        deployment_seeds,
        true,
        deployment_config,
    )?;

    msg!("Created non fungible");

    emit!(DeploymentActive {
        ticker: deployment.ticker.clone(),
        fungible_mint: fungible_mint.key(),
    });

    Ok(())
}
