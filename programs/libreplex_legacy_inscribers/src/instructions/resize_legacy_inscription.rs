use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use libreplex_inscriptions::{
    cpi::accounts::ResizeInscription, instructions::ResizeInscriptionInput,
    program::LibreplexInscriptions, Inscription,
};


use crate::legacy_inscription::LegacyInscription;

use super::InscribeLegacyInput;

// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(input: InscribeLegacyInput)]
pub struct ResizeLegacyInscription<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub inscription: Account<'info, Inscription>,

     /// CHECK: Checked via a CPI call
     #[account(mut)]
     pub inscription_data: UncheckedAccount<'info>,

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
        input.legacy_type.to_string().as_bytes(),
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
    ctx: Context<ResizeLegacyInscription>,
    input: ResizeInscriptionInput,
    legacy_input: InscribeLegacyInput
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription = &mut ctx.accounts.inscription;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    
    let mint_key = mint.key();
    let legacy_type = legacy_input.legacy_type.to_string();
    let inscription_auth_seeds: &[&[u8]] = &[
        legacy_type.as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps["legacy_inscription"]],
    ];

    libreplex_inscriptions::cpi::resize_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            ResizeInscription {
                authority: legacy_inscription.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        input,
    )?;

    Ok(())
}
