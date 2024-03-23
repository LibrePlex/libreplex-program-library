use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use libreplex_shared::operations::transfer_generic_spl;

use crate::NiftyMarker;

// Swaps are created by transferring a token in.
#[derive(Accounts)]
pub struct CreateNiftySwapCtx<'info> {
    // any account that can sign this. this is useful for grouping swaps
    pub namespace: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
        payer = payer,
        space = 8 + NiftyMarker::INIT_SPACE,
        seeds = [
            "nifty_marker".as_bytes(),
            namespace.key().as_ref(),
            asset.key().as_ref(),
            mint.key().as_ref()
        ],
            bump,
    )]
    pub nifty_marker: Account<'info, NiftyMarker>,

    #[account(
        constraint = asset.owner == nifty_program.key
    )]
    pub asset: UncheckedAccount<'info>,

    // each mint has to exist - there must be enough
    pub mint: InterfaceAccount<'info, Mint>,

    // escrow holders are organised by namespace + incoming mint -
    // that way you can get wallet contents to see what swaps are available to you
    /// CHECK: Checked in transfer logic
    #[account(
        seeds = [
            "nifty_escrow".as_bytes(),
            namespace.key().as_ref(),
            asset.key().as_ref(),
            mint.key().as_ref(),
        ], // always indexed by the incoming mint
        bump
    )]
    pub escrow_owner: UncheckedAccount<'info>,

    #[account(init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = escrow_owner,
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,

    // leave this here for integrations
    #[account(mut)]
    pub mint_outgoing_owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        address = nifty_asset::ID,
    )]
    pub nifty_program: UncheckedAccount<'info>,
}

pub fn create_nifty_swap(ctx: Context<CreateNiftySwapCtx>, amount: u64) -> Result<()> {
    let swap_marker = &mut ctx.accounts.nifty_marker;
    let mint = &ctx.accounts.mint;

    swap_marker.namespace = ctx.accounts.namespace.key();
    swap_marker.mint = ctx.accounts.mint.key();
    swap_marker.amount = amount;

    // transfer the outgoing mint into escrow -
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;

    transfer_generic_spl(
        &token_program.to_account_info(),
        &ctx.accounts.source_token_account.to_account_info(),
        &ctx.accounts.escrow_token_account.to_account_info(),
        &ctx.accounts.payer.to_account_info(),
        &mint.to_account_info(),
        &ctx.accounts.escrow_owner.to_account_info(),
        &associated_token_program.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        None, // payer signs
        &ctx.accounts.payer.to_account_info(),
        mint.decimals,
        amount,
    )?;

    Ok(())
}
