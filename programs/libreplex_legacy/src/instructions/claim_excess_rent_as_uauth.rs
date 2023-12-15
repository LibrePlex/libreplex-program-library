use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::ClaimExcessRent,
    program::LibreplexInscriptions, InscriptionV3,
};


use crate::legacy_inscription::LegacyInscription;

use super::check_metadata_uauth;




// Adds a metadata to a group
#[derive(Accounts)]
pub struct ClaimExcessRentAsUauth<'info> {
    // must match the update authority of the METADATA
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked in logic
    #[account(
        constraint = legacy_metadata.owner.key() == mpl_token_metadata::ID
    )]
    pub legacy_metadata: UncheckedAccount<'info>,

    #[account()]
    pub inscription_v3: Account<'info, InscriptionV3>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// is passed on to inscriptions program as the authority of the INSCRIPTION
    #[account(
        seeds=[
            "legacy_inscription".as_bytes(),
            mint.key().as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<ClaimExcessRentAsUauth>
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let payer = &ctx.accounts.payer;

    let metaplex_metadata = &ctx.accounts.legacy_metadata;
    check_metadata_uauth(
        metaplex_metadata,
        mint.key(),
        authority.key(),
        legacy_inscription.authority_type,
    )?;

    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps.legacy_inscription],
    ];

    libreplex_inscriptions::cpi::claim_excess_rent(CpiContext::new_with_signer(
        inscriptions_program.to_account_info(),
        ClaimExcessRent {
            payer: payer.to_account_info(),
            authority: legacy_inscription.to_account_info(),
            inscription_v3: inscription_v3.to_account_info(),
            system_program: system_program.to_account_info(),
            inscription_data: inscription_data.to_account_info(),
        },
        &[inscription_auth_seeds],
    ))?;

    Ok(())
}
