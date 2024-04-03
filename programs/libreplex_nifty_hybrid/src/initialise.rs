use anchor_lang::{prelude::*, system_program};

use libreplex_fair_launch::DeploymentRaw;
use nifty_asset::{instructions::CreateInstructionArgs, types::Standard};
use solana_program::program::invoke;
use crate::{events::NiftyHybridCreate, NiftyHybrid};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub seed: Pubkey,
    pub deployment: Pubkey,
    pub cosigner: Option<Pubkey>,
    pub cosigner_program_id: Option<Pubkey>,
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct InitialiseCtx<'info> {

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, DeploymentRaw>,

     /// CHECK: Can be anyone
    pub group_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(init, payer = payer, space = 8 + NiftyHybrid::INIT_SPACE,
         seeds = [b"nifty_hybrid", input.seed.as_ref()], bump)]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,

    pub system_program: Program<'info, System>,
}

pub fn init_handler(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        cosigner,
        deployment,
        cosigner_program_id,
    } = input;

    ctx.accounts.nifty_hybrid.set_inner(NiftyHybrid {
        seed,
        creator: ctx.accounts.creator.key(),
        bump: ctx.bumps.nifty_hybrid,
        deployment,
        cosigner: match &cosigner {
            Some(x)=>*x,
            _ => system_program::ID
        },
        cosigner_program_id: match cosigner_program_id {
            Some(x)=>x,
            _=>system_program::ID
        },
        padding: [0; 62],
    });


    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    let group_mint = &ctx.accounts.group_mint;
    let deployment = &ctx.accounts.deployment;
    // make the group asset
    invoke(
        &nifty_asset::instructions::Create {
            asset: group_mint.key(),
            authority: (deployment.key(), false),
            owner: deployment.key(),
            group: None,
            payer: Some(payer.key()),
            system_program: Some(system_program.key()),
        }
        .instruction(CreateInstructionArgs {
            name: deployment.ticker.clone(),
            standard: Standard::NonFungible,
            mutable: true,
        }),
        &[group_mint.to_account_info(),
        deployment.to_account_info(),
        payer.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info()]
    )?;
    

    emit_create(&ctx.accounts.nifty_hybrid);

    Ok(())
}

// Avoid blowing up the stack.
fn emit_create(nifty_hybrid: &Account<NiftyHybrid>) {
    let nifty_hybrid_ref: &NiftyHybrid = nifty_hybrid.as_ref();
    emit!(NiftyHybridCreate { nifty_hybrid: nifty_hybrid_ref.clone(), id: nifty_hybrid.key()});
}


