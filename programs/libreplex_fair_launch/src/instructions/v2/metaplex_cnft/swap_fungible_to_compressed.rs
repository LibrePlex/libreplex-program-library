use crate::{move_fungible_into_escrow, HashlistMarker};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use mpl_bubblegum::utils::get_asset_id;
// use libreplex_shared::operations::transfer_non_pnft;

use crate::Deployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
#[instruction(
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32
)]
pub struct SwapFungibleToCompressedCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Can be anything
    pub compressed_receiver: UncheckedAccount<'info>,

    /* fungible accounts */
    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = payer, // coimes out of the payer account
    )]
    pub fungible_source_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checked in transfer logic
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        get_asset_id(merkle_tree.key, nonce).as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: Checked by address
    #[account(address = mpl_bubblegum::ID)]
    pub bubble_gum_program: UncheckedAccount<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,
}

pub fn swap_fungible_to_compressed<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapFungibleToCompressedCtx<'info>>,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    // simples. two steps:
    // 1) move the fungible into the escrow
    move_fungible_into_escrow(
        token_program,
        fungible_source_token_account,
        fungible_target_token_account,
        source_wallet,
        fungible_mint,
        deployment,
        associated_token_program,
        system_program,
        payer,
    )?;

    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // 2) move the compressed out of the escrow
    bubblegum_proxy::cpi::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.bubble_gum_program.to_account_info(),
            bubblegum_proxy::cpi::accounts::Transfer {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                leaf_owner: deployment.to_account_info(),
                leaf_delegate: deployment.to_account_info(),
                new_leaf_owner: ctx.accounts.compressed_receiver.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[authority_seeds],
        )
        .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        root,
        data_hash,
        creator_hash,
        nonce,
        index,
    )?;

    // mark one of the non fungibles as moving out of the contract

    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
