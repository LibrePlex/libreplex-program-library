

use anchor_lang::prelude::*;

use crate::SharedError;

pub fn transfer_non_pnft<'info>(
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
    amount: u64
) -> Result<()> {
    
    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &target_wallet.key(), &mint.key());

    if expected_token_account != target_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }   
    msg!("{}", amount);

    if target_token_account.data_is_empty() {

        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: target_token_account.to_account_info(),
                authority: target_wallet.to_account_info(),
                mint: mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    match authority_seeds {
        Some(x) => {
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        to: target_token_account.clone(),
                        from: source_token_account.clone(),
                        authority: source_wallet.clone(),
                    },
                    x
                ),
                amount
            )?;

        }, None => {
            anchor_spl::token::transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        to: target_token_account.clone(),
                        from: source_token_account.clone(),
                        authority: source_wallet.clone(),
                    }
                ),
                amount
            )?;
        }
    }

    Ok(())
}
