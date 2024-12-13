use spl_token_metadata_interface::instruction::update_authority;
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token_2022::{self};
use spl_pod::optional_keys::OptionalNonZeroPubkey; 

use crate::{EditionsDeployment, errors::EditionsError};

/// TODO: Add hashlist marker or hashlist to verify mint?
#[derive(Accounts)]
pub struct ClaimMintAuthorityCtx<'info> {

    #[account(mut,
        seeds = ["editions_deployment".as_ref(), editions_deployment.symbol.as_ref()], bump)]
    pub editions_deployment: Account<'info, EditionsDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: AccountInfo<'info>,
    
    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn claim_mint_authority<'info>(ctx: Context<'_, '_, '_, 'info, ClaimMintAuthorityCtx<'info>>) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let editions_deployment = &ctx.accounts.editions_deployment;
    let payer = &ctx.accounts.payer;

    require!(editions_deployment.max_number_of_tokens == editions_deployment.number_of_tokens_issued, EditionsError::MintNotComplete);
    require!(editions_deployment.creator.key() == payer.key(), EditionsError::InvalidCreator);
    require!(payer.is_signer, EditionsError::InvalidSigner);

    let deployment_seeds: &[&[u8]] = &[
            "editions_deployment".as_bytes(),
            editions_deployment.symbol.as_ref(),
            &[ctx.bumps.editions_deployment],
        ];

    let account_infos = [
        payer.to_account_info(),
        editions_deployment.to_account_info(),
        mint.to_account_info(),
        token_program.to_account_info(),
    ];

    let payer = OptionalNonZeroPubkey::try_from(Some(payer.to_account_info().key()))?;

    let update_authority_ix = update_authority(
        &spl_token_2022::ID,
        &mint.key(),
        &editions_deployment.key(),
        payer
    );

    invoke_signed(&update_authority_ix, &account_infos, &[deployment_seeds])?;

    Ok(())
}