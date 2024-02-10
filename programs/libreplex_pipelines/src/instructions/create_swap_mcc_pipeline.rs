

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::spl_token, token_interface::{Mint,TokenAccount, spl_token_2022::{self}}};
use libreplex_fair_launch::{deploy_hybrid::sysvar_instructions_program, DeploymentConfig, Hashlist};
use libreplex_fair_launch::Deployment;
use libreplex_liquidity::{cpi::accounts::MintCtx, Liquidity};
use libreplex_shared::operations::transfer_generic_spl;
use mpl_token_metadata::accounts::Metadata;


use crate::{MccPipeline, SwapMarker};



/// this is where the magic happens.
/// 1) mint an item from the pipeline fair launch (this requires a creator cosign)
/// 2) put the acquired mint immediately into the swap so that mint_incoming can always be swapped for those

#[derive(Accounts)]
pub struct CreateSwapCtx<'info> {

    #[account(mut)]
    pub pipeline: Account<'info, MccPipeline>,

    #[account(init,
        payer = payer, 
        space = SwapMarker::SIZE,
        seeds = [
            "swap_marker".as_bytes(), 
            pipeline.key().as_ref(),
            non_fungible_mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub swap_marker: Account<'info, SwapMarker>,


    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
    constraint = pipeline.fair_launch_deployment == deployment.key())]
    pub deployment: Account<'info, Deployment>,


     /// CHECK: Checked via CPI
    
    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], 
        seeds::program = libreplex_fair_launch::ID,
        bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

    #[account(mut, seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        seeds::program = libreplex_fair_launch::ID,
        bump)]
    pub hashlist: Account<'info, Hashlist>,

    /// CHECK: Passed in via CPI
    #[account(mut,)]
    pub hashlist_marker: UncheckedAccount<'info>,
    
    /// CHECK: Checked in constraint against deployment_config
    #[account(mut,
        constraint = creator_fee_treasury.key() == deployment_config.creator_fee_treasury)]
    pub creator_fee_treasury: UncheckedAccount<'info>,


    // each mint has to exist. for now we restrict incoming mints to NFTS
    // to make sure that each of these marker pairs can only be hit once
    // unless the swap is reversed and then called again
    #[account(mut,
        constraint = non_fungible_mint_incoming.decimals == 0 && non_fungible_mint_incoming.supply == 1
    )] 
    pub non_fungible_mint_incoming: InterfaceAccount<'info, Mint>,

    /// CHECK: PDA derivation checked
    #[account(mut,
        seeds=[
                "metadata".as_bytes(),
                &mpl_token_metadata::ID.as_ref(),
                non_fungible_mint_incoming.key().as_ref(),
            ],
            bump,
        seeds::program=mpl_token_metadata::ID,
    )] 
    pub non_fungible_metadata_incoming: UncheckedAccount<'info>,

    // this is the fungible mint of the pipeline fair launch as well as the outgoing
    // mint of the wap
    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: InterfaceAccount<'info, Mint>, 

    // ... into this escrow account
    #[account(init,
        payer = payer,
        associated_token::mint = fungible_mint,
        associated_token::authority = pipeline // and deposited into the swap
    )]
    pub mint_outgoing_token_account_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for integrations
    #[account(mut)]
    pub signer: Signer<'info>,

    // swapper signer always has the same PDA derivation
    // it tells the multiswap that the call originated 
    // with a certain swapper program and that it's
    // ok to generate the marker
    #[account(mut,
        seeds = ["swapper_signer".as_bytes()],
        seeds::program = swapper_program.key(),
        bump,
    )]
    pub swapper_signer: Signer<'info>,


    #[account(mut,
        associated_token::mint = deployment.fungible_mint,
        associated_token::authority = deployment.key()
    )]
    pub non_fungible_fair_launch_escrow_target_token_account: InterfaceAccount<'info, TokenAccount>,


    /*

        LIQUIDITY-SPECIFIC ACCOUNTS

    */
   
    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_hashlist_marker: UncheckedAccount<'info>,
    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
        constraint = pipeline.liquidity == liquidity.key())]
    pub liquidity: Account<'info, Liquidity>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

     /// CHECK: Checked in cpi.
    #[account(mut)]
    pub liquidity_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    pub pooled_non_fungible_mint: UncheckedAccount<'info>,


    /// CHECK: Checked in cpi.
    pub pooled_non_fungible_token_account:UncheckedAccount<'info>, 



    /// program accounts
    #[account(
        constraint = libreplex_fair_launch_program.key() == libreplex_fair_launch::ID
    )]
    pub libreplex_fair_launch_program: UncheckedAccount<'info>,

    #[account(
        constraint = token_program.key() == spl_token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account(
        constraint = token_program_22.key() == spl_token_2022::ID
    )]
    pub token_program_22: UncheckedAccount<'info>,


    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Can we anything - see swapper_signer derivation above
    #[account(mut)]
    pub swapper_program: UncheckedAccount<'info>,    

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    pub sysvar_instructions: UncheckedAccount<'info>,

}

