use anchor_lang::prelude::*;


// use libreplex_shared::sysvar_instructions_program;

use libreplex_inscriptions::{
    cpi::accounts::MakeInscriptionImmutableV3,
    cpi::accounts::ResizeInscriptionV3,
    cpi::accounts::WriteToInscriptionV3,
    instructions::WriteToInscriptionInput,
};

use crate::Deployment;

pub fn create_immutable_inscription<'a>(
    deployment: &mut Account<'a, Deployment>,
    inscriptions_program: &AccountInfo<'a>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    inscription_data: &AccountInfo<'a>,
    inscription_v3: &AccountInfo<'a>,
    inscription_summary: &AccountInfo<'a>,
) -> Result<()> {
    let data_bytes = deployment.mint_template.clone().into_bytes();
    libreplex_inscriptions::cpi::resize_inscription_v3(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            ResizeInscriptionV3 {
                /* the inscription root is set to metaplex
                inscription object.
                */
                authority: payer.to_account_info(),

                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
            },
        ),
        libreplex_inscriptions::instructions::ResizeInscriptionInput {
            change: data_bytes.len() as i32 - 8,
            expected_start_size: 8,
            target_size: data_bytes.len() as u32,
        },
    )?;
    libreplex_inscriptions::cpi::write_to_inscription_v3(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            WriteToInscriptionV3 {
                authority: payer.to_account_info(),
                payer: payer.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
        ),
        WriteToInscriptionInput {
            data: data_bytes,
            start_pos: 0,
            media_type: Some("text/plain".to_owned()),
            encoding_type: Some("ascii".to_owned()),
        },
    )?;
    libreplex_inscriptions::cpi::make_inscription_immutable_v3(CpiContext::new(
        inscriptions_program.to_account_info(),
        MakeInscriptionImmutableV3 {
            payer: payer.to_account_info(),
            authority: payer.to_account_info(),
            inscription_summary: inscription_summary.to_account_info(),
            inscription_v3: inscription_v3.to_account_info(),
            system_program: system_program.to_account_info(),
        },
    ))?;
    Ok(())
}

