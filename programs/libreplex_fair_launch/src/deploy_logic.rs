use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV3,
    cpi::accounts::MakeInscriptionImmutableV3,
    cpi::accounts::ResizeInscriptionV3,
    cpi::accounts::WriteToInscriptionV3,
    instructions::{SignerType, WriteToInscriptionInput},
    InscriptionSummary,
};
use libreplex_shared::{
    create_mint_metadata_and_masteredition::create_mint_with_metadata_and_masteredition,
    MintAccounts,
};
use mpl_token_metadata::types::TokenStandard;

use crate::{Deployment, Hashlist};



pub fn deploy_logic<'f>(
    hashlist: &mut Account<'f, Hashlist>,
    deployment: &mut Account<'f, Deployment>,
    fungible_mint: &Signer<'f>,
    payer: &Signer<'f>,
    fungible_metadata: &UncheckedAccount<'f>,
    fungible_escrow_token_account: &UncheckedAccount<'f>,
    metadata_program: &UncheckedAccount<'f>,
    token_program: &Program<'f, Token>,
    associated_token_program: &Program<'f, AssociatedToken>,
    system_program: &Program<'f, System>,
    sysvar_instructions: &UncheckedAccount<'f>,
    non_fungible_mint: &Signer<'f>,
    non_fungible_metadata: &UncheckedAccount<'f>,
    non_fungible_master_edition: &UncheckedAccount<'f>,
    non_fungible_token_account: &UncheckedAccount<'f>,
    deployment_bump: u8
) -> Result<()> {
    hashlist.deployment = deployment.key();
    deployment.require_creator_cosign = false;
    deployment.use_inscriptions = true;
    deployment.fungible_mint = fungible_mint.key();
    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[deployment_bump],
    ];
    create_mint_with_metadata_and_masteredition(
        MintAccounts {
            authority_pda: deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: deployment.to_account_info(),
            nft_mint: fungible_mint.to_account_info(),
            nft_mint_authority: deployment.to_account_info(),
            nft_metadata: fungible_metadata.to_account_info(),
            nft_master_edition: None,
            token: Some(fungible_escrow_token_account.to_account_info()), // do not mint anything
            token_metadata_program: metadata_program.to_account_info(),
            spl_token_program: token_program.to_account_info(),
            spl_ata_program: associated_token_program.to_account_info(),
            system_program: system_program.to_account_info(),
            sysvar_instructions: sysvar_instructions.to_account_info(),
        },
        deployment_seeds,
        deployment.ticker.clone(),
        "".to_owned(),
        0,
        deployment.offchain_url.clone(),
        None,
        0, // number of print editions. always 0.
        false,
        0,
        deployment.decimals,
        TokenStandard::Fungible,
    )?;
    Ok(())
}