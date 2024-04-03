

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;


use libreplex_shared::sysvar_instructions_program;



use crate::{
    errors::FairLaunchError, Deployment, HashlistMarker, mint_legacy_logic,
};

#[derive(Accounts)]
pub struct MintLegacyCtx<'info> {
    #[account(mut,
       seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut, 
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = 8 + HashlistMarker::INIT_SPACE,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub inscriber: UncheckedAccount<'info>,

    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: Account<'info, Mint>,

    /// CHECK: Checked in logic, created as necessary
    #[account(
        mut,
    )]
    pub fungible_token_account_escrow: UncheckedAccount<'info>,

    // legacy - TokenKeg
    // libre - Token22
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::freeze_authority = deployment,
        mint::authority = deployment // will be a non
    )]
    pub non_fungible_mint: Account<'info, Mint>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,
    
    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_metadata: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_masteredition: UncheckedAccount<'info>,

    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: sent via CPI to libreplex_inscriptions_program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,



    /* BOILERPLATE PROGRAM ACCOUNTS */
    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,

    
    /// CHECK: address checked
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

}

pub fn mint_legacy<'info>(ctx: Context<'_, '_, '_, 'info, MintLegacyCtx<'info>>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;

    if deployment.require_creator_cosign {
        panic!("Only launches without creator cosign can currently use v1 methods")
    }

    // to be discussed w/ everybody and feedback. Not strictly in line with BRC 20 thinking
    // but seems pointless to issue tokens if they can never be valid
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        return Err(FairLaunchError::MintedOut.into());
    }

    if deployment.migrated_from_legacy {
        return Err(FairLaunchError::LegacyMigrationsAreMintedOut.into());
    }
    

    let hashlist = &mut ctx.accounts.hashlist;

    let inscription_summary = &ctx.accounts.inscription_summary;

    let payer = &ctx.accounts.payer;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let non_fungible_metadata =  &ctx.accounts.non_fungible_metadata;
    let non_fungible_masteredition = &ctx.accounts.non_fungible_masteredition;
    
    let non_fungible_token_account = &ctx.accounts.non_fungible_token_account;
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let fungible_token_account_escrow = &ctx.accounts.fungible_token_account_escrow;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let metadata_program = &ctx.accounts.metadata_program;
    let inscriber = &ctx.accounts.inscriber;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let sysvar_instructions_program = &ctx.accounts.sysvar_instructions;

    
    mint_legacy_logic(deployment, inscriptions_program, inscription_summary, non_fungible_mint, inscription_v3, system_program, payer, inscription_data, 
        fungible_mint, fungible_token_account_escrow, associated_token_program, token_program, inscriber, non_fungible_token_account, non_fungible_metadata, non_fungible_masteredition, metadata_program, sysvar_instructions_program, hashlist,
    &ctx.accounts.hashlist_marker,
    ctx.bumps.deployment)?;

    Ok(())
}
