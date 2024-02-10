use anchor_lang::prelude::*;
use libreplex_fair_launch::{cpi::accounts::InitialiseV2Ctx, InitialiseInputV2, HYBRID_DEPLOYMENT_TYPE};
use libreplex_liquidity::{cpi::accounts::Initialise, InitialiseInput};

use crate::MccPipeline;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseMetaplexPipelineInput {
    pub collection: Pubkey,
    // liquidity specific stuff
    pub lp_ratio: u16,
    pub pool_fee_basis_points: u64,
    pub liquidity_seed: Pubkey

}



#[derive(Accounts)]
#[instruction(fair_launch_input: InitialiseInputV2, input: InitialiseMetaplexPipelineInput)]
pub struct InitialiseMetaplexPipelineCtx<'info> {
    #[account(init,
        space = MccPipeline::SIZE,
        payer = payer,
        // each payer has their own namespace
        seeds=[b"pipeline", payer.key().as_ref(), fair_launch_input.ticker.as_bytes()],
        bump
    )]
    pub pipeline: Account<'info, MccPipeline>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub liquidity: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub deployment: UncheckedAccount<'info>,

    #[account(mut)]
    pub deployment_config: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for more wrapper contracts
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        constraint = libreplex_fair_launch_program.key().eq(&libreplex_fair_launch::ID)
    )]
    pub libreplex_fair_launch_program: UncheckedAccount<'info>,

    #[account(
        constraint = libreplex_fair_launch_program.key().eq(&libreplex_fair_launch::ID)
    )]
    pub libreplex_liquidity_program: UncheckedAccount<'info>,


    #[account()]
    pub system_program: Program<'info, System>,
}




pub fn initialise(
    ctx: Context<InitialiseMetaplexPipelineCtx>,
    fair_launch_input: InitialiseInputV2,
    pipeline_input: InitialiseMetaplexPipelineInput,
) -> Result<()> {
    if fair_launch_input.creator_cosign_program_id.is_some() {
        panic!("creator_cosign_program_id must be None")
    }
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let creator = &ctx.accounts.creator;
    let deployment = &ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let pipeline = &mut ctx.accounts.pipeline;
    let liquidity = &ctx.accounts.liquidity;
    let libreplex_liquidity_program = &ctx.accounts.libreplex_liquidity_program;

    let mut fair_launch_input = fair_launch_input.clone();

    // for now, only allow fair launches with type hybrid until
    // we have token-2022 support in liquidity
    if fair_launch_input.deployment_type != HYBRID_DEPLOYMENT_TYPE {
        panic!("Only hybrid deployment type allowed");
    }

    // we add a creator program - this means mints and swaps can only happen with
    // the signature of the pipelines program
    fair_launch_input.creator_cosign_program_id = Some(crate::ID);
    pipeline.collection = pipeline_input.collection;
    pipeline.fair_launch_deployment = deployment.key();
    pipeline.processed_item_count = 0;
    
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
                creator: creator.to_account_info(),
            },
        ),
        // this has the cosigner specified
        fair_launch_input,
    )?;

    // and initialise liquidity too

    libreplex_liquidity::cpi::initialise(
        CpiContext::new(
            libreplex_liquidity_program.to_account_info(),
            Initialise {
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
            seed: pipeline_input.liquidity_seed,
            deployment: deployment.key(),
            bootstrap_start_time: None, 
            bootstrap_requires_sold_out: true,
            creator_basis_points: 0,
            lp_ratio: pipeline_input.lp_ratio,
            pool_fee_basis_points: pipeline_input.pool_fee_basis_points,
            cosigner_program_id: Some(crate::ID)
        },
    )?;

    Ok(())
}
