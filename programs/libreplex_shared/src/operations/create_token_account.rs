use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;

use anchor_spl::token::ID;

pub fn create_token_account_signed<'info> (
    token_account_owner: &AccountInfo<'info>,
    source_token_account: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    extra_lamports: u64) -> Result<()> {

    let wrap_infos = vec![
        token_account_owner.to_account_info(), 
        source_token_account.to_account_info()
    ];
    // wrapped sol token account
    let minimum_rent = Rent::get()?.minimum_balance(TokenAccount::LEN) ;
    invoke_signed(
        &system_instruction::create_account(
            &token_account_owner.key(),
            &source_token_account.key(),
            // rent.minimum_balance(Mint::LEN),
            minimum_rent + extra_lamports,
            TokenAccount::LEN as u64,
            &ID,
        ),
        wrap_infos.as_slice(),
        &[&authority_seeds, &wrapped_sol_authority_seeds]
    )?;

    anchor_spl::token::initialize_account3(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            anchor_spl::token::InitializeAccount3 {
                account: source_token_account.to_account_info(),
                mint: mint.to_account_info(),
                authority: pod_creator.to_account_info(),
            },
            &[&authority_seeds]
        ),
    )?;
    Ok(())
}