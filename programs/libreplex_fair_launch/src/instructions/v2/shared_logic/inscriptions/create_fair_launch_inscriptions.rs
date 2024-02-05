use anchor_lang::prelude::*;


// use libreplex_shared::sysvar_instructions_program;

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV3,
    instructions::SignerType,
};

use crate::{
    Deployment, create_immutable_inscription,
};

pub fn create_fair_launch_inscriptions<'a>(
    inscriptions_program: &AccountInfo<'a>,
    inscription_summary: &AccountInfo<'a>,
    non_fungible_mint: &Signer<'a>,
    inscription_v3: &AccountInfo<'a>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    inscription_data: &AccountInfo<'a>,
    deployment: &mut Account<'a, Deployment>,
) -> Result<()> {
    libreplex_inscriptions::cpi::create_inscription_v3(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            CreateInscriptionV3 {
                /* the inscription root is set to metaplex
                    inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),

                root: non_fungible_mint.to_account_info(),
                // since root in this case can sign (we are creating a brand new mint),
                // it will sign
                signer: non_fungible_mint.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),

                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInputV3 {
            authority: Some(payer.key()), // this includes update auth / holder, hence
            signer_type: SignerType::Root,
            validation_hash: None,
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
