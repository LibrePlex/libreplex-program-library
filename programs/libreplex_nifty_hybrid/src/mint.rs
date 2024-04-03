use anchor_lang::prelude::*;
use nifty_asset::instructions::CreateInstructionArgs;
use solana_program::program::invoke;

use crate::NiftyHybrid;
use libreplex_fair_launch::{program::LibreplexFairLaunch,
    DeploymentRaw, MintInput};
use libreplex_fair_launch::cpi::accounts::JoinRawCtx;


#[derive(Accounts)]
pub struct MintCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = deployment)]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, DeploymentRaw>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,
}

pub fn mint_handler<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;
    let deployment = &ctx.accounts.deployment;
    let nifty_hybrid = &mut ctx.accounts.nifty_hybrid;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let receiver = &ctx.accounts.receiver;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    invoke(
        &nifty_asset::instructions::Create {
            asset: non_fungible_mint.key(),
            authority: (deployment.key(),false),
            owner: receiver.key(),
            group: None,
            payer: Some(payer.key()),
            system_program: Some(system_program.key()),
        }
        .instruction(CreateInstructionArgs {
            name: deployment.ticker.clone(),
            standard: nifty_asset::types::Standard::NonFungible,
            mutable: true,
        }),
        &[non_fungible_mint.to_account_info(),
        deployment.to_account_info(),
        receiver.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info()]
    )?;

    let seeds = &[
        b"nifty_hybrid",
        nifty_hybrid.seed.as_ref(),
        &[nifty_hybrid.bump],
    ];

    libreplex_fair_launch::cpi::joinraw(
        CpiContext::new_with_signer(
            fair_launch.to_account_info(),
            JoinRawCtx {
                deployment: ctx.accounts.deployment.to_account_info(),
                hashlist: ctx.accounts.hashlist.to_account_info(),
                hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: nifty_hybrid.to_account_info(),
                non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        )
        .with_remaining_accounts(vec![]),
        MintInput {
            multiplier_denominator: 1,
            multiplier_numerator: 1,
        },
    )?;
    Ok(())
}
