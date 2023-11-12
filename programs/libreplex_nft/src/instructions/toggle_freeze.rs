use crate::{errors::ErrorCode, state::WrappedMint};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint},
};


use anchor_spl::token_interface::TokenAccount;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum ToggleFreezeInput {
    Freeze,
    Unfreeze,
}

#[derive(Accounts)]
pub struct ToggleFreezeCtx<'info> {
    pub delegate: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(seeds = [token_account.mint.as_ref()], bump)]
    pub wrapped_mint: Account<'info, WrappedMint>,

    pub system_program: Program<'info, System>,

    /// CHECK: The token program
    #[account(
        address = spl_token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ToggleFreezeCtx>, input: ToggleFreezeInput) -> Result<()> {
    let token_account = &ctx.accounts.token_account;
    let mint_account = &ctx.accounts.mint;
    let delegate = &ctx.accounts.delegate;

    if token_account.mint != mint_account.key() {
        return Err(ErrorCode::InvalidMint.into());
    }

    if token_account.amount != 1 {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    if token_account.delegate.is_none()
        || token_account.delegate.as_ref().unwrap() != delegate.key
        || token_account.delegated_amount != 1
    {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    let bump = ctx.bumps.wrapped_mint;
    let signer_seeds = [token_account.mint.as_ref(), &[bump]];

    match input {
        ToggleFreezeInput::Freeze => {
            anchor_spl::token_2022::freeze_account(CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_2022::FreezeAccount {
                    account: token_account.to_account_info(),
                    authority: ctx.accounts.wrapped_mint.to_account_info(),
                    mint: mint_account.to_account_info(),
                },
                &[&signer_seeds],
            ))?;
        }
        ToggleFreezeInput::Unfreeze => {
            anchor_spl::token_2022::thaw_account(CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_2022::ThawAccount {
                    account: token_account.to_account_info(),
                    authority: ctx.accounts.wrapped_mint.to_account_info(),
                    mint: mint_account.to_account_info(),
                },
                &[&signer_seeds],
            ))?;
        }
    }

    Ok(())
}
