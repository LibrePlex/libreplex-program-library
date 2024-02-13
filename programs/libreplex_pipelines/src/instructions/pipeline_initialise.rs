use anchor_lang::prelude::*;
use libreplex_fair_launch::{cpi::accounts::InitialiseV2Ctx, InitialiseInputV2, HYBRID_DEPLOYMENT_TYPE};
use libreplex_liquidity::{InitialiseInput, DEPLOYMENT_TYPE_SPL};
use libreplex_liquidity::cpi::accounts::Initialise as InitialiseV2CtxLiquidity;
use crate::{Filter, Pipeline};



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialisePipeline {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub decimals: u8,
    pub ticker: String,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub creator_fee_treasury: Pubkey,
    pub creator_fee_per_mint_in_lamports: u64,
    pub filter: Filter,
    pub lp_ratio: u16,
    pub pool_fee_basis_points: u64,
    pub liquidity_seed: Pubkey,
    pub liquidity_provider_amount_in_spl: u64
    // this allows for interesting dynamics
}




#[derive(Accounts)]
#[instruction(input: InitialisePipeline)]
pub struct InitialisePipelineCtx<'info> {
    #[account(init,
        space = Pipeline::SIZE,
        payer = payer,
        // pipeline id can always be derived from the output launch ticker
        seeds=[b"pipeline",input.ticker.as_bytes()],
        bump
    )]
    pub pipeline: Account<'info, Pipeline>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub liquidity: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub deployment: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub deployment_config: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for more wrapper contracts
    #[account(mut, 
        // for now limit this to be the same as payer
        constraint = auth.key() == payer.key()
    )]
    pub auth: Signer<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = libreplex_fair_launch_program.key().eq(&libreplex_fair_launch::ID)
    )]
    pub libreplex_fair_launch_program: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = libreplex_fair_launch_program.key().eq(&libreplex_fair_launch::ID)
    )]
    pub libreplex_liquidity_program: UncheckedAccount<'info>,


    #[account()]
    pub system_program: Program<'info, System>,
}



pub fn initialise_pipeline(
    ctx: Context<InitialisePipelineCtx>,
    input: InitialisePipeline,
) -> Result<()> {
    
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let auth = &ctx.accounts.auth;
    let deployment = &ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let pipeline = &mut ctx.accounts.pipeline;
    let liquidity = &ctx.accounts.liquidity;
    let libreplex_liquidity_program = &ctx.accounts.libreplex_liquidity_program;

    let fair_launch_input = input.clone();


    // we add a creator program - this means mints and swaps can only happen with
    // the signature of the pipelines program
    pipeline.filter = input.filter;
    pipeline.fair_launch_deployment = deployment.key();
    pipeline.processed_item_count = 0;
    pipeline.auth = auth.key();
    pipeline.liquidity_provider_amount_in_spl = input.liquidity_provider_amount_in_spl;
    
    let clock = Clock::get()?;
    pipeline.creation_time = clock.unix_timestamp;
    pipeline.bump = ctx.bumps.pipeline;

    libreplex_fair_launch::cpi::initialise_v2(
        CpiContext::new(
            libreplex_fair_launch_program.to_account_info(),
            InitialiseV2Ctx {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                deployment: deployment.to_account_info(),
                deployment_config: deployment_config.to_account_info(),
                // important - FAIR LAUNCH creator be the liquidity account
                // as the mint is called via liquidity 
                creator: liquidity.to_account_info(),
            },
        ),
        // this has the cosigner specified
        InitialiseInputV2 {
            limit_per_mint: input.limit_per_mint,
            max_number_of_tokens: input.max_number_of_tokens,
            decimals: input.decimals,
            ticker: input.ticker,
            deployment_template: "".to_owned(),
            mint_template: "".to_owned(),
            offchain_url: fair_launch_input.offchain_url,
            
            // cosigner is the liquidity here, NOT pipeline
            creator_cosign_program_id: Some(libreplex_liquidity_program.key()),
            use_inscriptions: false,
            deployment_type: HYBRID_DEPLOYMENT_TYPE,
            creator_fee_treasury: payer.key(), // this is irrelevant since no creator fees are paid anyway
            creator_fee_per_mint_in_lamports: fair_launch_input.creator_fee_per_mint_in_lamports,
            deflation_rate_per_swap: 0,
        },
    )?;

    // and initialise liquidity too

    libreplex_liquidity::cpi::initialise(
        CpiContext::new(
            libreplex_liquidity_program.to_account_info(),
            InitialiseV2CtxLiquidity {
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                // can to liquidity can only be made via the pipeline program
                // as any mint needs to be accompanied by a swap
                authority: pipeline.to_account_info(),
                // this doesn't matter as we do not allow for additional
                // creator fees for swaps at this point
                treasury: system_program.to_account_info(),
                liquidity: liquidity.to_account_info(),
            },
        ),
        // this has the cosigner specified
        InitialiseInput {
            // deployment type set to spl - this doesn't mint extra stuff, but grabs
            // some SPL for the LP reserve and gives the minter a bunch of SPL 
            // instead of NFT
            deployment_type: DEPLOYMENT_TYPE_SPL,
            seed: input.liquidity_seed,
            deployment: deployment.key(),
            bootstrap_start_time: None, 
            bootstrap_requires_sold_out: true,
            creator_basis_points: 0,
            lp_ratio: input.lp_ratio,
            pool_fee_basis_points: input.pool_fee_basis_points,
            cosigner_program_id: Some(crate::ID)
        },
    )?;

    Ok(())
}
