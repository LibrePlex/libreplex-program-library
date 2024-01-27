use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use libreplex_shared::{
    create_token_2022_and_metadata, CreateMetadataAccounts, MintAccounts2022, TokenGroupInput,
};

use libreplex_shared::create_legacy_metadata;

use spl_token_metadata_interface::state::TokenMetadata;

use crate::{
    mint_all_fungibles, Deployment, Hashlist, HYBRID_DEPLOYMENT_TYPE,
};
use spl_pod::optional_keys::OptionalNonZeroPubkey;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

pub fn deploy_hybrid_logic<'f>(
    hashlist: &mut Account<'f, Hashlist>,
    deployment: &mut Account<'f, Deployment>,
    fungible_mint: &Signer<'f>,
    fungible_metadata: &UncheckedAccount<'f>,
    fungible_master_edition: &UncheckedAccount<'f>,
    payer: &Signer<'f>,
    fungible_escrow_token_account: &UncheckedAccount<'f>,
    token_program: &UncheckedAccount<'f>,
    associated_token_program: &Program<'f, AssociatedToken>,
    system_program: &Program<'f, System>,
    rent: &Sysvar<'f, Rent>,
    sysvar_instructions_info: &UncheckedAccount<'f>,
    metadata_program: &UncheckedAccount<'f>,
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
        Some(TokenGroupInput {
            max_size: deployment.max_number_of_tokens as u32,
        }),
        None,
        Some(deployment_seeds),
    )?;

    mint_all_fungibles(
        deployment,
        &fungible_mint.to_account_info(),
        fungible_escrow_token_account,
        associated_token_program,
        payer,
        system_program,
        token_program,
        deployment_seeds,
    )?;

    if deployment.deployment_type == HYBRID_DEPLOYMENT_TYPE {
        create_legacy_metadata(
            CreateMetadataAccounts {
                authority_pda: deployment.to_account_info(),
                payer: payer.to_account_info(),
                nft_mint: fungible_mint.to_account_info(),
                nft_mint_authority: deployment.to_account_info(),
                nft_metadata: fungible_metadata.to_account_info(),
                nft_master_edition: fungible_master_edition.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
                sysvar_instructions_info: sysvar_instructions_info.to_account_info(),
                metadata_program: metadata_program.to_account_info(),
                token_program: token_program.to_account_info()
            },
            deployment_seeds,
            deployment.ticker.clone(),
            deployment.ticker[..std::cmp::min(10, deployment.ticker.len())].to_string(),
            0,
            deployment.offchain_url.clone(),
            Some(vec![]),
            false,
        )?
    }

    Ok(())
}