pub fn create_swap(ctx: Context<CreateSwapCtx>) -> Result<()> {
    
    let swap_marker = &mut ctx.accounts.swap_marker;
    let pipeline = &mut ctx.accounts.pipeline;
    let metadata = &ctx.accounts.non_fungible_metadata_incoming;

    if !metadata.to_account_info().data_is_empty() {
        // we may have a pNFT
        
        let metadata_obj = Metadata::try_from(&metadata.to_account_info())?;
        if let Some(x) = metadata_obj.collection {
            if !x.key.eq(&pipeline.collection) {
                panic!("Invalid collection"); 
            }
        } else {
            panic!("No collection set");
        }
    } else {
        panic!("Metadata is empty");
    }




    swap_marker.incoming_mint = ctx.accounts.non_fungible_mint.key();
    swap_marker.pipeline = pipeline.key();
    
    // transfer the outgoing mint into escrow - 
    let token_program = &ctx.accounts.token_program;
    let mint_outgoing_token_account_escrow = &ctx.accounts.mint_outgoing_token_account_escrow;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let libreplex_fair_launch_program = &ctx.accounts.libreplex_fair_launch_program;
    let deployment = &ctx.accounts.deployment;
    let deployment_config = &ctx.accounts.deployment_config;
    let pipeline = &ctx.accounts.pipeline;
    let creator_fee_treasury = &ctx.accounts.creator_fee_treasury;
    let hashlist = &ctx.accounts.hashlist;
    let hashlist_marker = &ctx.accounts.hashlist_marker;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_token_account = &ctx.accounts.non_fungible_fair_launch_escrow_target_token_account;
    let token_program_22 = &ctx.accounts.token_program_22;
    let sysvar_instructions_program = &ctx.accounts.sysvar_instructions;
    /*
    
        LIQUIDITY-SPECIFIC ACCOUNTS
    
     */
    let liquidity = &ctx.accounts.liquidity;
    let deployment_fungible_token_account = &ctx.accounts.deployment_fungible_token_account;
    let deployment_non_fungible_token_account = &ctx.accounts.deployment_non_fungible_token_account;
    let pooled_hashlist_marker = &ctx.accounts.pooled_hashlist_marker;
    let liquidity_fungible_token_account = &ctx.accounts.liquidity_fungible_token_account;
    let pooled_non_fungible_mint = &ctx.accounts.pooled_non_fungible_mint;
    let pooled_non_fungible_token_account = &ctx.accounts.pooled_non_fungible_token_account;


    let payer = &ctx.accounts.payer;

    let pipeline_seeds: &[&[u8]] = &[
        "pipeline".as_bytes(),
        deployment.ticker.as_ref(),
        &[pipeline.bump],
    ];


    // mint one
    libreplex_liquidity::cpi::mint(
        CpiContext::new_with_signer(
            libreplex_fair_launch_program.to_account_info(),
            MintCtx {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                authority: pipeline.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                deployment: deployment.to_account_info(),
                deployment_config: deployment_config.to_account_info(),
                creator_fee_treasury: creator_fee_treasury.to_account_info(),
                hashlist: hashlist.to_account_info(),
                hashlist_marker: hashlist_marker.to_account_info(),
                
                fungible_mint: fungible_mint.to_account_info(),
                non_fungible_mint: non_fungible_mint.to_account_info(),
                non_fungible_token_account: non_fungible_token_account.to_account_info(),
                // passing dummy accounts to these as the pipelines do not use inscriptions
                // would be good to get a version of mint_2022 that ignores inscriptions
                // so as to clean up the interfaces
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                receiver: pipeline.to_account_info(),
                treasury: creator_fee_treasury.to_account_info(),
                liquidity: liquidity.to_account_info(),
                deployment_fungible_token_account: deployment_fungible_token_account.to_account_info(),
                deployment_non_fungible_token_account: deployment_non_fungible_token_account.to_account_info(),
                pooled_hashlist_market: pooled_hashlist_marker.to_account_info(),
                liquidity_fungible_token_account: liquidity_fungible_token_account.to_account_info(),
                pooled_non_fungible_mint: pooled_non_fungible_mint.to_account_info(),
                pooled_non_fungible_token_account: pooled_non_fungible_token_account.to_account_info(),
                token_program_22: token_program_22.to_account_info(),
                fair_launch: libreplex_fair_launch_program.to_account_info(),
                sysvar_instructions: sysvar_instructions_program.to_account_info(),
            },
            &[pipeline_seeds]
        )
    )?;

    // transfer the new mint into the escrow so it's available for swapping


    transfer_generic_spl(
            &token_program.to_account_info(),
            &non_fungible_token_account.to_account_info(),
            &mint_outgoing_token_account_escrow.to_account_info(),
            &payer.to_account_info(),
            &non_fungible_mint.to_account_info(),
            // pipeline owns all the escrow items to start with
            &pipeline.to_account_info(),
            &associated_token_program.to_account_info(),
            &system_program.to_account_info(),
            None, // payer signs
            &payer.to_account_info(),
            0, // a non-fungible has no decimals 
            1, // and there is only one of these
    )?;

    // libreplex_fair_launch::cpi::swap_to_fungible22(
    //     CpiContext::new_with_signer(
    //         libreplex_fair_launch_program.to_account_info(),
    //         SwapToFungible2022Ctx {
    //             /* the inscription root is set to metaplex
    //              inscription object.
    //             */
    //             signer: pipeline.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //             payer: payer.to_account_info(),
    //             deployment: deployment.to_account_info(),
    //             hashlist_marker: hashlist_marker.to_account_info(),
                
    //             fungible_mint: fungible_mint.to_account_info(),
    //             non_fungible_mint: non_fungible_mint.to_account_info(),
    //             // passing dummy accounts to these as the pipelines do not use inscriptions
    //             // would be good to get a version of mint_2022 that ignores inscriptions
    //             // so as to clean up the interfaces
    //             token_program: token_program.to_account_info(),
    //             associated_token_program: associated_token_program.to_account_info(),
    //             fungible_source_token_account: fungible_fair_launch_escrow_token_account.to_account_info(),
    //             fungible_target_token_account: fungible_fair_launch_escrow_token_account.to_account_info(),
    //             fungible_target_token_account_owner: pipeline.to_account_info(),
    //             non_fungible_source_token_account: fungible_fair_launch_escrow_token_account.to_account_info(),
    //             non_fungible_source_account_owner: pipeline.to_account_info(),
    //             non_fungible_target_token_account: fungible_fair_launch_escrow_token_account.to_account_info(),
    //             token_program_22: fungible_fair_launch_escrow_token_account.to_account_info(),
    //             sysvar_instructions: fungible_fair_launch_escrow_token_account.to_account_info(),
    //         },
    //         &[pipeline_seeds]
    //     )
    // )?;


    // transfer_generic_spl(
    //     &token_program.to_account_info(),
    //     &non_fungible_token_account.to_account_info(),
    //     &mint_outgoing_token_account_escrow.to_account_info(),
    //     &payer.to_account_info(),
    //     &non_fungible_mint.to_account_info(),
    //     // swap marker outgoing owns this to start with.
    //     // when swapping, this ATA will be emptied
    //     // and a new mint will come in
    //     &swap_marker.to_account_info(),
    //     &associated_token_program.to_account_info(),
    //     &system_program.to_account_info(),
    //     None, // payer signs
    //     &payer.to_account_info(),
    //     0, // a non-fungible has no decimals 
    //     1, // and there is only one of these
    // )?;
    

    Ok(())
}
