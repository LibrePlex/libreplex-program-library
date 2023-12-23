use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;




use crate::{
    errors::FairLaunchError, mint_fungible, update_deployment_and_hashlist, Deployment, STANDARD_DEPLOYMENT_TYPE, create_fair_launch_ghost_inscriptions,
};

pub fn redeem_cnft_logic<'info>(
    deployment: &mut Account<'info, Deployment>,
    inscriptions_program: &UncheckedAccount<'info>,
    inscription_summary: &Account<'info, InscriptionSummary>,
    inscription_v3: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    inscription_data: &UncheckedAccount<'info>,
    fungible_mint: &Account<'info, Mint>,
    fungible_token_account_escrow: &UncheckedAccount<'info>,
    associated_token_program: &Program<'info, AssociatedToken>,
    token_program: &Program<'info, Token>,
    ghost_root_signer: &UncheckedAccount<'info>,
    ghost_root_seeds: &[&[u8]],
    hashlist: &mut UncheckedAccount<'info>,
    asset_id: Pubkey,
    bump_deployment: u8,
) -> Result<()> {
    if deployment.deployment_type != STANDARD_DEPLOYMENT_TYPE {
        return Err(FairLaunchError::IncorrectMintType.into());
    }

    let ticker = deployment.ticker.clone();
    let deployment_seeds: &[&[u8]] =
        &["deployment".as_bytes(), ticker.as_ref(), &[bump_deployment]];

    mint_fungible(
        deployment,
        fungible_mint,
        fungible_token_account_escrow,
        associated_token_program,
        payer,
        system_program,
        token_program,
        deployment_seeds,
    )?;

    update_deployment_and_hashlist(
        deployment,
        fungible_mint,
        token_program,
        deployment_seeds,
        hashlist,
        payer,
        system_program,
        asset_id,
        inscription_summary,
    )?;

    if deployment.use_inscriptions {
        create_fair_launch_ghost_inscriptions(
            inscriptions_program,
            inscription_summary,
            ghost_root_signer,
            inscription_v3,
            system_program,
            payer,
            inscription_data,
            ghost_root_seeds,
            asset_id,
            deployment,
        )?;
    }
    Ok(())
}
