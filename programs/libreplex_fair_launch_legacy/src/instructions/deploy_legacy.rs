use anchor_lang::prelude::*;
use anchor_spl::{token::Token, associated_token::AssociatedToken};
use anchor_spl::token::TokenAccount;

use libreplex_inscriptions::InscriptionSummary;
use libreplex_shared::create_metadata_and_masteredition;
use libreplex_fair_launch::DeployInput;
use mpl_token_metadata::types::Creator;



pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}   


#[derive(Accounts)]
#[instruction(input: DeployInput)]
pub struct DeployLegacyCtx<'info>  {
    
    /// CHECK: Passed into libreplex_src20 via CPI
    pub deployment: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Can be anything. Used for PDA wrapper programs like legacy spl
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    // gets created, passed into spl_20 via  CPI
    #[account(mut)]
    pub fungible_mint: Signer<'info>,

    // gets created, passed into spl_20 via  CPI
    #[account(mut)]
    pub fungible_metadata: UncheckedAccount<'info>,

    // gets created, passed into spl_20 via  CPI
    #[account(mut)]
    pub fungible_master_edition: UncheckedAccount<'info>,
    
    // gets created, passed into spl_20 via  CPI
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    // gets created, passed into spl_20 via  CPI
    #[account(mut)]
    pub non_fungible_metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    // CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    // CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    rent: Sysvar<'info, Rent>,


    /// CHECK: Id checked in constraint
    #[account(
        constraint = metadata_program.key() == mpl_token_metadata::ID
    )]
    #[account()]
    pub metadata_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,

    #[account(
        constraint = libreplex_src20_program.key() == libreplex_fair_launch::ID
    )]
    libreplex_src20_program: UncheckedAccount<'info>

}
/* 
    this method has two jobs:
        1) to forward a cpi call to spl20
        2) create legacy metadata for the newly created mints - fungible and nonfungible

*/
pub fn deploy(ctx: Context<DeployLegacyCtx>, input: &DeployInput) -> Result<()> {
   
   let libreplex_src20_program = &ctx.accounts.libreplex_src20_program;

   let deployment = &mut ctx.accounts.deployment;
   let system_program = &ctx.accounts.system_program;
   let payer = &ctx.accounts.payer;
   let inscriptions_program = &ctx.accounts.inscriptions_program;
   let inscription_summary = &ctx.accounts.inscription_summary;
   let inscription = &ctx.accounts.inscription;
   let inscription_v3 = &ctx.accounts.inscription_v3;
   let inscription_data = &ctx.accounts.inscription_data;
   let fungible_mint = &ctx.accounts.fungible_mint;
   let fungible_metadata= &ctx.accounts.fungible_metadata;
   let non_fungible_mint = &ctx.accounts.non_fungible_mint;
   let non_fungible_metadata= &ctx.accounts.non_fungible_metadata;
   let fungible_escrow_token_account = &ctx.accounts.fungible_escrow_token_account;
   let token_program = &ctx.accounts.token_program;
   let associated_token_program = &ctx.accounts.associated_token_program;
   let sysvar_instructions = &ctx.accounts.sysvar_instructions;
   let metadata_program = &ctx.accounts.metadata_program;

    libreplex_fair_launch::cpi::deploy(
        CpiContext::new(
            libreplex_src20_program.to_account_info(),
            libreplex_fair_launch::cpi::accounts::DeployCtx {
                deployment: deployment.to_account_info(),
                payer: payer.to_account_info(),
                fungible_mint: fungible_mint.to_account_info(),
                fungible_escrow_token_account: fungible_escrow_token_account.to_account_info(),
                non_fungible_mint: fungible_mint.to_account_info(),
                inscription_summary: inscription_summary.to_account_info(),
                inscription: inscription.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                inscriptions_program: inscriptions_program.to_account_info(),
                system_program: system_program.to_account_info(),
                sysvar_instructions: sysvar_instructions.to_account_info()
            },
        ),
        input.clone()
    )?;

    // create the fungible metadata

    let ticker = &input.ticker;
    let offchain_url = &input.offchain_url;
    create_metadata_and_masteredition(
        &payer.to_account_info(),
        &payer.to_account_info(),
        &fungible_mint.to_account_info(),
        &fungible_metadata.to_account_info(),
        None,
        &token_program.to_account_info(),
        &metadata_program.to_account_info(),
        &system_program.to_account_info(),
        None,
        // rent.to_account_into(),
        ticker,
        ticker,
        offchain_url,
        0,
        None,
        None, // this is the supply of the editions. always 0
        None,
        true,
    )?;


    // create non-fungible metadata
    create_metadata_and_masteredition(
        &payer.to_account_info(),
        &payer.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &non_fungible_metadata.to_account_info(),
        None,
        &token_program.to_account_info(),
        &metadata_program.to_account_info(),
        &system_program.to_account_info(),
        None,
        // rent.to_account_into(),
        ticker,
        ticker,
        offchain_url,
        0,
        Some([Creator {
            address: payer.key(),
            verified: false,
            share: 100
        }].to_vec()),
        None, // this is the supply of the editions. always 0
        None,
        false,
    )?;

 

    Ok(())
}