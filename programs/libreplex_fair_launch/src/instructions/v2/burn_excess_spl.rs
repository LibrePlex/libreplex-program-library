use anchor_lang::prelude::*;

use anchor_spl::token_interface::TokenAccount;

use crate::{Deployment, DeploymentConfig};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct BurnExcessSplCtx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    #[account(mut,
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump
    )]
    pub deployment_config: Account<'info, DeploymentConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        constraint = !deployment.require_creator_cosign || deployment.creator.eq(&signer.key())
    )]
    pub signer: Signer<'info>,

    /// CHECK: checked above against deployment.fungible_mint
    #[account(mut)]
    pub fungible_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = token_program.key().eq(fungible_mint.owner)
    )]
    pub token_program: UncheckedAccount<'info>
}

pub fn burn_excess_spl<'info>(
    ctx: Context<'_, '_, '_, 'info, BurnExcessSplCtx<'info>>,
) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    
    
    let fungible_mint = &ctx.accounts.fungible_mint;
    let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
    let deployment = &mut ctx.accounts.deployment;
    let deployment_config = &mut ctx.accounts.deployment_config;
    
    if deployment.number_of_tokens_issued < deployment.max_number_of_tokens {
        panic!("Cannot burn before mint-out")
    }

    if !deployment_config.allow_burn {
        panic!("This deployment does not allow burning")
    }

    let ticker = deployment.ticker.clone();

    let authority_seeds = &[
        "deployment".as_bytes(),
        ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];


    anchor_spl::token_interface::burn(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            anchor_spl::token_interface::Burn {
                mint: fungible_mint.to_account_info(),
                from: fungible_escrow_token_account.to_account_info(),
                authority: deployment.to_account_info(),
            },
            &[authority_seeds]
        ),
        deployment_config.spl_excess_in_escrow,
    )?;

    
    deployment_config.spl_excess_in_escrow = 0;

    Ok(())
}
