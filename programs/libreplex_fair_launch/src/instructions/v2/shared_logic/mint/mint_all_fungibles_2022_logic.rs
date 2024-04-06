use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, set_authority, MintTo, SetAuthority},
};
// use libreplex_shared::sysvar_instructions_program;

use libreplex_shared::SharedError;

use crate::{Deployment, DeploymentConfig};


pub fn revoke_mint_auths<'a>(
    deployment: &Account<'a, Deployment>,
    token_program: &UncheckedAccount<'a>,
    fungible_mint: &AccountInfo<'a>,
    deployment_seeds: &[&[u8]],
) -> Result<()> {
    msg!("Removing freeze auth");
    
    set_authority(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SetAuthority {
                current_authority: deployment.to_account_info(),
                account_or_mint: fungible_mint.clone(),
            },
            &[deployment_seeds],
        ),
        anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::FreezeAccount,
        None,
    )?;

    msg!("Removing mint authority");
    // ok we are at max mint
    set_authority(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SetAuthority {
                current_authority: deployment.to_account_info(),
                account_or_mint: fungible_mint.clone(),
            },
            &[deployment_seeds],
        ),
        anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::MintTokens,
        None,
    )?;

    Ok(())
}

pub fn mint_all_fungibles<'a>(
    deployment: &Account<'a, Deployment>,
    fungible_mint: &AccountInfo<'a>,
    fungible_token_account_escrow: &AccountInfo<'a>,
    associated_token_program: &Program<'a, AssociatedToken>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    token_program: &UncheckedAccount<'a>,
    deployment_seeds: &[&[u8]],
    revoke_auths: bool,
    deployment_config: &DeploymentConfig,
) -> Result<()> {
    msg!("Mint all fungibles {}", token_program.key());
    let expected_token_account =
        anchor_spl::associated_token::get_associated_token_address_with_program_id(
            &deployment.key(),
            &fungible_mint.key(),
            &token_program.key(),
        );
    if expected_token_account != fungible_token_account_escrow.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }
    if fungible_token_account_escrow
        .to_account_info()
        .data_is_empty()
    {
        msg!("{}", payer.key());
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
                mint: fungible_mint.clone(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    msg!("Minting {}", token_program.key());
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: fungible_mint.clone(),
                // always mint spl tokens to the program escrow
                to: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
            },
            &[deployment_seeds],
        ),
        deployment.get_max_fungible_mint_amount_per_deployment(&deployment_config.multiplier_limits),
    )?;

    if revoke_auths {
        revoke_mint_auths(deployment, token_program, fungible_mint, deployment_seeds)?;
    }

    Ok(())
}
