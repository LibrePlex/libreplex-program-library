use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use libreplex_inscriptions::{
    instructions::MakeInscriptionImmutableInput, program::LibreplexInscriptions, cpi::accounts::MakeInscriptionImmutable,
};

use crate::legacy_inscription::LegacyInscription;

use super::InscribeLegacyInput;

// Adds a metadata to a group
#[derive(Accounts)]
pub struct MakeImmutable<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_ranks_current_page: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_ranks_next_page: UncheckedAccount<'info>,

    #[account(
        token::mint = mint,
        token::authority = authority
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,
    seeds=[
        "inscription_mp".as_bytes(),
        mint.key().as_ref()
    ], bump)]
    pub metaplex_inscription: Account<'info, LegacyInscription>,

    /// CHECK: The token program
    #[account(
        address = anchor_spl::token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<MakeImmutable>,
    input: MakeInscriptionImmutableInput,
    legacy_input: InscribeLegacyInput
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;
    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;
    let mint = &ctx.accounts.mint;
    let inscription_summary = &ctx.accounts.inscription_summary;
    let inscription_ranks_current_page = &ctx.accounts.inscription_ranks_current_page;
    let inscription_ranks_next_page = &ctx.accounts.inscription_ranks_next_page;
   
    let metaplex_inscription = &ctx.accounts.metaplex_inscription;
    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object
    let mint_key = mint.key();
    let legacy_type = legacy_input.legacy_type.to_string();
    let inscription_auth_seeds: &[&[u8]] = &[
        legacy_type.as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps["legacy_inscription"]],
    ];

    libreplex_inscriptions::cpi::make_inscription_immutable(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            MakeInscriptionImmutable {
                authority: metaplex_inscription.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: authority.to_account_info(),
                inscription_summary: inscription_summary.to_account_info(),
                inscription_ranks_current_page: inscription_ranks_current_page.to_account_info(),
                inscription_ranks_next_page: inscription_ranks_next_page.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        input,
    )?;

    Ok(())
}
