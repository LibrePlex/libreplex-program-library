

use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::spl_token, token_interface::{Mint, spl_token_2022::{self}}};
use libreplex_fair_launch::{Deployment, TOKEN2022_DEPLOYMENT_TYPE};

use libreplex_monoswap::{cpi::accounts::CreateMonoSwapCtx, CreateMonoSwapInput};
use libreplex_shared::{operations::transfer_generic_spl, sysvar_instructions_program};
use solana_program::keccak;






/// this is where the magic happens.
/// 1) mint an item from the pipeline fair launch (this requires a creator cosign)
/// 2) swap that mint immediately in the fair launch escrow for X tokens
/// 3) keep Y tokens (for now - for liquidity pool later)
/// 4) put X-Y tokens into the swap so that mint_incoming can always be swapped for those




use crate::{Filter, Pipeline, PipelineSwapMarker};




// Anybody can call this method as long as there is enough SPL available in the pipeline fungible escrow
// if there is not enough, then the fungible escrow must be topped up via add_liquidity 
// When minted out, there will always be enough SPL available to create all the swaps.
// if any amounts are burned, this method will leave some access SPL in the pipeline escrow.
// that's ok too.
#[derive(Accounts)]
pub struct CreateSwapCtx<'info> {

    #[account(mut)]
    pub pipeline: Account<'info, Pipeline>,

    #[account(init,
        payer = payer, 
        space = PipelineSwapMarker::SIZE,
        seeds = [
            b"swap_marker", 
            pipeline.key().as_ref(),
            non_fungible_mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub pipeline_swap_marker: Account<'info, PipelineSwapMarker>,

    /// CHECK: Checked in CPI
    #[account(mut)]
    pub monoswap_swap_marker: UncheckedAccount<'info>,


    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
    constraint = pipeline.fair_launch_deployment == deployment.key())]
    pub deployment: Box<Account<'info, Deployment>>,


    // each mint has to exist. for now we restrict incoming mints to NFTS
    // to make sure that each of these marker pairs can only be hit once
    // unless the swap is reversed and then called again
    #[account(mut,
        constraint = non_fungible_mint_incoming.decimals == 0 && non_fungible_mint_incoming.supply == 1
    )] 
    pub non_fungible_mint_incoming: Box<InterfaceAccount<'info, Mint>>,

       
    // this is the fungible mint of the pipeline fair launch as well as the outgoing
    // mint of the wap
    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: InterfaceAccount<'info, Mint>, 

    #[account(mut)]
    pub payer: Signer<'info>,

     // leave this here for integrations
     #[account(mut,
        constraint = !pipeline.require_cosigner || auth.key().eq(&pipeline.auth) )]
    pub auth: Signer<'info>,


    /// CHECK: Checked via CPI
    #[account(mut)]
    pub payer_nonfungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub payer_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked via CPI
    #[account(mut)]
    pub pipeline_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in transfer logic (must be an ATA owned by the swap)
    #[account(mut)]
    pub monoswap_nonfungible_token_account: UncheckedAccount<'info>,


    // there is one escrow holder per namespace + incoming mint
    // this helps to find stuff by getTokenAccountsByOwner 
    /// CHECK: Checked via CPI
    #[account(mut,
        seeds = ["swap_escrow".as_bytes(), 
            pipeline.key().as_ref(),
            fungible_mint.key().as_ref()],
        seeds::program = libreplex_monoswap::ID,
    bump)]
    pub escrow_holder: UncheckedAccount<'info>,
    /*

        LIQUIDITY-SPECIFIC ACCOUNTS

    */
  
    /// CHECK: ID checked in constraint
    #[account(
        constraint = token_program.key() == spl_token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = token_program_22.key() == spl_token_2022::ID
    )]
    pub token_program_22: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Id checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = libreplex_monoswap_program.key() == libreplex_monoswap::ID
    )]
    pub libreplex_monoswap_program: UncheckedAccount<'info>,

    /// CHECK: ID checked in constraint
    #[account(
        constraint = libreplex_pipelines_program.key() == crate::ID
    )]
    pub libreplex_pipelines_program: UncheckedAccount<'info>

}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum FilterInput {
    Hashlist {
        proof: Vec<[u8; 32]>,
    },
    Other

}

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            computed_hash = keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            computed_hash = keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    computed_hash == root
}  


