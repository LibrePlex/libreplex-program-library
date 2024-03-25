use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{spl_token, Token},
    token_interface::spl_token_2022
};
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment, DeploymentConfig};
use libreplex_liquidity::{cpi::accounts::MintSplCtx, program::LibreplexLiquidity, Liquidity};

use libreplex_shared::{operations::transfer_generic_spl, sysvar_instructions_program};

/// this is where the magic happens.
/// 1) mint an item from the pipeline fair launch (this requires a creator cosign)
/// 2) swap that mint immediately in the fair launch escrow for X tokens
/// 3) keep Y tokens (for now - for liquidity pool later)
/// 4) put X-Y tokens into the swap so that mint_incoming can always be swapped for those
use crate::Pipeline;

// this method does the following:
// 1) calls liquidity mint via CPI (produces an NFT)
// 2) swaps the NFT for SPL
// 3) populates the liquidity pool reserve with the appropriate amount
// 4) puts the remaining SPL into a fungible swap reserve (this is used
//    for creating swaps)

// this endpoint is independent of the type of swap
#[derive(Accounts)]
pub struct AddLiquidityCtx<'info> {
    #[account(mut)]
    pub pipeline: Account<'info, Pipeline>,

    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
    constraint = pipeline.fair_launch_deployment == deployment.key())]
    pub deployment: Box<Account<'info, Deployment>>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub creator_fee_treasury: UncheckedAccount<'info>,

    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], 
        seeds::program = libreplex_fair_launch_program.key(),
        bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

     /// CHECK: Checked via CPI
    #[account(mut, seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        seeds::program = libreplex_fair_launch_program.key(),
        bump)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Passed in via CPI
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Passed in via CPI
    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for integrations
    #[account(mut,
        constraint = pipeline.auth_program_id.eq(&system_program::ID) || auth.key().eq(&pipeline.auth) )]
    pub auth: Signer<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub pipeline_fungible_token_account: UncheckedAccount<'info>,

    /*

        LIQUIDITY-SPECIFIC ACCOUNTS

    */
    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
        constraint = pipeline.liquidity == liquidity.key())]
    pub liquidity: Box<Account<'info, Liquidity>>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in logic. This is where the liquidity providers' share of token goes.
    #[account(mut,
        constraint = liquidity_provider_escrow_token_account.key().eq(
            &anchor_spl::associated_token::get_associated_token_address_with_program_id(
                &liquidity_provider_escrow.key(),
                &fungible_mint.key(),
                fungible_mint.to_account_info().owner,
            )
        ))]
    pub liquidity_provider_escrow_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in logic. This is where the liquidity providers' share of token goes.
    #[account(mut,
        seeds=[b"liq_provider_escrow",pipeline.key().as_ref(), liquidity_provider.key().as_ref()], bump)]
    pub liquidity_provider_escrow: UncheckedAccount<'info>,

    /// CHECK: Can be any account - owns the escrow that holds SPL until bootstrapping, after which they can be withdrawn
    pub liquidity_provider: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub liquidity_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub liquidity_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account()]
    pub libreplex_fair_launch_program: Program<'info, LibreplexFairLaunch>,

    /// CHECK: ID checked in constraint
    pub token_program: Program<'info, Token>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = token_program_22.key() == spl_token_2022::ID
    )]
    pub token_program_22: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub libreplex_liquidity_program: Program<'info, LibreplexLiquidity>,

    pub system_program: Program<'info, System>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn add_liquidity(ctx: Context<AddLiquidityCtx>) -> Result<()> {
    // transfer the outgoing mint into escrow -
    let token_program = &ctx.accounts.token_program;
    let token_program_22 = &ctx.accounts.token_program_22;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;
    let deployment = &ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let libreplex_liquidity_program = &ctx.accounts.libreplex_liquidity_program;
    let hashlist = &ctx.accounts.hashlist;
    let hashlist_marker = &ctx.accounts.hashlist_marker;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let liquidity = &ctx.accounts.liquidity;
    // let liquidity_fungible_token_account = &ctx.accounts.liquidity_fungible_token_account;
    let deployment_fungible_token_account = &ctx.accounts.deployment_fungible_token_account;
    let deployment_non_fungible_token_account = &ctx.accounts.deployment_non_fungible_token_account;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let liquidity_non_fungible_token_account = &ctx.accounts.liquidity_non_fungible_token_account;
    let pipeline_fungible_token_account = &ctx.accounts.pipeline_fungible_token_account;
    let sysvar_instructions_program = &ctx.accounts.sysvar_instructions;
    let pipeline = &mut ctx.accounts.pipeline;
    let liquidity_provider_escrow_token_account = &ctx.accounts.liquidity_provider_escrow_token_account;
    let liquidity_provider_escrow = &ctx.accounts.liquidity_provider_escrow;
    // let liquidity_provider_token_account = &ctx.accounts.liquidity_provider_token_account;
    let payer = &ctx.accounts.payer;
    let liquidity_provider = &ctx.accounts.liquidity_provider;
    let creator_fee_treasury = &ctx.accounts.creator_fee_treasury;
    let pipeline_seeds: &[&[u8]] = &[
        "pipeline".as_bytes(),
        deployment.ticker.as_ref(),
        &[pipeline.bump],
    ];
    let liquidity_fungible_token_account = &mut ctx.accounts.liquidity_fungible_token_account;

    // mint one, the excess goes to pipeline_fungible_token_account.
    libreplex_liquidity::cpi::mint_spl(CpiContext::new_with_signer(
        libreplex_liquidity_program.to_account_info(),
        MintSplCtx {
            /* the inscription root is set to metaplex
             inscription object.
            */
            authority: pipeline.to_account_info(),
            system_program: system_program.to_account_info(),
            payer: payer.to_account_info(),
            deployment: deployment.to_account_info(),
            deployment_config: deployment_config.to_account_info(),
            creator_fee_treasury: creator_fee_treasury.to_account_info(),
            // liquidity treasury is always system program for these
            hashlist: hashlist.to_account_info(),
            hashlist_marker: hashlist_marker.to_account_info(),
            // fungible accounts
            receiver: liquidity_provider_escrow.to_account_info(),
            fungible_token_account_receiver: liquidity_provider_escrow_token_account.to_account_info(),
            fungible_mint: fungible_mint.to_account_info(),
            // passing dummy accounts to these as the pipelines do not use inscriptions
            // would be good to get a version of mint_2022 that ignores inscriptions
            // so as to clean up the interfaces
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            liquidity: liquidity.to_account_info(),
            deployment_fungible_token_account: deployment_fungible_token_account.to_account_info(),
            deployment_non_fungible_token_account: deployment_non_fungible_token_account
                .to_account_info(),
            liquidity_fungible_token_account: liquidity_fungible_token_account.to_account_info(),
            non_fungible_mint: non_fungible_mint.to_account_info(),
            liquidity_non_fungible_token_account: liquidity_non_fungible_token_account.to_account_info(),
            token_program_22: token_program_22.to_account_info(),
            fair_launch: libreplex_fair_launch_program.to_account_info(),
            sysvar_instructions: sysvar_instructions_program.to_account_info(),
        },
        &[pipeline_seeds],
    ))?;

    let token_program_for_fungible = match *fungible_mint.owner {
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("Fungible mint is not owned by tokenkeg or token-2022");
        }
    };

    let pipeline_key = pipeline.key();
    let liquidity_provider_key = liquidity_provider.key();

    let liquidity_provider_escrow_seeds: &[&[u8]] = &[
        "liq_provider_escrow".as_bytes(),
        pipeline_key.as_ref(), liquidity_provider_key.as_ref(),
        &[ctx.bumps.liquidity_provider_escrow],
    ];

    // move swap amount to pipeline escrow - the rest remains in liquidity_provider_escrow_token_account,
    // where it can be reclaimed after LP has been bootstrapped
    transfer_generic_spl(
            &token_program_for_fungible,
            liquidity_provider_escrow_token_account,
            &pipeline_fungible_token_account.to_account_info(),
            liquidity_provider_escrow,
            &fungible_mint.to_account_info(),
            &pipeline.to_account_info(),
            associated_token_program,
            system_program,
            Some(&[liquidity_provider_escrow_seeds]), // pipeline signs
            payer,
            deployment.decimals,
            pipeline.spl_swap_amount_primary,
            &[]
        )?;
    

    pipeline.fungible_chunk_count += 1;
    // somebody else could send spl here as well.
    // we ignore that and just count what we need
    pipeline.fungible_amount_total += pipeline.spl_swap_amount_primary;
    pipeline.fungible_amount_net += pipeline.spl_swap_amount_primary;

    Ok(())
}
