use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::MakeInscriptionImmutableV3, program::LibreplexInscriptions, InscriptionV3,
};

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode, instructions::check_metadata_type};

use super::check_metadata_uauth;


// Adds a metadata to a group
#[derive(Accounts)]
pub struct MakeImmutableV3<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_v3: Account<'info, InscriptionV3>,
    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: Checked in logic
    #[account(
        constraint = legacy_metadata.owner.key() == mpl_token_metadata::ID
    )]
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

pub fn make_immutable_v3(ctx: Context<MakeImmutableV3>) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;
    let mint = &ctx.accounts.mint;
    let inscription_summary = &ctx.accounts.inscription_summary;

    let legacy_inscription = &ctx.accounts.legacy_inscription;

    let legacy_metadata = &ctx.accounts.legacy_metadata;

    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object
    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps.legacy_inscription],
    ];

    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object

    check_metadata_uauth(
        legacy_metadata,
        mint.key(),
        authority.key(),
        super::AuthorityType::UpdateAuthority,
    )?;
    check_metadata_type(legacy_metadata, mint)?;

    
    let inscription_v3_seeds: &[&[u8]] = &[
        "inscription_v3".as_bytes(),
        mint_key.as_ref()
    ];

    let expected_inscription_v3_key = Pubkey::find_program_address(inscription_v3_seeds, &libreplex_inscriptions::id()).0;

    if expected_inscription_v3_key != inscription_v3.key() {
        return Err(LegacyInscriptionErrorCode::Inscription2KeyMismatch.into());
    }


    libreplex_inscriptions::cpi::make_inscription_immutable_v3(CpiContext::new_with_signer(
        inscriptions_program.to_account_info(),
        MakeInscriptionImmutableV3 {
            authority: legacy_inscription.to_account_info(),
            inscription_v3:inscription_v3.to_account_info(),
            system_program: system_program.to_account_info(),
            payer: authority.to_account_info(),
            inscription_summary: inscription_summary.to_account_info(),
        },
        &[inscription_auth_seeds],
    ))?;

    Ok(())
}
