use anchor_lang::prelude::*;

use libreplex_inscriptions::instructions::SignerType;
// use libreplex_shared::sysvar_instructions_program;
use crate::{
    create_immutable_inscription, Deployment,
};

pub fn create_fair_launch_ghost_inscriptions<'a>(
    inscriptions_program: &UncheckedAccount<'a>,
    inscription_summary: &UncheckedAccount<'a>,
    ghost_root_signer: &UncheckedAccount<'a>,
    inscription_v3: &UncheckedAccount<'a>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    inscription_data: &UncheckedAccount<'a>,
    ghost_root_seeds: &[&[u8]],
    asset_id: Pubkey,
    deployment: &mut Account<'a, Deployment>,
) -> Result<()> {
    libreplex_inscriptions::cpi::create_ghost_root_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            libreplex_inscriptions::cpi::accounts::CreateGhostRootInscription {
                /* the inscription root is set to metaplex
                inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),
                signer: ghost_root_signer.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[ghost_root_seeds],
        ),
        libreplex_inscriptions::instructions::CreateGhostRootInscriptionInput {
            authority: Some(payer.key()), // this includes update auth / holder, hence
            signer_type: SignerType::FairLaunchGhostRootSigner,
            validation_hash: None,
            root: asset_id,
        },
    )?;
    
    create_immutable_inscription(
        deployment,
        inscriptions_program,
        payer,
        system_program,
        inscription_data,
        inscription_v3,
        inscription_summary,
    )?;

    Ok(())
}
