use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::Mint;
use libreplex_monoswap_client::instructions::CreateSwapInstructionArgs;
use nifty_asset::instructions::CreateInstructionArgs;
use solana_program::program::invoke;

use crate::NiftyHybrid;
use libreplex_fair_launch::cpi::accounts::JoinRawCtx;
use libreplex_fair_launch::{program::LibreplexFairLaunch, DeploymentRaw, MintInput};

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

    /// CHECK: Passed into monoswap program
    #[account(mut)]
    pub swap_marker: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program
    #[account(mut)]
    pub swap_marker_aux: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program - is this the same as non_fungible_mint?
    #[account(mut)]
    pub incoming_asset: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program
    #[account(mut)]
    pub incoming_asset_aux: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program  - is this the same as fungible_mint?
    #[account(mut)]
    pub external_asset: UncheckedAccount<'info>,

    #[account(mut)]
    pub incoming_asset_program: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut, 
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn mint_handler<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;
    let deployment = &ctx.accounts.deployment;
    let nifty_hybrid = &mut ctx.accounts.nifty_hybrid;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let receiver = &ctx.accounts.receiver;
    let payer = &ctx.accounts.payer;
    let swap_marker = &ctx.accounts.swap_marker;
    let swap_marker_aux = &ctx.accounts.swap_marker_aux;
    let incoming_asset = &ctx.accounts.incoming_asset;
    let incoming_asset_aux = &ctx.accounts.incoming_asset_aux;
    let external_asset = &ctx.accounts.external_asset;
    let incoming_asset_program = &ctx.accounts.incoming_asset_program;
    let system_program = &ctx.accounts.system_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let fungible_mint = &ctx.accounts.fungible_mint;
   invoke(
        &nifty_asset::instructions::Create {
            asset: non_fungible_mint.key(),
            authority: (deployment.key(), false),
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
        &[
            non_fungible_mint.to_account_info(),
            deployment.to_account_info(),
            receiver.to_account_info(),
            payer.to_account_info(),
            system_program.to_account_info(),
        ],
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

    invoke(
        &libreplex_monoswap_client::instructions::CreateSwap {
            payer: payer.key(),
            // use raw deployment as the namespace - this allows us to narrow things
            // down when filtering on target options
            namespace: deployment.key(),
            authority: deployment.key(),
            swap_marker: swap_marker.key(),
            swap_marker_aux: swap_marker_aux.key(),
            incoming_asset: incoming_asset.key(),
            incoming_asset_aux: Some(incoming_asset_aux.key()),
            external_asset: external_asset.key(),
            incoming_asset_program: incoming_asset_program.key(),
            associated_token_program: associated_token_program.key(),
            system_program: system_program.key(),
        }
        .instruction(CreateSwapInstructionArgs {
            incoming_amount: 1, // one NFT in
            external_amount: deployment.get_base_amount_per_mint(fungible_mint),
        }),
        &[
            payer.to_account_info(),
            deployment.to_account_info(),
            swap_marker.to_account_info(),
            swap_marker_aux.to_account_info(),
            incoming_asset.to_account_info(),
            incoming_asset_aux.to_account_info(),
            external_asset.to_account_info(),
            incoming_asset_program.to_account_info(),
            associated_token_program.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
