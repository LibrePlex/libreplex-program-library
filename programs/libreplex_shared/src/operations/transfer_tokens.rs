use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
};
use solana_program::program_pack::Pack;

use crate::SharedError;

pub fn transfer_tokens<'info>(
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
    amount: u64,
) -> Result<()> {
    // simple. move the token from source token account to the target token account

    // simple. move the token from source token account to the target token account

    let expected_token_account = get_associated_token_address_with_program_id(
        &target_wallet.key(),
        &mint.key(),
        &token_program.key(),
    );

    // let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
    //     &target_wallet.key(), &mint.key());

    if expected_token_account != target_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    msg!("{}", amount);


    let acc_data = &mint.try_borrow_data().unwrap()[..][..];
    let mint_obj = spl_token_2022::state::Mint::unpack_from_slice(acc_data).unwrap();
    drop(acc_data);

    if target_token_account.data_is_empty() {
        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
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

    msg!("b");

    let ix = spl_token_2022::instruction::transfer_checked(
        token_program.key,
        source_token_account.key,
        mint.key,
        target_token_account.key,
        source_wallet.key,
        &[],
        amount,
        mint_obj.decimals
    )?;

    msg!("c");

    match authority_seeds {
        Some(x) => {
            solana_program::program::invoke_signed(
                &ix,
                &[
                    source_token_account.clone(),
                    mint.clone(),
                    target_token_account.clone(),
                    source_wallet.clone(),
                    token_program.clone()
                ],
                x,
            )?;
        }
        None => {


            // &token_program.key(),
            // &source_token_account.key(),
            // &mint.key(),
            // &target_token_account.key(),
            // &source_wallet.key(),
            // &[&source_wallet.key()],

            solana_program::program::invoke(
                &ix,
                &[
                    source_token_account.clone(),
                    mint.clone(),
                    target_token_account.clone(),
                    source_wallet.clone(),
                ],
            )?;
        }
    }

    Ok(())
}
