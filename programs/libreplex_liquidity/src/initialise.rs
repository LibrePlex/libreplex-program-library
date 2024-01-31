use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::Liquidity;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    seed: Pubkey,

    pub deployment: Pubkey,
    pub fungible_mint: Pubkey,
    pub bootstrap_start_time: Option<i64>,
    pub bootstrap_requires_sold_out: bool,
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct Initialise<'info> {

    /// CHECK: CAn be anyone
    pub authority: UncheckedAccount<'info>,

    pub fungible_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::authority = liquidity,
        associated_token::mint = fungible_mint)]
    pub fungible_escrow: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = anchor_spl::token::spl_token::native_mint::id())]
    pub wrapped_sol_mint: Box<Account<'info, Mint>>,

    #[account(init, payer = payer, 
        associated_token::mint = wrapped_sol_mint, 
        associated_token::authority = liquidity)]
    pub wrapped_sol_escrow: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + Liquidity::INIT_SPACE,
         seeds = [b"liquidity", input.seed.as_ref()], bump)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>
}

pub fn init_handler(ctx: Context<Initialise>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        fungible_mint,
    } = input;

    ctx.accounts.liquidity.set_inner(Liquidity {
        pool_bootstrapped: false,
        seed,
        bump: ctx.bumps.liquidity,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        fungible_mint,
        authority: ctx.accounts.authority.key(),
        padding: [0; 100],
    });

    Ok(())
}
