use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::WriteToInscription, instructions::WriteToInscriptionInput,
    program::LibreplexInscriptions,
};

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode};

use super::resize_legacy_inscription_as_uauth::check_metadata_uauth;

// having to redefine this here as otherwise anchor IDL will be missing a type
// hope this gets sorted at some point!

// Adds a metadata to a group
#[derive(Accounts)]
pub struct WriteToLegacyInscriptionAsUAuth<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_v2: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// CHECK: Checked in logic
    #[account()]
    pub legacy_metadata: UncheckedAccount<'info>,

    #[account(mut,
    seeds=[
        "legacy_inscription".as_bytes(),
        mint.key().as_ref()
    ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<WriteToLegacyInscriptionAsUAuth>,
    input: WriteToInscriptionInput,
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;
    let inscription_v2 = &mut ctx.accounts.inscription_v2;

    let inscription_data = &mut ctx.accounts.inscription_data;

    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let legacy_inscription = &ctx.accounts.legacy_inscription;

    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object
    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps.legacy_inscription],
    ];
    let metaplex_metadata = &ctx.accounts.legacy_metadata;
    check_metadata_uauth(
        metaplex_metadata,
        mint.key(),
        authority.key(),
        legacy_inscription.authority_type,
    )?;

    let inscription_v2_seeds: &[&[u8]] = &[
        "inscription_v3".as_bytes(),
        mint_key.as_ref()
    ];

    let expected_inscription_v2_key = Pubkey::find_program_address(inscription_v2_seeds, &libreplex_inscriptions::id()).0;

    if expected_inscription_v2_key != inscription_v2.key() {
        return Err(LegacyInscriptionErrorCode::Inscription2KeyMismatch.into());
    }

    libreplex_inscriptions::cpi::write_to_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            WriteToInscription {
                authority: legacy_inscription.to_account_info(),
                payer: authority.to_account_info(),
                inscription: inscription.to_account_info(),
                inscription2: match inscription_v2.data_is_empty() {
                    true => None,
                    _ => Some(inscription_v2.to_account_info()),
                },
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        input,
    )?;

    Ok(())
}
