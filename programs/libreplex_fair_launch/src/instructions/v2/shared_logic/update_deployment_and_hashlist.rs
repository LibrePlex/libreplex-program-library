use anchor_lang::prelude::*;
use anchor_spl::token::{
        set_authority, spl_token::instruction::AuthorityType, Mint, SetAuthority,
        Token,
    };
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;

use crate::{
    add_to_hashlist, Deployment, MintEvent,
};


pub fn update_deployment_and_hashlist<'a>(
    deployment: &mut Account<'a, Deployment>,
    fungible_mint: &Account<'a, Mint>,
    token_program: &Program<'a, Token>,
    deployment_seeds: &[&[u8]],
    hashlist: &mut UncheckedAccount<'a>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    root_key: Pubkey,
    inscription_summary: &Account<'a, InscriptionSummary>,
) -> Result<()> {
    deployment.number_of_tokens_issued += 1;

    if deployment.number_of_tokens_issued == deployment.max_number_of_tokens {
        if fungible_mint.freeze_authority.is_some() {
            // ok we are at max mint
            set_authority(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    SetAuthority {
                        current_authority: deployment.to_account_info(),
                        account_or_mint: fungible_mint.to_account_info(),
                    },
                    &[deployment_seeds],
                ),
                AuthorityType::FreezeAccount,
                None,
            )?;
        }

        if fungible_mint.mint_authority.is_some() {
            // ok we are at max mint
            set_authority(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    SetAuthority {
                        current_authority: deployment.to_account_info(),
                        account_or_mint: fungible_mint.to_account_info(),
                    },
                    &[deployment_seeds],
                ),
                AuthorityType::MintTokens,
                None,
            )?;
        }
    }
    if deployment.number_of_tokens_issued <= 262144 {
        add_to_hashlist(
            deployment.number_of_tokens_issued as u32,
            hashlist,
            payer,
            system_program,
            &root_key,
            &deployment.key(),
            inscription_summary.inscription_count_total,
        )?;
    }
    emit!(MintEvent {
        mint: root_key,
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });
    Ok(())
}
