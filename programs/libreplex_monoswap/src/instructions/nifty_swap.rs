use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, Token2022, TokenAccount},
};
use libreplex_shared::operations::transfer_generic_spl;
use nifty_asset::instructions::TransferCpi;

use crate::{MarkerState, MonoSwapError, NiftyMarker, SwapDirection};

// the swap endpoint is symmetrical.
// it can be used to swap back and forth
#[derive(Accounts)]
pub struct NiftySwapCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        close = payer,
        constraint = mint.key() == nifty_marker.mint,
        constraint = asset.key() == nifty_marker.asset,
        seeds = [
            "nifty_marker".as_bytes(),
            nifty_marker.namespace.as_ref(),
            asset.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub nifty_marker: Account<'info, NiftyMarker>,

    #[account(
        constraint = asset.owner == nifty_program.key
    )]
    pub asset: UncheckedAccount<'info>,

    // each mint has to exist - there must be enough
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Check in pda derivation
    #[account(
        seeds = [
            "nifty_escrow".as_bytes(),
            nifty_marker.namespace.key().as_ref(),
            asset.key().as_ref(),
            mint.key().as_ref(),
        ], // always indexed by the incoming mint
        bump
    )]
    pub escrow_owner: UncheckedAccount<'info>,

    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = escrow_owner
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    // it is the responsibility of each swapper program to create enough
    // of the outgoing mint so that the swap can happen. It is deposited
    // from this account
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub external_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        address = nifty_asset::ID,
    )]
    pub nifty_program: UncheckedAccount<'info>,
}

pub fn process_nifty_swap(ctx: Context<NiftySwapCtx>, direction: SwapDirection) -> Result<()> {
    let nifty_marker = &mut ctx.accounts.nifty_marker;
    let asset_pubkey = ctx.accounts.asset.key();
    let mint_pubkey = ctx.accounts.mint.key();

    let authority_seeds: &[&[u8]] = &[
        "nifty_escrow".as_bytes(),
        nifty_marker.namespace.as_ref(),
        asset_pubkey.as_ref(),
        mint_pubkey.as_ref(),
        &[ctx.bumps.escrow_owner],
    ];

    // Determine the direction of the swap
    match direction {
        // Nifty asset comes in, fungibles go out
        SwapDirection::AssetIn => {
            require!(
                nifty_marker.state == MarkerState::FungibleEscrowed,
                MonoSwapError::InvalidMarkerState
            );

            // Nifty Transfer from payer to escrow_owner
            TransferCpi {
                __program: &ctx.accounts.nifty_program.to_account_info(),
                asset: &ctx.accounts.asset.to_account_info(),
                signer: &ctx.accounts.payer.to_account_info(),
                recipient: &ctx.accounts.escrow_owner.to_account_info(),
                group_asset: None,
            }
            .invoke()?;

            // Transfer fungible from escrow to payer
            transfer_generic_spl(
                &ctx.accounts.token_program.to_account_info(),
                &ctx.accounts.escrow_token_account.to_account_info(),
                &ctx.accounts.external_token_account.to_account_info(),
                &ctx.accounts.escrow_owner.to_account_info(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.payer.to_account_info(),
                &ctx.accounts.associated_token_program.to_account_info(),
                &ctx.accounts.system_program.to_account_info(),
                Some(&[&authority_seeds]), // payer signs
                &ctx.accounts.payer.to_account_info(),
                ctx.accounts.mint.decimals,
                nifty_marker.amount,
            )?;

            // Change nifty marker state
            nifty_marker.state = MarkerState::AssetEscrowed;
        }
        // Fungibles come in, nifty asset goes out
        SwapDirection::AssetOut => {
            // Nifty Transfer from escrow_owner to payer
            TransferCpi {
                __program: &ctx.accounts.nifty_program.to_account_info(),
                asset: &ctx.accounts.asset.to_account_info(),
                signer: &ctx.accounts.escrow_owner.to_account_info(),
                recipient: &ctx.accounts.payer.to_account_info(),
                group_asset: None,
            }
            .invoke()?;

            // Transfer fungible from payer to escrow
            transfer_generic_spl(
                &ctx.accounts.token_program.to_account_info(),
                &ctx.accounts.external_token_account.to_account_info(),
                &ctx.accounts.escrow_token_account.to_account_info(),
                &ctx.accounts.payer.to_account_info(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.escrow_owner.to_account_info(),
                &ctx.accounts.associated_token_program.to_account_info(),
                &ctx.accounts.system_program.to_account_info(),
                None, // payer signs
                &ctx.accounts.payer.to_account_info(),
                ctx.accounts.mint.decimals,
                nifty_marker.amount,
            )?;

            // Change nifty marker state
            nifty_marker.state = MarkerState::FungibleEscrowed;
        }
    }

    Ok(())
}
