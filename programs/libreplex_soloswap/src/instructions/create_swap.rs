

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::spl_token, token_interface::{Mint,TokenAccount, spl_token_2022}};
use libreplex_shared::operations::transfer_generic_spl;

use crate::SwapMarker;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateSwapInput {
    pub mint_outgoing_amount: u64
}

#[derive(Accounts)]
pub struct CreateSwapCtx<'info> {

    #[account(init,
        payer = payer, 
        space = SwapMarker::SIZE,
        seeds = ["swap_marker".as_bytes(), 
        swapper_program.key().as_ref(),
        mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub swap_marker: Account<'info, SwapMarker>,

    // each mint has to exist. for now we restrict incoming mints to NFTS
    // to make sure that each of these marker pairs can only be hit once
    // unless the swap is reversed and then called again
    #[account(mut,
        constraint = mint_incoming.decimals == 0 && mint_incoming.supply == 1
    )] 
    pub mint_incoming: InterfaceAccount<'info, Mint>,

    // each mint has to exist - there must be enough 
    pub mint_outgoing: InterfaceAccount<'info, Mint>, 

    // it is the responsibility of each swapper program to create enough
    // of the outgoing mint so that the swap can happen. It is deposited 
    // from this account
    #[account(mut,
        associated_token::mint = mint_outgoing,
        associated_token::authority = signer
    )] 
    pub mint_outgoing_token_account_source: InterfaceAccount<'info, TokenAccount>,
    
    // ... into this escrow account
    #[account(init,
        payer = payer,
        associated_token::mint = mint_outgoing,
        associated_token::authority = swap_marker
    )]
    pub mint_outgoing_token_account_escrow: InterfaceAccount<'info, TokenAccount>,


    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for integrations
    #[account(mut)]
    pub signer: Signer<'info>,

    // swapper signer always has the same PDA derivation
    // it tells the multiswap that the call originated 
    // with a certain swapper program and that it's
    // ok to generate the marker
    #[account(mut,
        seeds = ["swapper_signer".as_bytes()],
        seeds::program = swapper_program.key(),
        bump,
    )]
    pub swapper_signer: Signer<'info>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == spl_token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Can we anything - see swapper_signer derivation above
    #[account(mut)]
    pub swapper_program: UncheckedAccount<'info>,    

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn create_swap(ctx: Context<CreateSwapCtx>, input: CreateSwapInput) -> Result<()> {
    
    let swap_marker = &mut ctx.accounts.swap_marker;
    let mint_outgoing = &mut ctx.accounts.mint_outgoing;
    

    swap_marker.mint_incoming = ctx.accounts.mint_incoming.key();
    swap_marker.mint_outgoing = mint_outgoing.key();
    swap_marker.swapper_program = ctx.accounts.swapper_program.key();
    swap_marker.used = false;

    // transfer the outgoing mint into escrow - 
    let token_program = &ctx.accounts.token_program;
    let mint_outgoing_token_account_source = &ctx.accounts.mint_outgoing_token_account_source;
    let mint_outgoing_token_account_escrow = &ctx.accounts.mint_outgoing_token_account_escrow;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    let payer = &ctx.accounts.payer;

    transfer_generic_spl(
        &token_program.to_account_info(),
        &mint_outgoing_token_account_source.to_account_info(),
        &mint_outgoing_token_account_escrow.to_account_info(),
        &payer.to_account_info(),
        &mint_outgoing.to_account_info(),
        // swap marker outgoing owns this to start with.
        // when swapping, this ATA will be emptied
        // and a new mint will come in
        &swap_marker.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        mint_outgoing.decimals,
        input.mint_outgoing_amount,
    )?;
    

    Ok(())
}
