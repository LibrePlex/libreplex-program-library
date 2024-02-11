use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, Token2022, TokenAccount},
};
use libreplex_shared::operations::transfer_generic_spl;

use crate::SwapMarker;

// the swap endpoint is symmetrical.
// it can be used to swap back and forth
#[derive(Accounts)]
pub struct SwapCtx<'info> {
    #[account(mut, 
        close = payer,
        constraint = mint_incoming.key() == swap_marker.mint_incoming,
        constraint = mint_outgoing.key() == swap_marker.mint_outgoing,
        seeds = [
            "swap_marker".as_bytes(), 
            swapper_program.key().as_ref(),
            mint_incoming.key().as_ref()],
        bump,)]
    pub swap_marker: Account<'info, SwapMarker>,

    /// swapping always creates a symmetrical swap marker that enables a swap back
    #[account(init,
        payer = payer, 
        space = SwapMarker::SIZE,
        seeds = ["swap_marker".as_bytes(), 
        swapper_program.key().as_ref(),
        mint_outgoing.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub swap_marker_reverse: Account<'info, SwapMarker>,

    // each mint has to exist. for now we restrict incoming mints to NFTS
    // to make sure that each of these marker pairs can only be hit once
    // unless the swap is reversed and then called again
    #[account()]
    pub mint_incoming: InterfaceAccount<'info, Mint>,

    #[account()]
    pub mint_outgoing: InterfaceAccount<'info, Mint>,

     // it is the responsibility of each swapper program to create enough
    // of the outgoing mint so that the swap can happen. It is deposited
    // from this account
    #[account(mut,
        associated_token::mint = mint_incoming,
        associated_token::authority = payer
    )]
    pub mint_incoming_token_account_source: InterfaceAccount<'info, TokenAccount>,

    // ... into this escrow account
    #[account(mut,
        associated_token::mint = mint_incoming,
        associated_token::authority = swap_marker_reverse // this goes into the new swap_marker
    )]
    pub mint_incoming_token_account_target: InterfaceAccount<'info, TokenAccount>,


    // it is the responsibility of each swapper program to create enough
    // of the outgoing mint so that the swap can happen. It is deposited
    // from this account
    #[account(mut,
        associated_token::mint = mint_outgoing,
        associated_token::authority = swap_marker
    )]
    pub mint_outgoing_token_account_source: InterfaceAccount<'info, TokenAccount>,

    // ... into this escrow account
    #[account(mut,
        associated_token::mint = mint_outgoing,
        associated_token::authority = payer
    )]
    pub mint_outgoing_token_account_target: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub token_program_2022: Program<'info, Token2022>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Can we anything - see swapper_signer derivation above
    #[account(mut)]
    pub swapper_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn swap(ctx: Context<SwapCtx>) -> Result<()> {

    
    let swap_marker_reverse = &mut ctx.accounts.swap_marker_reverse;
    let mint_incoming = &mut ctx.accounts.mint_incoming;
    let mint_outgoing = &mut ctx.accounts.mint_outgoing;
    
    let swap_marker = &ctx.accounts.swap_marker;
    swap_marker_reverse.used = true; // cannot delete these since the swap has been activated
    swap_marker_reverse.mint_incoming = mint_outgoing.key();
    swap_marker_reverse.mint_outgoing = mint_incoming.key();
    swap_marker_reverse.mint_incoming_amount = swap_marker.mint_outgoing_amount;
    swap_marker_reverse.mint_outgoing_amount = swap_marker.mint_incoming_amount;
    
    // transfer the outgoing mint into escrow -
    let token_program = &ctx.accounts.token_program;
    let mint_outgoing_token_account_source = &ctx.accounts.mint_outgoing_token_account_source;
    let mint_outgoing_token_account_target = &ctx.accounts.mint_outgoing_token_account_target;

    let mint_incoming_token_account_source = &ctx.accounts.mint_incoming_token_account_source;
    let mint_incoming_token_account_target = &ctx.accounts.mint_incoming_token_account_target;

    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    let payer = &ctx.accounts.payer;

    // outgoing is going to the payer
    transfer_generic_spl(
        &token_program.to_account_info(),
        &mint_outgoing_token_account_source.to_account_info(),
        &mint_outgoing_token_account_target.to_account_info(),
        &swap_marker.to_account_info(),
        &mint_outgoing.to_account_info(),
        // swap marker outgoing owns this to start with.
        // when swapping, this ATA will be emptied
        // and a new mint will come in
        &payer.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        mint_outgoing.decimals,
        swap_marker.mint_outgoing_amount,
    )?;

    transfer_generic_spl(
        &token_program.to_account_info(),
        &mint_incoming_token_account_source.to_account_info(),
        &mint_incoming_token_account_target.to_account_info(),
        &payer.to_account_info(),
        &mint_outgoing.to_account_info(),
        // swap marker outgoing owns this to start with.
        // when swapping, this ATA will be emptied
        // and a new mint will come in
        &swap_marker_reverse.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        mint_outgoing.decimals,
        swap_marker.mint_incoming_amount,
    )?;


    Ok(())
}
