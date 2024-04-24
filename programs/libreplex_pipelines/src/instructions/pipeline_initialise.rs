use anchor_lang::{prelude::*, system_program};
use libreplex_fair_launch::MultiplierLimits;
use libreplex_fair_launch::{cpi::accounts::InitialiseV3Ctx, InitialiseInputV3, HYBRID_DEPLOYMENT_TYPE};
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
    pub filter: Filter,
    pub lp_ratio: u16,
    pub pool_fee_basis_points: u64,
    pub liquidity_seed: Pubkey,
    pub liquidity_provider_amount_in_lamports: u64,
    pub liquidity_provider_amount_in_spl: u64,
    pub hashlist_url: String,
    pub require_cosigner: bool,
    pub transfer_fee_withdraw_authority: Option<Pubkey>
   
    // this allows for interesting dynamics
}




#[derive(Accounts)]
#[instruction(input: InitialisePipeline)]
pub struct InitialisePipelineCtx<'info> {
    #[account(init,
        space = 8 + Pipeline::INIT_SPACE,
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
        constraint = libreplex_liquidity_program.key().eq(&libreplex_liquidity::ID)
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
    let deployment = &ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let pipeline = &mut ctx.accounts.pipeline;
    let liquidity = &ctx.accounts.liquidity;
    let libreplex_liquidity_program = &ctx.accounts.libreplex_liquidity_program;

    let fair_launch_input = input.clone();

    msg!("{} {} {}",input.limit_per_mint, input.lp_ratio, input.liquidity_provider_amount_in_spl );
    let limit_per_mint_with_decimals = input.limit_per_mint.checked_mul(10_u64.checked_pow(input.decimals as u32).unwrap()).unwrap(); 
    let spl_swap_amount_secondary = limit_per_mint_with_decimals;

    let spl_swap_amount_primary = limit_per_mint_with_decimals.checked_sub(
        limit_per_mint_with_decimals.checked_div(input.lp_ratio as u64).unwrap()).unwrap().checked_sub(input.liquidity_provider_amount_in_spl).unwrap();
    
    // we add a creator program - this means mints and swaps can only happen with
    // the signature of the pipelines program
    let clock = Clock::get()?;
    pipeline.set_inner(Pipeline{ 
        fair_launch_deployment: deployment.key(), 
        liquidity: ctx.accounts.liquidity.key(), 
        auth: ctx.accounts.auth.key(), 
        processed_item_count: 0, 
        creation_time: clock.unix_timestamp, 
        bump: ctx.bumps.pipeline, 
        filter: input.filter, 
        liquidity_provider_amount_in_spl: input.liquidity_provider_amount_in_spl, 
        fungible_chunk_count: 0, 
        fungible_amount_net: 0, 
        fungible_amount_total: 0, 
        created_swap_count: 0, 
        require_cosigner: input.require_cosigner,
        // maybe the most important calc here: N - N/r - lp_provider_amount where r = lp_ratio 
        spl_swap_amount_primary,
        spl_swap_amount_secondary, 
        hashlist_url: input.hashlist_url,
        auth_program_id: system_program::ID });

   
    if fair_launch_input.liquidity_provider_amount_in_lamports == 0 {
        panic!("Liquidity fee cannot be 0 for pipelines")
    }

    libreplex_fair_launch::cpi::initialise_v3(
        CpiContext::new(
            libreplex_fair_launch_program.to_account_info(),
            InitialiseV3Ctx {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                deployment: deployment.to_account_info(),
                deployment_config: deployment_config.to_account_info(),
                // important - FAIR LAUNCH creator must be the liquidity account
                // as the mint is called via liquidity 
                creator: liquidity.to_account_info(),
            },
        ),
        // this has the cosigner specified
        InitialiseInputV3 {
            limit_per_mint: input.limit_per_mint,
            max_number_of_tokens: input.max_number_of_tokens,
            decimals: input.decimals,
            ticker: input.ticker,
            multiplier_limits: MultiplierLimits{
                max_numerator: 1,
                min_denominator: 1,
            },
            deployment_template: "".to_owned(),
            mint_template: "".to_owned(),
            offchain_url: fair_launch_input.offchain_url,
            
            // cosigner is the liquidity here, NOT pipeline
            creator_cosign_program_id: Some(libreplex_liquidity_program.key()),
            use_inscriptions: false,
            deployment_type: HYBRID_DEPLOYMENT_TYPE,
            // and treasury is always liquidity
            creator_fee_treasury: liquidity.key(),
            creator_fee_per_mint_in_lamports: fair_launch_input.liquidity_provider_amount_in_lamports,
            transfer_fee_config: None
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
            // all fees go to the pool
            creator_basis_points: 0,
            lp_ratio: input.lp_ratio,
            pool_fee_basis_points: input.pool_fee_basis_points,
            cosigner_program_id: Some(crate::ID)
        },
    )?;

    Ok(())
}
