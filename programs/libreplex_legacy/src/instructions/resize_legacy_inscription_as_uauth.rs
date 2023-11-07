use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::ResizeInscription, instructions::ResizeInscriptionInput,
    program::LibreplexInscriptions, Inscription,
};
use mpl_token_metadata::accounts::Metadata;

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode};

use super::{
    check_metadata_type::check_metadata_type, create_legacy_inscription_logic::AuthorityType, resize_legacy_inscription_as_holder::ResizeLegacyInscriptionInput,
};

// Adds a metadata to a group
#[derive(Accounts)]
pub struct ResizeLegacyInscriptionAsUauth<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked in logic
    #[account()]
    pub legacy_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    #[account(mut,
        seeds=[
            "legacy_inscription".as_bytes(),
            mint.key().as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    /// CHECK: The token program
    #[account(
        address = anchor_spl::token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<ResizeLegacyInscriptionAsUauth>,
    input: ResizeLegacyInscriptionInput,
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let legacy_metadata = &ctx.accounts.legacy_metadata;
    let payer = &ctx.accounts.payer;

    let metaplex_metadata = &ctx.accounts.legacy_metadata;
    let mai = metaplex_metadata.to_account_info().clone();
    let data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data.clone())?;

    if metadata_obj.mint != ctx.accounts.mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }

    if metadata_obj.update_authority != ctx.accounts.authority.key()
        || legacy_inscription.authority_type != AuthorityType::UpdateAuthority
    {
        // return bad authority - only the owner of the mint / update authority can sign
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }

    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps["legacy_inscription"]],
    ];

    check_metadata_type(legacy_metadata, mint)?;

    libreplex_inscriptions::cpi::resize_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            ResizeInscription {
                payer: payer.to_account_info(),
                authority: legacy_inscription.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        ResizeInscriptionInput {
            change: input.change,
            expected_start_size: input.expected_start_size,
            target_size: input.target_size,
        },
    )?;

    Ok(())
}