pub fn create_swap(ctx: Context<CreateSwapCtx>, input: FilterInput) -> Result<()> {
    
    // to make sure we don't create initial swap twice for the same mint.
    let pipeline_swap_marker = &mut ctx.accounts.pipeline_swap_marker;
    

    pipeline_swap_marker.pipeline = ctx.accounts.pipeline.key();
    pipeline_swap_marker.incoming_mint = ctx.accounts.non_fungible_mint_incoming.key();

    let monoswap_swap_marker = &ctx.accounts.monoswap_swap_marker;
   
    // transfer the outgoing mint into escrow - 
    let token_program = &ctx.accounts.token_program;
    let token_program_22 = &ctx.accounts.token_program_22;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let deployment = &ctx.accounts.deployment;
    let fungible_mint = &ctx.accounts.fungible_mint;
    
    let payer_nonfungible_token_account = &ctx.accounts.payer_nonfungible_token_account; 
    let libreplex_monoswap_program = &ctx.accounts.libreplex_monoswap_program;
    let non_fungible_mint_incoming = &ctx.accounts.non_fungible_mint_incoming;
    let monoswap_nonfungible_token_account = &ctx.accounts.monoswap_nonfungible_token_account;
    let pipeline = &mut ctx.accounts.pipeline;
    let pipeline_fungible_token_account = &ctx.accounts.pipeline_fungible_token_account;
    let payer_fungible_token_account = &ctx.accounts.payer_fungible_token_account;
    let escrow_holder = &ctx.accounts.escrow_holder;

    match pipeline.filter {
        
        Filter::Hashlist { root } => {
            // do some validation here to
            let leaf = keccak::hash(&non_fungible_mint_incoming.key().to_bytes()).to_bytes();
            match input {
                FilterInput::Hashlist { proof} => {
                    if !crate::verify(proof, root, leaf) {
                        panic!("Not in hashlist");
                    }
                    // ok validation successful
                },
                _=> {
                    panic!("Incorrect input type for Hashlist")
                }
            }
        },
        _ => {
            panic!("Unexpected filter. Only MCC currently supported");
        }
    }

    let payer = &ctx.accounts.payer;

    let pipeline_seeds: &[&[u8]] = &[
        "pipeline".as_bytes(),
        deployment.ticker.as_ref(),
        &[pipeline.bump],
    ];


    let fungible_token_program = match deployment.deployment_type  {
        TOKEN2022_DEPLOYMENT_TYPE => token_program_22.to_account_info(),
        _ => token_program.to_account_info()
    };


    // 

    // creates a BACKWARDS swap (SPL -> NFT)
    libreplex_monoswap::cpi::create_monoswap(
        CpiContext::new_with_signer(
            libreplex_monoswap_program.to_account_info(),
            CreateMonoSwapCtx {
                /* the inscription root is set to metaplex
                    inscription object.
                */
                mint_outgoing_owner: payer.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                escrow_holder: escrow_holder.to_account_info(),
                // passing dummy accounts to these as the pipelines do not use inscriptions
                // would be good to get a version of mint_2022 that ignores inscriptions
                // so as to clean up the interfaces
                associated_token_program: associated_token_program.to_account_info(),

                // this comes out of the escrow fungible
                token_program: fungible_token_program,
                swap_marker: monoswap_swap_marker.to_account_info(),
                // the SPL comes in 
                mint_incoming: fungible_mint.to_account_info(),
                // and the NFT comes out
                mint_outgoing: non_fungible_mint_incoming.to_account_info(),
                // where the fungible is coming from
                mint_outgoing_token_account_source: payer_nonfungible_token_account.to_account_info(),
                // ... and where it's going to (into the swap)
                mint_outgoing_token_account_escrow: monoswap_nonfungible_token_account.to_account_info(),
                namespace: pipeline.to_account_info(),
            },
            &[pipeline_seeds]
        ),CreateMonoSwapInput {
            // secondary amount is the swap rate - higher than the original swap rate
            mint_incoming_amount: pipeline.spl_swap_amount_secondary,
            mint_outgoing_amount: 1 // just one NFT coming out
        }
    )?;

    let source_token_program = match *fungible_mint.to_account_info().owner {
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("Unexpected fungible mint owner (not keg or t22)")
        }
    };


    // calculate the total amount of the swap
    let swap_amount = pipeline.fungible_amount_net.checked_div(pipeline.fungible_chunk_count).unwrap(); 

    transfer_generic_spl(
        &source_token_program.to_account_info(),
        &pipeline_fungible_token_account.to_account_info(),
        &payer_fungible_token_account.to_account_info(),
        &pipeline.to_account_info(),
        &fungible_mint.to_account_info(),
        &payer.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[pipeline_seeds]),
        payer,
        fungible_mint.decimals,
        swap_amount
    )?;

    // one less of these, it's gone into the swaps
    pipeline.fungible_chunk_count -= 1;
    pipeline.created_swap_count += 1;
    // somebody else could send spl here as well. 
    // we ignore that and just count what we need
    pipeline.fungible_amount_net -= swap_amount;

    Ok(())
}
