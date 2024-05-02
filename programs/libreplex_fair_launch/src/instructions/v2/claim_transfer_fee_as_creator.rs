

use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}};

use solana_program::program::invoke_signed;
use spl_token_2022::extension::transfer_fee::instruction::withdraw_withheld_tokens_from_accounts;

use crate::{Deployment, DeploymentConfig};


#[derive(Accounts)]
pub struct ClaimTransferFeeAuthAsCreatorCtx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), 
                    deployment.ticker.as_ref()
                ], bump,
        // constraint = deployment.creator == creator.key()
    )]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    #[account(mut,  
        constraint = creator.key() == deployment.creator)]
    pub creator: UncheckedAccount<'info>,

    /// CHECK: Can be anyone.
    #[account(mut,
        token::authority = creator)]
    pub creator_ata: InterfaceAccount<'info, TokenAccount>,

    #[account()]
    pub token_program_22: Program<'info, Token2022>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn claim_transfer_fee_as_creator_handler<'f>(
    ctx: Context<'_, '_, '_, 'f, ClaimTransferFeeAuthAsCreatorCtx<'f>>,
    sources: Vec<Pubkey>,
) -> Result<()> {
    let deployment: &mut Account<'_, Deployment> = &mut ctx.accounts.deployment;

    let deployment_config: &mut Account<'_, DeploymentConfig> = &mut ctx.accounts.deployment_config;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let token_program_22 = &ctx.accounts.token_program_22;
    let creator_ata = &ctx.accounts.creator_ata;
    let creator = &ctx.accounts.creator;
    if !deployment_config.allow_claim_transfer_fee_auth_as_creator {
        panic!("Creator is not allowed to claim transfer fees for this deployment")
    }

    // let mut sources: Vec<&Pubkey> = Vec::new();

    let sources = sources.iter().collect::<Vec<&Pubkey>>();

    let seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    let mut account_infos = vec![
        token_program_22.to_account_info(),
        fungible_mint.to_account_info(),
        creator_ata.to_account_info(),
        creator.to_account_info(),
        deployment.to_account_info(),
    ];

    for s in ctx.remaining_accounts {
        account_infos.push(s.clone())
    }

    invoke_signed(
        &withdraw_withheld_tokens_from_accounts(
            &token_program_22.key(),
            &fungible_mint.key(),
            &creator_ata.key(),
            &deployment.key(),
            &[],
            &sources,
        )?,
        &account_infos,
        &[seeds],
    )?;

    Ok(())
}
