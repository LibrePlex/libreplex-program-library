use anchor_lang::{prelude::*, system_program};
use libreplex_shared::SharedError;



use crate::errors::FairLaunchError;
use crate::{add_to_hashlist, DeploymentV2, MintInput};
use crate::
    HashlistMarker
;

/* 
    Raw join does the following:
    
    1) Add mint ID (of any type: Token-2022, Nifty, Legacy) to the hashlist
    2) Update deployment counters

    Specifically, it does not create any mints itselfx
*/
#[derive(Accounts)]
pub struct JoinRawCtx<'info> {
    #[account(mut,
       seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, DeploymentV2>,

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
    #[account()]
    pub signer: Signer<'info>,

    // The wrapper program is responsible for producing assets
    // that make some sense
    /// CHECK: Can be anything.
    #[account()]
    pub non_fungible_mint: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn joinraw_handler<'info>(ctx: Context<'_, '_, '_, 'info, JoinRawCtx<'info>>, input: MintInput) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let payer = &ctx.accounts.payer; 
    let signer = &ctx.accounts.signer;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let system_program = &ctx.accounts.system_program;
    
    let hashlist = &mut ctx.accounts.hashlist;
    let hashlist_marker = &mut ctx.accounts.hashlist_marker;
    if !deployment.deployed {
        panic!("Not deployed. Cannot join/mint");
    }
    add_to_hashlist(
        (deployment.number_of_tokens_issued + 1) as u32,
        hashlist,
        payer,
        system_program,
        &non_fungible_mint.key(),
        &deployment.key(),
        0,
    )?;

    msg!("Add to hashlist finished");
    
    hashlist_marker.multiplier_denominator = input.multiplier_denominator;
    hashlist_marker.multiplier_numerator = input.multiplier_numerator;


    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        return Err(FairLaunchError::MintedOut.into());
    }

    if deployment.cosigner_mint != system_program::ID && !signer.key().eq(&deployment.cosigner_mint.key()) {
        return Err(SharedError::InvalidCreatorCosigner.into());
    }
    
    deployment.number_of_tokens_issued += 1;
    Ok(())
}