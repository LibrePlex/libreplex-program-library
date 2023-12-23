use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to, Mint, MintTo, Token,
    },
};
// use libreplex_shared::sysvar_instructions_program;


use libreplex_shared::SharedError;

use crate::Deployment;

pub fn mint_fungible<'a>(
    deployment: &Account<'a, Deployment>,
    fungible_mint: &Account<'a, Mint>,
    fungible_token_account_escrow: &UncheckedAccount<'a>,
    associated_token_program: &Program<'a, AssociatedToken>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    token_program: &Program<'a, Token>,
    deployment_seeds: &[&[u8]],
) -> Result<()> {
    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &deployment.key(),
        &fungible_mint.key(),
    );
    if expected_token_account != fungible_token_account_escrow.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    if fungible_token_account_escrow
        .to_account_info()
        .data_is_empty()
    {
        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
                mint: fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: fungible_mint.to_account_info(),
                // always mint spl tokens to the program escrow
                to: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
            },
            &[deployment_seeds],
        ),
        deployment.get_fungible_mint_amount(),
    )?;
    Ok(())
}
