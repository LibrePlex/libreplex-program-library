use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::spl_token_2022
};
use libreplex_fair_launch::{Deployment, DeploymentConfig, Hashlist};
use libreplex_liquidity::{cpi::accounts::MintSplCtx, Liquidity};

use libreplex_shared::sysvar_instructions_program;

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

    #[account(mut, seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        seeds::program = libreplex_fair_launch_program.key(),
        bump)]
    pub hashlist: Account<'info, Hashlist>,

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
    #[account(mut)]
    pub liquidity_provider_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in logic. This is where the liquidity providers' share of token goes.
    #[account(mut)]
    pub liquidity_provider: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub liquidity_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = libreplex_fair_launch_program.key() == libreplex_fair_launch::ID
    )]
    pub libreplex_fair_launch_program: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    pub token_program: Program<'info, Token>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = token_program_22.key() == spl_token_2022::ID
    )]
    pub token_program_22: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

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
    let hashlist = &ctx.accounts.hashlist;
    let hashlist_marker = &ctx.accounts.hashlist_marker;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let liquidity = &ctx.accounts.liquidity;
    // let liquidity_fungible_token_account = &ctx.accounts.liquidity_fungible_token_account;
    let deployment_fungible_token_account = &ctx.accounts.deployment_fungible_token_account;
    let deployment_non_fungible_token_account = &ctx.accounts.deployment_non_fungible_token_account;
    let pooled_non_fungible_mint = &ctx.accounts.pooled_non_fungible_mint;
    let pooled_non_fungible_token_account = &ctx.accounts.pooled_non_fungible_token_account;
    let pipeline_fungible_token_account = &ctx.accounts.pipeline_fungible_token_account;
    let sysvar_instructions_program = &ctx.accounts.sysvar_instructions;
    let pipeline = &mut ctx.accounts.pipeline;
    // let liquidity_provider_token_account = &ctx.accounts.liquidity_provider_token_account;
    let payer = &ctx.accounts.payer;
    // let liquidity_provider = &ctx.accounts.liquidity_provider;
    let creator_fee_treasury = &ctx.accounts.creator_fee_treasury;
    let pipeline_seeds: &[&[u8]] = &[
        "pipeline".as_bytes(),
        deployment.ticker.as_ref(),
        &[pipeline.bump],
    ];
    let liquidity_fungible_token_account = &mut ctx.accounts.liquidity_fungible_token_account;

    // let mut balance_before = 0;

    if !liquidity_fungible_token_account.data_is_empty() {
        // let tokenaccount_info_before =
        //     AsRef::<AccountInfo>::as_ref(liquidity_fungible_token_account.as_ref());

        // let mut data: &[u8] = &tokenaccount_info_before.try_borrow_data()?;
        // let acc = TokenAccount::try_deserialize(&mut data)?;

        // balance_before = acc.amount;
    }

    // mint one, the excess goes to pipeline_fungible_token_account.
    libreplex_liquidity::cpi::mint_spl(CpiContext::new_with_signer(
        libreplex_fair_launch_program.to_account_info(),
        MintSplCtx {
            /* the inscription root is set to metaplex
             inscription object.
            */
            authority: pipeline.to_account_info(),
            system_program: system_program.to_account_info(),
            payer: payer.to_account_info(),
            deployment: deployment.to_account_info(),
            deployment_config: deployment_config.to_account_info(),
            // creator fee treasury is always system program for these
            creator_fee_treasury: creator_fee_treasury.to_account_info(),
            hashlist: hashlist.to_account_info(),
            hashlist_marker: hashlist_marker.to_account_info(),

            // fungible accounts
            receiver: pipeline.to_account_info(),
            fungible_token_account_minter: pipeline_fungible_token_account.to_account_info(),
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
            pooled_non_fungible_mint: pooled_non_fungible_mint.to_account_info(),
            pooled_non_fungible_token_account: pooled_non_fungible_token_account.to_account_info(),
            token_program_22: token_program_22.to_account_info(),
            fair_launch: libreplex_fair_launch_program.to_account_info(),
            sysvar_instructions: sysvar_instructions_program.to_account_info(),
        },
        &[pipeline_seeds],
    ))?;

    // let token_program_for_fungible = match *fungible_mint.owner {
    //     spl_token::ID => token_program.to_account_info(),
    //     spl_token_2022::ID => token_program_22.to_account_info(),
    //     _ => {
    //         panic!("Fungible mint is not owned by tokenkeg or token-2022");
    //     }
    // };

    // let tokenaccount_info_after = AsRef::<AccountInfo>::as_ref(liquidity.as_ref());

    // let mut data: &[u8] = &tokenaccount_info_after.try_borrow_data()?;
    // let acc = TokenAccount::try_deserialize(&mut data)?;

    // let balance_after = acc.amount;

    // let total_funds_received = balance_after - balance_before;

    // let lp_provider_share = pipeline.liquidity_provider_amount_in_spl;

    // // a little safety net here
    // if lp_provider_share > total_funds_received {
    //     panic!("Attempted to transfer too much. The code is wrong but caught you anyway.")
    // }

    // if lp_provider_share > 0 {
    //     // transfer the liquidity provider's share
    //     transfer_generic_spl(
    //         &token_program_for_fungible,
    //         pipeline_fungible_token_account,
    //         liquidity_provider_token_account,
    //         &pipeline.to_account_info(),
    //         fungible_mint,
    //         liquidity_provider.as_ref(),
    //         associated_token_program,
    //         system_program,
    //         Some(&[pipeline_seeds]), // pipeline signs
    //         payer,
    //         deployment.decimals,
    //         lp_provider_share,
    //     )?;
    // }

    // pipeline.fungible_chunk_count += 1;
    // // somebody else could send spl here as well.
    // // we ignore that and just count what we need
    // pipeline.fungible_amount_total += total_funds_received - lp_provider_share;
    // pipeline.fungible_amount_net += total_funds_received - lp_provider_share;

    Ok(())
}
