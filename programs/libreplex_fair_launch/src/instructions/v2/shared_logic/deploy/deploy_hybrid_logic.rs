use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use libreplex_shared::CreateMetadataAccounts;

use libreplex_shared::create_legacy_metadata;

use mpl_token_metadata::types::Creator;


use crate::DeploymentActive;
use crate::DeploymentConfig;
use crate::{
    mint_all_fungibles, revoke_mint_auths, Deployment, Hashlist, HYBRID_DEPLOYMENT_TYPE
};


pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

pub fn deploy_hybrid_logic<'f>(
    hashlist: &mut Account<'f, Hashlist>,
    deployment: &mut Account<'f, Deployment>,
    fungible_mint: &AccountInfo<'f>,
    fungible_metadata: &UncheckedAccount<'f>,
    fungible_master_edition: &UncheckedAccount<'f>,
    payer: &Signer<'f>,
    fungible_escrow_token_account: &AccountInfo<'f>,
    token_program: &UncheckedAccount<'f>,
    associated_token_program: &Program<'f, AssociatedToken>,
    system_program: &Program<'f, System>,
    rent: &Sysvar<'f, Rent>,
    sysvar_instructions_info: &UncheckedAccount<'f>,
    metadata_program: &UncheckedAccount<'f>,
    // non_fungible_mint: &Signer<'f>,
    deployment_bump: u8,
    deployment_config: &Account<'f, DeploymentConfig>,
) -> Result<()> {
    hashlist.deployment = deployment.key();
    msg!("Set fungible mint to {}", fungible_mint.key());
    deployment.fungible_mint = fungible_mint.key();
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
        false,
        deployment_config,
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
            vec![
                Creator { address: payer.key(), verified: false, share: 100 }
            ],
            false,
        )?
    }

    revoke_mint_auths(deployment, token_program, fungible_mint, deployment_seeds)?;

    emit!(DeploymentActive { 
        ticker: deployment.ticker.clone(),
        fungible_mint: fungible_mint.key(),
    });

    Ok(())
}
