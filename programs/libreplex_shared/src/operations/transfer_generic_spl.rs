use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer, Transfer, transfer_checked, TransferChecked};

use crate::SharedError;

pub fn transfer_generic_spl<'info>(
    token_program: &AccountInfo<'info>,
    source_token_account: &AccountInfo<'info>,
    target_token_account: &AccountInfo<'info>,
    source_wallet: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    target_wallet: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    authority_seeds: Option<&[&[&[u8]]]>,
    payer: &AccountInfo<'info>,
    decimals: u8,
    amount: u64,
) -> Result<()> {
    let expected_token_account =
        anchor_spl::associated_token::get_associated_token_address_with_program_id(
            &target_wallet.key(),
            &mint.key(),
            &token_program.key(),
        );

    if expected_token_account != target_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    msg!("Creating token account");

    if target_token_account.data_is_empty() {
        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.clone(),
            anchor_spl::associated_token::Create {
                payer: payer.clone(),
                associated_token: target_token_account.clone(),
                authority: target_wallet.clone(),
                mint: mint.clone(),
                system_program: system_program.clone(),
                token_program: token_program.clone(),
            },
        ))?;
    }

    msg!("Transferring {} (decimals={})", amount, decimals );

    match authority_seeds {
        Some(x) => {
            transfer_checked(
                CpiContext::new_with_signer(
                    token_program.clone(),
                    TransferChecked {
                        to: target_token_account.clone(),
                        from: source_token_account.clone(),
                        authority: source_wallet.clone(),
                        mint: mint.clone(),
                    },
                    x,
                ),
                amount,
                decimals
            )?;
        }
        None => {
            transfer_checked(
                CpiContext::new(
                    token_program.clone(),
                    TransferChecked {
                        to: target_token_account.clone(),
                        from: source_token_account.clone(),
                        authority: source_wallet.clone(),
                        mint: mint.clone(),
                    },
                ),
                amount,
                decimals
            )?;
        }
    }

    Ok(())
}
