

use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token::AssociatedToken, token::spl_token, token_interface::{Mint, spl_token_2022::{self}}};
use libreplex_fair_launch::{Deployment, DeploymentConfig, TOKEN2022_DEPLOYMENT_TYPE};

use libreplex_monoswap::{cpi::accounts::CreateMonoSwapCtx, CreateMonoSwapInput};
use libreplex_shared::sysvar_instructions_program;
use mpl_token_metadata::accounts::Metadata;
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
            "swap_marker".as_bytes(), 
            pipeline.key().as_ref(),
            non_fungible_mint_incoming.key().as_ref()], // always indexed by the incoming mint
        bump,)]
    pub pipeline_swap_marker: Account<'info, PipelineSwapMarker>,


    /// CHECK: Checked via CPI and against the pipeline deployment
    #[account(mut,
    constraint = pipeline.fair_launch_deployment == deployment.key())]
    pub deployment: Box<Account<'info, Deployment>>,




     /// CHECK: Checked via CPI
    
    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], 
        seeds::program = libreplex_fair_launch::ID,
        bump)]
    pub deployment_config: Box<Account<'info, DeploymentConfig>>,

    
    // each mint has to exist. for now we restrict incoming mints to NFTS
    // to make sure that each of these marker pairs can only be hit once
    // unless the swap is reversed and then called again
    #[account(mut,
        constraint = non_fungible_mint_incoming.decimals == 0 && non_fungible_mint_incoming.supply == 1
    )] 
    pub non_fungible_mint_incoming: Box<InterfaceAccount<'info, Mint>>,

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
    // #[account(init,
    //     payer = payer,
    //     associated_token::mint = fungible_mint,
    //     associated_token::authority = pipeline // and deposited into the swap
    // )]
    // pub mint_outgoing_token_account_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // leave this here for integrations
    #[account(mut)]
    pub signer: Signer<'info>,

    // swapper signer always has the same PDA derivation
    // it tells the multiswap that it's
    // ok to generate the marker in this namespace
    pub namespace: Signer<'info>,

      // leave this here for integrations
      #[account(mut,
        constraint = pipeline.auth_program_id.eq(&system_program::ID) || auth.key().eq(&pipeline.auth_program_id) )]
    pub auth: Signer<'info>,

    /// CHECK: Checked via CPI
    pub pipeline_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in transfer logic (must be an ATA owned by the swap)
    pub monoswap_fungible_token_account: UncheckedAccount<'info>,

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
    
    let pipeline_swap_marker = &mut ctx.accounts.pipeline_swap_marker;
    

    pipeline_swap_marker.pipeline = ctx.accounts.pipeline.key();
    pipeline_swap_marker.incoming_mint = ctx.accounts.non_fungible_mint.key();
   
    // transfer the outgoing mint into escrow - 
    let token_program = &ctx.accounts.token_program;
    let token_program_22 = &ctx.accounts.token_program_22;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let deployment = &ctx.accounts.deployment;
    let fungible_mint = &ctx.accounts.fungible_mint;
    
    let pipeline_fungible_token_account = &ctx.accounts.pipeline_fungible_token_account; 
    let libreplex_monoswap_program = &ctx.accounts.libreplex_monoswap_program;
    let non_fungible_mint_incoming = &ctx.accounts.non_fungible_mint_incoming;
    let monoswap_fungible_token_account = &ctx.accounts.monoswap_fungible_token_account;
    let namespace = &ctx.accounts.namespace;
    let libreplex_pipelines_program = &ctx.accounts.libreplex_pipelines_program;
    let pipeline = &mut ctx.accounts.pipeline;
    let non_fungible_metadata_incoming = &ctx.accounts.non_fungible_metadata_incoming;


    match pipeline.filter {
        Filter::MCC {
            collection_id
        } => {
            let metadata_obj = Metadata::try_from(&non_fungible_metadata_incoming.to_account_info())?;
            match metadata_obj.collection {
                Some(x) => {
                    if !x.key.eq(&collection_id) {
                        panic!("Bad collection. Expected {}", {collection_id.to_string()});
                    } else {
                        // this is the happy path
                    }
                },
                _=>{
                    panic!("Metadata has no collection. Expected {}", {collection_id.to_string()});
                }
            }
        },
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

    // calculate the total amount of the swap
    let swap_amount = pipeline.fungible_amount_net.checked_div(pipeline.fungible_chunk_count).unwrap(); 

    // swap all to fungible
    libreplex_monoswap::cpi::create_monoswap(
        CpiContext::new_with_signer(
            libreplex_monoswap_program.to_account_info(),
            CreateMonoSwapCtx {
                /* the inscription root is set to metaplex
                    inscription object.
                */
                mint_outgoing_owner: pipeline.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                // passing dummy accounts to these as the pipelines do not use inscriptions
                // would be good to get a version of mint_2022 that ignores inscriptions
                // so as to clean up the interfaces
                associated_token_program: associated_token_program.to_account_info(),

                // this comes out of the escrow fungible
                token_program: fungible_token_program,
                swap_marker: pipeline_swap_marker.to_account_info(),
                // the NFT from the original collection that can be swapped for SPL
                mint_incoming: non_fungible_mint_incoming.to_account_info(),
                mint_outgoing: fungible_mint.to_account_info(),
                // where the fungible is coming from
                mint_outgoing_token_account_source: pipeline_fungible_token_account.to_account_info(),
                // ... and where it's going to (into the swap)
                mint_outgoing_token_account_escrow: monoswap_fungible_token_account.to_account_info(),
                namespace: namespace.to_account_info(),
                swapper_program: libreplex_pipelines_program.to_account_info(),
            },
            &[pipeline_seeds]
        ),CreateMonoSwapInput {
            // empty out the swapper temporary account
            mint_outgoing_amount: swap_amount
        }
    )?;

    // one less of these, it's gone into the swaps
    pipeline.fungible_chunk_count -= 1;
    pipeline.created_swap_count += 1;
    // somebody else could send spl here as well. 
    // we ignore that and just count what we need
    pipeline.fungible_amount_total -= swap_amount;
    pipeline.fungible_amount_net -= swap_amount;

    Ok(())
}
