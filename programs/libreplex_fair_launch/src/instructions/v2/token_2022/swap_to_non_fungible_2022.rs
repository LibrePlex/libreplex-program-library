use crate::{DeploymentConfig, HashlistMarker};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{spl_token, Token},
    token_2022,
    token_interface::{Token2022, TokenAccount},
};
use libreplex_shared::operations::transfer_generic_spl;
// use libreplex_shared::operations::transfer_non_pnft;

use crate::Deployment;

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

#[derive(Accounts)]
pub struct SwapToNonFungible2022Ctx<'info> {
    #[account(
        mut,
        constraint = deployment.fungible_mint == fungible_mint.key(),
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump
    )]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: Checked by deserialize in transfer logic
    #[account(
        // deployment must be executed by the payer 
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump
    )]
    pub deployment_config: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /* fungible accounts */
    /// CHECK: Checked in constraint
    #[account(mut,
        constraint = fungible_mint.owner.eq(&token_2022::ID) || fungible_mint.owner.eq(&spl_token::ID))]
    pub fungible_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = fungible_mint,
        token::authority = payer, // coimes out of the payer account
    )]
    pub fungible_source_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Checked in transfer logic
    #[account(mut)]
    pub fungible_target_token_account: UncheckedAccount<'info>,

    /* NON-FUNGIBLE COMES OUT OF THE ESCROW */
    /// CHECK: Checked in constraint
    #[account(mut,
        constraint = non_fungible_mint.owner.eq(&token_2022::ID) || non_fungible_mint.owner.eq(&spl_token::ID))]
    pub non_fungible_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = deployment, // escrow is always owned by the deployment
    )]
    pub non_fungible_source_token_account: InterfaceAccount<'info, TokenAccount>,

    // verifies that the NFT coming out of the escrow has
    // been registered with the escrow, either via minting or importing
    // from legacy hashlist.
    // this is important in case the escrow ends up holding other crap
    // that people send in just for s**ts and giggles
    #[account(seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    /// CHECK: Checked in transfer logic
    #[account(mut)]
    pub non_fungible_target_token_account: UncheckedAccount<'info>,

    pub token_program_22: Program<'info, Token2022>,

    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,
}

pub fn swap_to_nonfungible_2022<'a>(ctx: Context<'_,'_,'_,'a, SwapToNonFungible2022Ctx<'a>>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;

    let payer = &ctx.accounts.payer;
    let non_fungible_source_token_account = &ctx.accounts.non_fungible_source_token_account;
    let non_fungible_target_token_account = &ctx.accounts.non_fungible_target_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;

    let source_wallet = &ctx.accounts.payer;
    let fungible_source_token_account = &ctx.accounts.fungible_source_token_account;
    let fungible_target_token_account = &ctx.accounts.fungible_target_token_account;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let deployment = &mut ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;

    let token_program_22 = &ctx.accounts.token_program_22;

    // simples. two steps:
    // 1) move the fungible into the escrow

    let source_token_program = match *non_fungible_mint.owner {
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("How could you do this to me")
        }
    };

    let target_token_program = match *fungible_mint.owner {
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("How could you do this to me")
        }
    };

    
    let mut fungible_amount_to_transfer = deployment.get_fungible_mint_amount(&ctx.accounts.hashlist_marker);
    if !deployment_config.data_is_empty() {
        
        let tai = deployment_config.to_account_info();
        let mut data: &[u8] = &tai.try_borrow_data()?;
        let deployment_config_object = DeploymentConfig::try_deserialize(&mut data)?;
     
        if deployment_config_object.transfer_fee_in_basis_points > 0 {
            // where there is deflation, adjust accordingly
            let mut numerator = (deployment.get_fungible_mint_amount(&ctx.accounts.hashlist_marker) as u128)
                .checked_mul(10_000_u128)
                .unwrap();
            let denominator = 10_000_u128
                .checked_sub(deployment_config_object.transfer_fee_in_basis_points as u128)
                .unwrap();

            let remainder = numerator.checked_rem(denominator);

            if let Some(x) = remainder {
                if x > 0 {
                    numerator = numerator
                        .checked_add(denominator)
                        .unwrap()
                        .checked_sub(x)
                        .unwrap();
                }
            }

            fungible_amount_to_transfer = numerator.checked_div(denominator).unwrap() as u64;
        }
    };

    // msg!("Fungible amount to transfer: {}", fungible_amount_to_transfer);
    transfer_generic_spl(
        &target_token_program.to_account_info(),
        &fungible_source_token_account.to_account_info(),
        &fungible_target_token_account.to_account_info(),
        &source_wallet.to_account_info(),
        &fungible_mint.to_account_info(),
        &deployment.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        None,
        &payer.to_account_info(),
        deployment.decimals,
        // we need to add deflationary adjustment if applicable
        fungible_amount_to_transfer,
        ctx.remaining_accounts,
    )?;
    deployment.escrow_non_fungible_count -= 1;

    let authority_seeds = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    // 2) move the non_fungible_mint out of the escrow

    transfer_generic_spl(
        &source_token_program.to_account_info(),
        &non_fungible_source_token_account.to_account_info(),
        &non_fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &source_wallet.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]), // payer signs
        &payer.to_account_info(),
        0,
        1,
        ctx.remaining_accounts,
    )?;

    // We have crossed the NFT / Defi barrier. As a side effect have a splittable SPL 20

    Ok(())
}
