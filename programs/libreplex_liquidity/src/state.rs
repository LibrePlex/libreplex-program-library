use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Liquidity {
    pub seed: Pubkey,
    pub bump: u8,
    

    pub bootstrap_start_time: Option<i64>,
    pub bootstrap_requires_sold_out: bool,

    pub pool_bootstrapped: bool,

    pub creator_basis_points: u64,

    pub deployment: Pubkey,
    pub authority: Pubkey,
    pub padding: [u8; 100]
}

