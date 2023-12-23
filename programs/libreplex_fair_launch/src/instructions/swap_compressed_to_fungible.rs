use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use libreplex_shared::operations::transfer_non_pnft;
use mpl_bubblegum::utils::get_asset_id;

use crate::{Deployment, HashlistMarker, instruction};

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
pub struct SwapCompressedToFungibleCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mut)]
    pub compressed_holder: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Just needs to match
    pub leaf_delegate: UncheckedAccount<'info>,
    
    /// CHECK: Checked in cpi
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub tree_authority: UncheckedAccount<'info>,

    /* fungible accounts */
    #[account(mut)]
    pub fungible_mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked in cpi
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: Checked by address
    #[account(address = mpl_bubblegum::ID)]
    pub bubble_gum_program: UncheckedAccount<'info>,

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        get_asset_id(merkle_tree.key, nonce).as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// this always exists so we can specify the account type explicitly
    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_source_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: derivation checked in Logic. May not exist so created as required.
    #[account(
        mut
    )]
    pub fungible_target_token_account: UncheckedAccount<'info>,

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

pub fn swap_compressed_to_fungible<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, SwapCompressedToFungibleCtx<'info>>,   
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let tree_authority = &ctx.accounts.tree_authority;
    let compressed_holder = &ctx.accounts.compressed_holder;
    let leaf_delegate = &ctx.accounts.leaf_delegate;
    let merkle_tree = &ctx.accounts.merkle_tree;
    let log_wrapper = &ctx.accounts.log_wrapper;
    let compression_program = &ctx.accounts.compression_program;


    // simples. two steps:
    // 1) move the compressed into the escrow

    bubblegum_proxy::cpi::transfer(
        CpiContext::new(ctx.accounts.bubble_gum_program.to_account_info(), 
            bubblegum_proxy::cpi::accounts::Transfer {
                tree_authority: tree_authority.to_account_info(),
                leaf_owner: compressed_holder.to_account_info(),
                leaf_delegate: leaf_delegate.to_account_info(),
                new_leaf_owner: deployment.to_account_info(),
                merkle_tree: merkle_tree.to_account_info(),
                log_wrapper: log_wrapper.to_account_info(),
                compression_program: compression_program.to_account_info(),
                system_program: system_program.to_account_info(),
            },
        ).with_remaining_accounts(ctx.remaining_accounts.to_vec()), 
        root, data_hash, creator_hash, nonce, index)?;

    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // // 2) move the fungible_mint out of the escrow
    transfer_non_pnft(
        &token_program.to_account_info(),
        &fungible_source_token_account.to_account_info(),
        &fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &fungible_mint.to_account_info(),
        &payer.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]),
        &payer.to_account_info(),
        deployment.get_fungible_mint_amount(),
    )?;


    deployment.escrow_non_fungible_count += 1;
    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
