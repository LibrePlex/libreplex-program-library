use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::Liquidity;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    seed: Pubkey,

    pub deployment: Pubkey,
    pub bootstrap_start_time: Option<i64>,
    pub bootstrap_requires_sold_out: bool,
    pub creator_basis_points: u64,

    pub lp_ratio: u16,
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct Initialise<'info> {

    /// CHECK: CAn be anyone
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + Liquidity::INIT_SPACE,
         seeds = [b"liquidity", input.seed.as_ref()], bump)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,
}

pub fn init_handler(ctx: Context<Initialise>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        creator_basis_points,
        lp_ratio,
    } = input;

    ctx.accounts.liquidity.set_inner(Liquidity {
        pool_bootstrapped: false,
        lp_ratio,
        total_mints: 0,
        seed,
        bump: ctx.bumps.liquidity,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        creator_basis_points,
        authority: ctx.accounts.authority.key(),
        padding: [0; 100],
    });

    Ok(())
}
