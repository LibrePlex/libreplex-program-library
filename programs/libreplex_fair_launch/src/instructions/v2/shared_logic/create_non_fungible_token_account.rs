use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Mint,
};
// use libreplex_shared::sysvar_instructions_program;

use libreplex_shared::SharedError;


pub fn create_non_fungible_token_account<'a>(
    inscriber: &UncheckedAccount<'a>,
    non_fungible_mint: &Account<'a, Mint>,
    non_fungible_token_account: &UncheckedAccount<'a>,
    associated_token_program: &Program<'a, AssociatedToken>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    token_program: &UncheckedAccount<'a>,
) -> Result<()> {
    let expected_non_fungible_token_account =
        anchor_spl::associated_token::get_associated_token_address_with_program_id(
            &inscriber.key(),
            &non_fungible_mint.key(),
            &token_program.key()

        );
    if expected_non_fungible_token_account != non_fungible_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    if non_fungible_token_account.to_account_info().data_is_empty() {
        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: non_fungible_token_account.to_account_info(),
                authority: inscriber.to_account_info(),
                mint: non_fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }
    Ok(())
    
}
