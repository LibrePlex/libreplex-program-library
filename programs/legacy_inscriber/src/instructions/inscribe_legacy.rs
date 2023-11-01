use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use libreplex_inscriptions::{cpi::accounts::CreateInscription, program::LibreplexInscriptions};
use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};


use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode, LegacyType};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InscribeLegacyInput {
    pub legacy_type: LegacyType,
}

// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(input: InscribeLegacyInput)]
pub struct InscribeLegacy<'info> {
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

    #[account(init,
        payer = authority,
        space = LegacyInscription::SIZE,
        seeds=[
            input.legacy_type.to_string().as_bytes(),
            mint.key().as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    /// CHECK: Checked in logic
    #[account()]
    pub legacy_object: UncheckedAccount<'info>,

    /// CHECK: The token program
    #[account(
        address = anchor_spl::token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(ctx: Context<InscribeLegacy>, input: InscribeLegacyInput) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &mut ctx.accounts.inscription_summary;
    let inscription = &mut ctx.accounts.inscription;
    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let legacy_object = &ctx.accounts.legacy_object;
    match input.legacy_type {
        LegacyType::MetaplexMint => {
            // make sure we are dealing with the correct metadata object.
            // this is to ensure that the mint in question is in fact a legacy
            // metadata object
            let metaplex_metadata = &ctx.accounts.legacy_object;
            let mai = metaplex_metadata.to_account_info().clone();
            let data: &[u8] = &mai.try_borrow_data()?[..];
            let metadata_obj = Metadata::deserialize(&mut data.clone())?;

            if metadata_obj.mint != ctx.accounts.mint.key() {
                return Err(LegacyInscriptionErrorCode::BadMint.into());
            }

            match metadata_obj.token_standard {
                Some(x) => match &x {
                    TokenStandard::Fungible => {
                        return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
                    }
                    TokenStandard::FungibleAsset => {
                        return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
                    }
                    _ => {}
                },
                None => {
                    return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
                }
            }
        }
    }

    let legacy_type = input.legacy_type.to_string();
    let mint_key = mint.key();
    let inscription_auth_seeds: &[&[u8]] = &[
        legacy_type.as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps["legacy_inscription"]],
    ];

    libreplex_inscriptions::cpi::create_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscription {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),
                root: legacy_object.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: authority.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInput {
            /*  set authority equal to the metaplex_inscription
            so that this program can later update it and
             make it immutable as long as the current holder
             of the metadata authorises it

             This delegation is needed to ensure that
             any authority associated with the inscription
             travels with the mint.

             the authority can (as of today) do any of the following
             1) resize the inscription
             2) upload data to the inscription (co-authorised by global signer
                    to ensure integrity of what's written )
             3) make the inscription immutable (and get a rank)
            */
            authority: Some(legacy_inscription.key()),
            max_data_length: 0,
            current_rank_page: 0,
        },
    )?;

    Ok(())
}
