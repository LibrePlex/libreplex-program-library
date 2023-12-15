use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{program::LibreplexInscriptions, InscriptionV3};

use crate::{
    legacy_inscription::LegacyInscription,
    LegacyInscriptionErrorCode,
};

use super::check_metadata_uauth;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ResizeLegacyInscriptionInput {
    pub change: i32,
    /*
        This only exists to show solana
        that each of the resize inputs is
        in fact a separate transaction
    */
    pub expected_start_size: u32,
    /*
        target size is specified
        to make sure that multiple resizes
        executed concurrently never increase / decrease
        the size beyond target size
    */
    pub target_size: u32,
}

// Adds a metadata to a group
#[derive(Accounts)]
pub struct ResizeLegacyInscriptionAsUauthV3<'info> {
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

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_v3: Account<'info, InscriptionV3>,

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
    ctx: Context<ResizeLegacyInscriptionAsUauthV3>,
    input: ResizeLegacyInscriptionInput,
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

    let inscription_v3_seeds: &[&[u8]] = &["inscription_v3".as_bytes(), mint_key.as_ref()];

    let expected_inscription_v3_key =
        Pubkey::find_program_address(inscription_v3_seeds, &libreplex_inscriptions::id()).0;

    if expected_inscription_v3_key != inscription_v3.key() {
        return Err(LegacyInscriptionErrorCode::InscriptionV3KeyMismatch.into());
    }

    libreplex_inscriptions::cpi::resize_inscription_v3(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            libreplex_inscriptions::cpi::accounts::ResizeInscriptionV3 {
                payer: payer.to_account_info(),
                authority: legacy_inscription.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        libreplex_inscriptions::instructions::ResizeInscriptionInput {
            change: input.change,
            expected_start_size: input.expected_start_size,
            target_size: input.target_size,
        },
    )?;

    Ok(())
}
