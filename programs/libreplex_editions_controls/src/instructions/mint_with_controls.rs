

use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken, token_2022
};


use libreplex_editions::group_extension_program;
use libreplex_editions::{program::LibreplexEditions, EditionsDeployment};
use libreplex_editions::cpi::accounts::MintCtx;

use crate::{EditionsControls, MinterStats};


use crate::check_phase_constraints;



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MintInput {
    pub phase_index: u32
}


#[derive(Accounts)]
#[instruction(mint_input: MintInput)]

pub struct MintWithControlsCtx<'info> {

    #[account(mut)]
    pub editions_deployment: Box<Account<'info, EditionsDeployment>>,

    #[account(mut,
        seeds = [b"editions_controls", editions_deployment.key().as_ref()],
        bump
    )]
    pub editions_controls: Box<Account<'info, EditionsControls>>,


     /// CHECK: Checked via CPI
     #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(constraint = editions_controls.cosigner_program_id == system_program::ID || signer.key() == editions_deployment.creator)]
    pub signer: Signer<'info>,

    /// CHECK: Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub minter: UncheckedAccount<'info>,

    /// CHECK: Anybody can sign, anybody can receive the inscription
    #[account(init_if_needed,
        payer = payer,
        seeds=[b"minter_stats", editions_deployment.key().as_ref(), minter.key().as_ref()],
        bump,
        space=MinterStats::SIZE)]
    pub minter_stats: Box<Account<'info, MinterStats>>,

    /// CHECK: Anybody can sign, anybody can receive the inscription
    #[account(init_if_needed,
        payer = payer,
        seeds=["minter_stats_phase".as_bytes(), editions_deployment.key().as_ref(), minter.key().as_ref()
        , &mint_input.phase_index.to_le_bytes()],
        bump,
        space=MinterStats::SIZE)]
    pub minter_stats_phase: Box<Account<'info, MinterStats>>,



    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    pub member: Signer<'info>,

    /// CHECK: checked in constraint
    #[account(mut,
    constraint = editions_deployment.group == group.key())]
    pub group: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    
    /// CHECK: Checked in constraint
    #[account(mut,
        constraint = editions_controls.treasury == treasury.key())]
    pub treasury: UncheckedAccount<'info>,


    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: address checked
    #[account(address = group_extension_program::ID)]
    pub group_extension_program: AccountInfo<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    pub libreplex_editions_program: Program<'info, LibreplexEditions>

}



pub fn mint_with_controls(ctx: Context<MintWithControlsCtx>, mint_input: MintInput) -> Result<()> {
    
    let libreplex_editions_program = &ctx.accounts.libreplex_editions_program;
    let editions_deployment = &ctx.accounts.editions_deployment;
    let editions_controls = &mut ctx.accounts.editions_controls;
   
    let hashlist = &ctx.accounts.hashlist;
    let hashlist_marker = &ctx.accounts.hashlist_marker;
    let payer = &ctx.accounts.payer;
    let mint = &ctx.accounts.mint;
    let token_account = &ctx.accounts.token_account;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let minter = &ctx.accounts.minter;
    let group = &ctx.accounts.group;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let minter_stats = &mut ctx.accounts.minter_stats;
    let treasury = &ctx.accounts.treasury;
    let minter_stats_phase = &mut ctx.accounts.minter_stats_phase;
    let group_extension_program = &ctx.accounts.group_extension_program;
    let member = &ctx.accounts.member;
    if mint_input.phase_index >= editions_controls.phases.len() as u32 {
        if editions_controls.phases.is_empty() {
            panic!("No phases added. Cannot mint");
        } else {
            panic!("Attempted to mint with phase {} (max phase {})", mint_input.phase_index, editions_controls.phases.len());
        }
    }
    let current_phase = &editions_controls.phases[mint_input.phase_index as usize]; 
    check_phase_constraints(current_phase,
        minter_stats,
        minter_stats_phase,
        editions_controls);

    msg!("[mint_count] total:{} phase: {}", minter_stats.mint_count, minter_stats_phase.mint_count);
     // update this in case it has been initialised
    // idempotent because the account is determined by the PDA
    minter_stats.wallet = minter.key();
    minter_stats.mint_count += 1; 


    minter_stats_phase.wallet = minter.key();
    minter_stats_phase.mint_count += 1; 

    
    // ok, we are gucci. transfer funds to treasury if applicable

    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: payer.to_account_info(),
                to: treasury.to_account_info(),
            },
        ),
        current_phase.price_amount,
    )?;



    let editions_deployment_key = editions_deployment.key();
    let seeds = &[
        b"editions_controls",
        editions_deployment_key.as_ref(),
        &[ctx.bumps.editions_controls],
    ];

    libreplex_editions::cpi::mint(
        CpiContext::new_with_signer(
            libreplex_editions_program.to_account_info(),
            MintCtx {
                editions_deployment: editions_deployment.to_account_info(),
                hashlist: hashlist.to_account_info(),
                hashlist_marker: hashlist_marker.to_account_info(),
                payer: payer.to_account_info(),
                signer: editions_controls.to_account_info(),
                minter: minter.to_account_info(),
                mint: mint.to_account_info(),
                group: group.to_account_info(),
                token_account: token_account.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                system_program: system_program.to_account_info(),
                group_extension_program: group_extension_program.to_account_info(),
                member: member.to_account_info(), 
            },
            &[seeds]
        ))?;
    Ok(())
}
