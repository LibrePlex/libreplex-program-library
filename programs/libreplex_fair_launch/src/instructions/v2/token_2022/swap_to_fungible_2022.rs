use anchor_lang::prelude::*;

use anchor_spl::{token_2022,  associated_token::AssociatedToken, token_interface::{Token2022, TokenAccount}, token::{spl_token, Token}};
use libreplex_shared::operations::transfer_generic_spl;

use crate::{Deployment, HashlistMarker};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct SwapToFungible2022Ctx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /* fungible accounts */
    
    /// CHECK: Owner must be spl token or token 2022
    #[account(mut,
        constraint = fungible_mint.owner.eq(&token_2022::ID) || fungible_mint.owner.eq(&spl_token::ID))]
    pub fungible_mint: UncheckedAccount<'info>,

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// this always exists so we can specify the account type explicitly
    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_source_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: derivation checked in Logic. May not exist so created as required.
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* non-fungible accounts */
    /// CHECK: checked in constraint
    #[account(mut,
        owner = token_2022::ID,
    )]
    pub non_fungible_mint: UncheckedAccount<'info>,

    /// this always exists (otherwise we couldn't swap), so we can specify the account
    /// type explicitly
    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = payer,
    )]
    pub non_fungible_source_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: derivation checked in Logic. Will be created as needed
    #[account(mut)]
    pub non_fungible_target_token_account: UncheckedAccount<'info>,

    pub token_program_22: Program<'info, Token2022>,

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

pub fn swap_to_fungible_2022(ctx: Context<SwapToFungible2022Ctx>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let token_program_22 = &ctx.accounts.token_program_22;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;


    // simples. two steps:
    // 1) move the non_fungible into the escrow

    msg!("Transferring non fungible into escrow");
    transfer_generic_spl(
        &token_program_22.to_account_info(),
        &non_fungible_source_token_account.to_account_info(),
        &non_fungible_target_token_account.to_account_info(),
        &payer.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &deployment.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None, // payer signs
        &payer.to_account_info(),
        0,
        1,
    )?;

    let ticker = deployment.ticker.clone();

    let authority_seeds = &[
        "deployment".as_bytes(),
        ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // // 2) move the fungible_mint out of the escrow
    msg!("Transferring fungible from escrow");

    let target_token_program = match *fungible_mint.owner {
        spl_token::ID => {
            token_program.to_account_info()
        },
        spl_token_2022::ID => {
            token_program_22.to_account_info()
        },
        _ => {
            panic!("How could you do this to me")
        }
    };

    transfer_generic_spl(
        &target_token_program,
        &fungible_source_token_account.to_account_info(),
        &fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &fungible_mint.clone(),
        &payer.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]),
        &payer.to_account_info(),
        deployment.decimals,
        deployment.get_fungible_mint_amount(),
    )?;
    deployment.escrow_non_fungible_count += 1;


    Ok(())
}
