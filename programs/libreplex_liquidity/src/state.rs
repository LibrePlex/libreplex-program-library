use anchor_lang::prelude::*;
use libreplex_fair_launch::{Deployment, HashlistMarker};

pub const DEPLOYMENT_TYPE_NFT: u8 = 0;
pub const DEPLOYMENT_TYPE_SPL: u8 = 1;
pub const DEPLOYMENT_TYPE_NFT_JOIN: u8 = 2;

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

    pub treasury: Pubkey,

    pub lp_ratio: u16,

    pub total_mints: u64,

    pub pool_fee_basis_points: u64,

    pub lookup_table_address: Pubkey,

    // 111111111111..... if not required
    pub cosigner_program_id: Pubkey, 

    pub deployment_type: u8,

    pub required_double_mints: Option<u32>,

    pub padding: [u8; 62]
}

impl Liquidity {
    pub fn amount_to_transfer_to_minter(&self, deployment: &Deployment, marker: &HashlistMarker) -> u64 {
        deployment.get_fungible_mint_amount(marker).checked_mul(
            self.lp_ratio as u64 - 1
        ).unwrap().checked_div(self.lp_ratio as u64).unwrap()
    }
}

pub mod events {
    use super::*;

    #[event]
    pub struct LiquidityCreate {
        pub id: Pubkey,
        pub liquidity: Liquidity,
    }

    #[event]
    pub struct Mint {
        pub liquidity: Pubkey,
        pub total_mints: u64,
    }

    #[event]
    pub struct Bootstrap {
        pub liquidity: Pubkey,
    }
}

