

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::spl_token, token_interface::{Mint,TokenAccount, spl_token_2022}};
use libreplex_shared::operations::transfer_generic_spl;

use crate::SwapMarker;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateMonoSwapInput {
    pub mint_outgoing_amount: u64,
    pub mint_incoming_amount: u64
}

#[derive(Accounts)]
pub struct CreateMonoSwapCtx<'info> {

    #[account(init,
        payer = payer, 
        space = SwapMarker::SIZE,
        seeds = ["swap_marker".as_bytes(), 
        namespace.key().as_ref(),
        mint_outgoing.key().as_ref(),
        mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub swap_marker: Account<'info, SwapMarker>,

    #[account(mut)] 
    pub mint_incoming: InterfaceAccount<'info, Mint>,

    // each mint has to exist - there must be enough 
    pub mint_outgoing: InterfaceAccount<'info, Mint>, 

    // it is the responsibility of each swapper program to create enough
    // of the outgoing mint so that the swap can happen. It is deposited 
    // from this account
    #[account(mut,
        associated_token::mint = mint_outgoing,
        associated_token::authority = mint_outgoing_owner
    )] 
    pub mint_outgoing_token_account_source: InterfaceAccount<'info, TokenAccount>,
    
    // escrow holders are organised by namespace + incoming mint - 
    // that way you can get wallet contents to see what swaps are available to you
    /// CHECK: Checked in transfer logic
    #[account(
        seeds = ["swap_escrow".as_bytes(), 
        namespace.key().as_ref(),
        mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump)]
    pub escrow_holder: UncheckedAccount<'info>,

    // ... into this escrow account
    #[account(init,
        payer = payer,
        // yes this is mint_outgoing
        associated_token::mint = mint_outgoing,
        associated_token::authority = escrow_holder
    )]
    pub mint_outgoing_token_account_escrow: InterfaceAccount<'info, TokenAccount>,


    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for integrations
    #[account(mut)]
    pub mint_outgoing_owner: Signer<'info>,

    // any account that can sign this. this is useful for grouping swaps 
    pub namespace: Signer<'info>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == spl_token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn create_swap(ctx: Context<CreateMonoSwapCtx>, input: CreateMonoSwapInput) -> Result<()> {
    
    let swap_marker = &mut ctx.accounts.swap_marker;
    let mint_outgoing = &mut ctx.accounts.mint_outgoing;
    

    swap_marker.namespace = ctx.accounts.namespace.key();
    swap_marker.mint_incoming = ctx.accounts.mint_incoming.key();
    swap_marker.mint_outgoing = mint_outgoing.key();
    swap_marker.mint_incoming_amount = input.mint_incoming_amount;
    swap_marker.mint_outgoing_amount = input.mint_outgoing_amount;

    swap_marker.used = false;

    // transfer the outgoing mint into escrow - 
    let token_program = &ctx.accounts.token_program;
    let mint_outgoing_token_account_source = &ctx.accounts.mint_outgoing_token_account_source;
    let mint_outgoing_token_account_escrow = &ctx.accounts.mint_outgoing_token_account_escrow;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let escrow_holder = &ctx.accounts.escrow_holder;
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
        &escrow_holder.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        mint_outgoing.decimals,
        input.mint_outgoing_amount,
        &[]
    )?;
    

    Ok(())
}
