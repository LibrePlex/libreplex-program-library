use anchor_lang::prelude::*;


// use libreplex_shared::sysvar_instructions_program;




use crate::{
    errors::FairLaunchError, update_deployment_and_hashlist, Deployment, STANDARD_DEPLOYMENT_TYPE, create_fair_launch_ghost_inscriptions,
};

pub fn redeem_cnft_logic<'info>(
    deployment: &mut Account<'info, Deployment>,
    inscriptions_program: &UncheckedAccount<'info>,
    inscription_summary: &UncheckedAccount<'info>,
    inscription_v3: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    inscription_data: &UncheckedAccount<'info>,
    ghost_root_signer: &UncheckedAccount<'info>,
    ghost_root_seeds: &[&[u8]],
    hashlist: &mut UncheckedAccount<'info>,
    asset_id: Pubkey,
) -> Result<()> {
    if deployment.deployment_type != STANDARD_DEPLOYMENT_TYPE {
        return Err(FairLaunchError::IncorrectMintType.into());
    }

    update_deployment_and_hashlist(
        deployment,
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
