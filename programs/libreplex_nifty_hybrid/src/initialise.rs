use anchor_lang::{prelude::*, system_program};

use crate::{events::NiftyHybridCreate, NiftyHybrid};
use libreplex_fair_launch::DeploymentV2;
use nifty_asset::{
    extensions::{ExtensionBuilder, GroupingBuilder},
    instructions::CreateInstructionArgs,
    types::{ExtensionInput, ExtensionType, Standard},
};
use solana_program::program::invoke_signed;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub seed: Pubkey,
    pub cosigner: Option<Pubkey>,
    pub cosigner_program_id: Option<Pubkey>,
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct InitialiseCtx<'info> {
    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, DeploymentV2>,

    /// CHECK: Can be any
    #[account(mut)]
    pub group_mint: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(init, payer = payer, space = 8 + NiftyHybrid::INIT_SPACE,
         seeds = [b"nifty_hybrid", input.seed.as_ref()], bump)]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,

    pub system_program: Program<'info, System>,

    #[account(
        constraint = nifty_program.key().eq(&nifty_asset::ID)
    )]
    pub nifty_program: UncheckedAccount<'info>,
}

pub fn init_handler(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        cosigner,
        cosigner_program_id,
    } = input;

    ctx.accounts.nifty_hybrid.set_inner(NiftyHybrid {
        seed,
        creator: ctx.accounts.creator.key(),
        bump: ctx.bumps.nifty_hybrid,
        deployment: ctx.accounts.deployment.key(),
        group_mint: ctx.accounts.group_mint.key(),
        cosigner: match &cosigner {
            Some(x) => *x,
            _ => system_program::ID,
        },
        cosigner_program_id: match cosigner_program_id {
            Some(x) => x,
            _ => system_program::ID,
        },
        padding: [0; 62],
    });

    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    let group_mint = &ctx.accounts.group_mint;
    let deployment = &ctx.accounts.deployment;
    let nifty_program = &ctx.accounts.nifty_program;
    let nifty_hybrid = &ctx.accounts.nifty_hybrid;

    let seeds = &[
        b"nifty_hybrid",
        input.seed.as_ref(),
        &[ctx.bumps.nifty_hybrid],
    ];

    let mut grouping_extension = GroupingBuilder::default();
    let grouping_data = grouping_extension.data();

    // make the group asset
    invoke_signed(
        &nifty_asset::instructions::Create {
            asset: group_mint.key(),
            group_authority: None,
            authority: (nifty_hybrid.key(), true),
            owner: nifty_hybrid.key(),
            group: None,
            payer: Some(payer.key()),
            system_program: Some(system_program.key()),
        }
        .instruction(CreateInstructionArgs {
            name: deployment.ticker.clone(),
            standard: Standard::NonFungible,
            mutable: true,
            extensions: Some(vec![ExtensionInput {
                extension_type: ExtensionType::Grouping,
                length: grouping_data.len() as u32,
                data: Some(grouping_data),
            }]),
        }),
        &[
            group_mint.to_account_info(),
            nifty_hybrid.to_account_info(),
            payer.to_account_info(),
            nifty_program.to_account_info(),
            system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    // mint the required amount of fungible
    emit_create(&ctx.accounts.nifty_hybrid);

    Ok(())
}

// Avoid blowing up the stack.
fn emit_create(nifty_hybrid: &Account<NiftyHybrid>) {
    let nifty_hybrid_ref: &NiftyHybrid = nifty_hybrid.as_ref();
    emit!(NiftyHybridCreate {
        nifty_hybrid: nifty_hybrid_ref.clone(),
        id: nifty_hybrid.key()
    });
}
