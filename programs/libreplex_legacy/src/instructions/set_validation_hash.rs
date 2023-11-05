use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::SetValidationHash as BaseSetValidationHash,
    program::LibreplexInscriptions,
};

use crate::legacy_inscription::LegacyInscription;

use super::inscribe_metaplex_metadata::AuthorityType;

// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(authority_type: AuthorityType)]
pub struct SetValidationHash<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    #[account(mut,
    seeds=[
        &(authority_type as u32).to_le_bytes(),
        "legacy_inscription".as_bytes(),
        mint.key().as_ref()
    ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<SetValidationHash>,
    authority_type: AuthorityType,
    validation_hash: Option<String>,
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;

    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;

    /*
    check that authority is OK.
    For update authority, no second signer is needed
     */
    // TODO: Check permissions

    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        &(authority_type as u32).to_le_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps["legacy_inscription"]],
    ];

    libreplex_inscriptions::cpi::set_validation_hash(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            BaseSetValidationHash {
                payer: ctx.accounts.payer.to_account_info(),
                signer: legacy_inscription.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        validation_hash,
    )?;

    Ok(())
}
