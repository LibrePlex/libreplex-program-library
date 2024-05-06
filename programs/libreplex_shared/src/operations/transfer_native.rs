use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::{
    account_info::AccountInfo,
    program::invoke_signed,
    rent::Rent,
    system_instruction,
};
use spl_token::ID;

use crate::{wrapped_sol, SharedError};

pub fn transfer_native<'info>(
    from_account: &AccountInfo<'info>,
    tmp_token_account: &AccountInfo<'info>, // used for transferring wrapped SOL, must be owned by payer
    target_token_account: &AccountInfo<'info>, // final target token account for wrapped SOL
    authority_seeds: Option<&[&[u8]]>,      // None or Some as needed for CPI
    payer: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    native_sol_auth_seeds: &[&[u8]], // None or Some as needed for CPI
    amount: u64,
) -> Result<()> {
    // this is a native transfer
    let minimum_rent = Rent::get()?.minimum_balance(TokenAccount::LEN);
    msg!("Transferring native token");

    let mut create_tmp_account = false;
    if tmp_token_account.data_is_empty() {
        create_tmp_account = true;
        let wrap_infos = vec![
            from_account.to_account_info(),
            tmp_token_account.to_account_info(),
        ];
        match authority_seeds {
            Some(_authority_seeds) => {
                // if native_sol_auth_seeds.is_none() {
                //     return Err(SharedError::NativeSolAuthSeedsNotSpecified.into())
                // }
                // wrapped sol token account
                invoke_signed(
                    &system_instruction::create_account(
                        &payer.key(),
                        &tmp_token_account.key(),
                        // rent.minimum_balance(Mint::LEN),
                        minimum_rent + amount,
                        TokenAccount::LEN as u64,
                        &ID,
                    ),
                    wrap_infos.as_slice(),
                    &[_authority_seeds, native_sol_auth_seeds],
                )?;

                anchor_spl::token::initialize_account3(CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    anchor_spl::token::InitializeAccount3 {
                        account: tmp_token_account.to_account_info(),
                        mint: mint.to_account_info(),
                        authority: payer.to_account_info(),
                    },
                    &[_authority_seeds],
                ))?;
            }
            None => {
                msg!("Create account");
                invoke_signed(
                    &system_instruction::create_account(
                        &payer.key(),
                        &tmp_token_account.key(),
                        // rent.minimum_balance(Mint::LEN),
                        minimum_rent + amount,
                        TokenAccount::LEN as u64,
                        &ID,
                    ),
                    wrap_infos.as_slice(),
                    &[native_sol_auth_seeds],
                )?;

                msg!("Initialise account");
                anchor_spl::token::initialize_account3(CpiContext::new(
                    token_program.to_account_info(),
                    anchor_spl::token::InitializeAccount3 {
                        account: tmp_token_account.to_account_info(),
                        mint: mint.to_account_info(),
                        authority: payer.to_account_info(),
                    },
                ))?;
                msg!("Created & initialised.");
            }
        }
    } else {
        msg!("bad token account ");
        // ok we can use an existing one as long as the owner and mint match
        let tai = tmp_token_account.to_account_info();
        let data: &[u8] = &tai.try_borrow_data()?;
        #[allow(noop_method_call)]
        let tmp_token_account_obj = TokenAccount::try_deserialize(&mut data.clone())?;
        if tmp_token_account_obj.mint != wrapped_sol::ID {
            return Err(SharedError::BadTokenAccountMint.into());
        }

        if tmp_token_account_obj.owner != payer.key() {
            return Err(SharedError::BadTokenAccountOwner.into());
        }

        // transfer the native sol
        match authority_seeds {
            Some(_authority_seeds) => {
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        &payer.key(),
                        &tmp_token_account.key(),
                        amount,
                    ),
                    &[payer.clone(), tmp_token_account.clone()],
                    &[_authority_seeds],
                )?;
            }
            None => {
                anchor_lang::solana_program::program::invoke(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        &payer.key(),
                        &tmp_token_account.key(),
                        amount,
                    ),
                    &[payer.clone(), tmp_token_account.clone()],
                )?;
            }
        }
    }

    match authority_seeds {
        Some(_authority_seeds) => {
            msg!("Transferring native token");
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        to: target_token_account.to_account_info(),
                        from: tmp_token_account.to_account_info(),
                        authority: from_account.to_account_info(),
                    },
                    &[_authority_seeds],
                ),
                amount,
            )?;
            if create_tmp_account {
                // clean up
                anchor_spl::token::close_account(CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    anchor_spl::token::CloseAccount {
                        account: tmp_token_account.to_account_info(),
                        destination: payer.to_account_info(),
                        authority: payer.to_account_info(),
                    },
                    &[_authority_seeds],
                ))?;
            }
        },
        None => {
            msg!("Transferring native token without auth");
            anchor_spl::token::transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        to: target_token_account.to_account_info(),
                        from: tmp_token_account.to_account_info(),
                        authority: from_account.to_account_info(),
                    },
                ),
                amount,
            )?;

            msg!("Cleaning up");
            if create_tmp_account {
                anchor_spl::token::close_account(CpiContext::new(
                    token_program.to_account_info(),
                    anchor_spl::token::CloseAccount {
                        account: tmp_token_account.to_account_info(),
                        destination: payer.to_account_info(),
                        authority: payer.to_account_info(),
                    },
                ))?;
            }
        }
    };

    // clean up if we created this

    Ok(())
}
