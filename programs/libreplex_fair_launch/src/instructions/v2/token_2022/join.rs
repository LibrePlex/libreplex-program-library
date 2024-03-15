use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022, token_interface::Mint
};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_2022::extension::group_member_pointer::GroupMemberPointer;
use spl_token_2022::extension::BaseStateWithExtensions;
use spl_token_2022::extension::metadata_pointer::MetadataPointer;



use crate::MintInput;
use crate::{
    Deployment, HashlistMarker,
    mint_token2022_logic, DeploymentConfig,
};

#[derive(Accounts)]
pub struct JoinCtx<'info> {
    #[account(mut,
       has_one = fungible_mint,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

    /// CHECK: checked in constraint
    #[account(mut,
        constraint = deployment_config.creator_fee_treasury == creator_fee_treasury.key())] 
    pub creator_fee_treasury: UncheckedAccount<'info>,



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

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    #[account(mut, owner = spl_token_2022::ID)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Will check in instruction
    #[account(associated_token::mint = non_fungible_mint, 
        associated_token::authority = non_fungible_token_account_owner, associated_token::token_program = token_program)]
    pub non_fungible_token_account: InterfaceAccount<'info, TokenAccount>,

    pub non_fungible_token_account_owner: Signer<'info>,
    

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn join_handler<'info>(ctx: Context<'_, '_, '_, 'info, JoinCtx<'info>>, input: MintInput) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let deployment_config = &mut ctx.accounts.deployment_config;
    let payer = &ctx.accounts.payer; 
    let signer = &ctx.accounts.signer;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_token_account = &ctx.accounts.non_fungible_token_account;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let fungible_mint = &ctx.accounts.fungible_mint;

    let creator_fee_treasury = &mut ctx.accounts.creator_fee_treasury;
    let hashlist = &mut ctx.accounts.hashlist;
    
    if !deployment.require_creator_cosign {
        panic!("Joins require a co signer")
    }
   
    if non_fungible_token_account.amount != 1 {
        panic!("Can only join NFTs")
    }

    let non_fungible_mint_data = non_fungible_mint.try_borrow_data()?;
    let mint_with_extensions = spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(
        &non_fungible_mint_data,
    )?;

    let mint_base = &mint_with_extensions.base;

    if mint_base.supply != 1 || 
        mint_base.mint_authority.is_some() || 
        mint_base.freeze_authority.expect("Freeze authority not provided") != deployment.key() {
        panic!("Invalid join mint")
    }

    mint_with_extensions.get_extension::<MetadataPointer>()?;
    let group_member_ptr = mint_with_extensions.get_extension::<GroupMemberPointer>()?;

    if group_member_ptr.authority != OptionalNonZeroPubkey::try_from(Some(deployment.key()))? {
        panic!("Invalid group pointer")
    }

    mint_token2022_logic(
        deployment, 
        deployment_config,
        creator_fee_treasury,
        &fungible_mint.to_account_info(),
        non_fungible_mint, 
        system_program, 
        payer, 
        associated_token_program, 
        token_program, 
        ctx.accounts.non_fungible_token_account_owner.as_ref(), 
        non_fungible_token_account.as_ref(), 
        hashlist,
        &mut ctx.accounts.hashlist_marker,
        ctx.bumps.deployment,
        ctx.remaining_accounts, signer, false, input)?;

    Ok(())
}