use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::Deployment;

#[derive(Accounts)]
pub struct ToggleFreeze<'info> {
    #[account(seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mint::freeze_authority = deployment)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(token::mint = mint,
         constraint = token_account.amount == 1)]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub delegate: Signer<'info>,

    /// CHECK: Checked by addy
    #[account(constraint = 
        token_program.key == &anchor_spl::token::ID ||
         token_program.key == &anchor_spl::token_2022::ID)]
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn toggle_freeze(ctx: Context<ToggleFreeze>) -> Result<()> {
    let token_account = ctx.accounts.token_account.as_ref();
    let delegate = &ctx.accounts.delegate;
    let deployment = ctx.accounts.deployment.as_ref();

    if token_account.delegate.is_none() || token_account.delegated_amount != 1 ||
     token_account.delegate.as_ref().unwrap() != delegate.key {
        panic!("Not delegated");
    }


    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    if token_account.is_frozen() {
        anchor_spl::token_interface::thaw_account(
            CpiContext::new_with_signer(ctx.accounts.token_account.to_account_info(),
                 anchor_spl::token_interface::ThawAccount {
                    account: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: deployment.to_account_info(),
                }, 
                 &[authority_seeds])
        )?;
    } else {
        anchor_spl::token_interface::freeze_account(
            CpiContext::new_with_signer(ctx.accounts.token_account.to_account_info(),
                 anchor_spl::token_interface::FreezeAccount {
                    account: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: deployment.to_account_info(),
                }, 
                 &[authority_seeds])
        )?;
    }
    
    Ok(())
}