/*
    Wrap an SPL-2022 mint taking control of its freeze authority, close authority and mint authority.
    Must have a supply of 1 and decimals of 0

    Allows the holder to delegate freeze.
    Allows the holder to close the mint.

    Does not allow further minting of tokens.
 */


use crate::{errors::ErrorCode, state::WrappedMint};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::ID as TOKEN_2022_PROGRAM_ID,
    token_interface::spl_token_2022::instruction::AuthorityType,
};
use solana_program::program_option::COption;
use spl_token_2022::{
    extension::{
        mint_close_authority::MintCloseAuthority, BaseStateWithExtensions, StateWithExtensions,
    },
    state::Mint,
};

#[derive(Accounts)]
pub struct WrapCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Account state checked in instruction
    #[account(
        mut,
        constraint = mint.owner.eq(&TOKEN_2022_PROGRAM_ID)
    )]
    pub mint: UncheckedAccount<'info>,

    #[account(init, seeds = [mint.key.as_ref()], bump, payer = payer, space = WrappedMint::LEN)]
    pub wrapped_mint: Account<'info, WrappedMint>,

    pub system_program: Program<'info, System>,

    /// CHECK: The token program
    #[account(
        address = spl_token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,
}


pub fn handler(ctx: Context<WrapCtx>) -> Result<()> {
    let mint_info = &ctx.accounts.mint;
    let authority_info = &ctx.accounts.authority;

    let (maybe_close_authority_extension, mint) = {
        let mint_data = mint_info.try_borrow_data()?;

        let mint: StateWithExtensions<'_, Mint> = StateWithExtensions::<Mint>::unpack(&mint_data)?;

        let maybe_close_authority_extension 
            = mint.get_extension::<MintCloseAuthority>().map(|m|{*m});

        (maybe_close_authority_extension, mint.base)
    };

    let wrapped_mint_info = &ctx.accounts.wrapped_mint;

    let token_program = &ctx.accounts.token_program;

    if mint.supply != 1 || mint.decimals != 0 {
        return Err(ErrorCode::MintCannotRepresentNFT.into());
    }

    if let COption::Some(actual_mint_authority) = mint.mint_authority.as_ref() {
        if actual_mint_authority != authority_info.key {
            return Err(ErrorCode::InvalidMintAuthority.into());
        }

        anchor_spl::token_2022::set_authority(
            CpiContext::new(
                token_program.to_account_info(),
                anchor_spl::token_2022::SetAuthority {
                    account_or_mint: mint_info.to_account_info(),
                    current_authority: authority_info.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            Some(wrapped_mint_info.key()),
        )?;
    } else {
        return Err(ErrorCode::InvalidMintAuthority.into());
    };

    if let COption::Some(actual_freeze_authority) = mint.freeze_authority.as_ref() {
        if actual_freeze_authority != authority_info.key {
            return Err(ErrorCode::InvalidMintAuthority.into());
        }

        anchor_spl::token_2022::set_authority(
            CpiContext::new(
                token_program.to_account_info(),
                anchor_spl::token_2022::SetAuthority {
                    account_or_mint: mint_info.to_account_info(),
                    current_authority: authority_info.to_account_info(),
                },
            ),
            AuthorityType::FreezeAccount,
            Some(wrapped_mint_info.key()),
        )?;
    }


    if let Ok(close_authority_extension) = maybe_close_authority_extension {
        let maybe_close_authority: Option<Pubkey> =
            close_authority_extension.close_authority.into();

        if let Some(actual_close_authority) = maybe_close_authority {
            if &actual_close_authority != authority_info.key {
                return Err(ErrorCode::InvalidMintAuthority.into());
            }

            anchor_spl::token_2022::set_authority(
                CpiContext::new(
                    token_program.to_account_info(),
                    anchor_spl::token_2022::SetAuthority {
                        account_or_mint: mint_info.to_account_info(),
                        current_authority: authority_info.to_account_info(),
                    },
                ),
                AuthorityType::CloseMint,
                Some(wrapped_mint_info.key()),
            )?;
        }
    }

    Ok(())
}
