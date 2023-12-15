use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::SetValidationHash as BaseSetValidationHash, program::LibreplexInscriptions,
};
use mpl_token_metadata::accounts::Metadata;

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode};

use super::{check_metadata_type::content_validator_signer, AuthorityType};

// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(authority_type: AuthorityType)]
pub struct SetValidationHash<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
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

pub fn handler(
    ctx: Context<SetValidationHash>,
    validation_hash: Option<String>,
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;

    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let legacy_metadata = &ctx.accounts.legacy_metadata;

    let authority = &ctx.accounts.authority;

    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &["legacy_inscription".as_bytes(), mint_key.as_ref(), 
    &[ctx.bumps.legacy_inscription]];

    let mai = legacy_metadata.to_account_info().clone();
    let mut data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data)?;
    if metadata_obj.mint != mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }

    if authority.key() != content_validator_signer::ID {
        match legacy_inscription.authority_type {
            AuthorityType::Holder => {
                // A holder cannot override the validation hash
                return Err(LegacyInscriptionErrorCode::BadAuthorityForHolderInscription.into());
            }
            AuthorityType::UpdateAuthority => {
                // update authority wallet can override the validation hash
                if authority.key() != metadata_obj.update_authority {
                    return Err(
                        LegacyInscriptionErrorCode::BadAuthorityForUpdateAuthInscription.into(),
                    );
                }
            }
        }
    }

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
