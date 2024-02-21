use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub const TICKER_LIMIT: usize = 200;
pub const TEMPLATE_LIMIT: usize = 1200;
pub const OFFCHAIN_URL_LIMIT: usize = 1200;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum DeploymentStatus {
    Initialised,
    Deployed,
    MintedOut,
}

#[account]
#[derive(InitSpace)]
pub struct Deployment {
    // creator has two purposes: 
    // 1) to deploy: deployment must be performed by the same wallet that initialises
    // the launch
    // 2) (optionally) to allow for a third party
    // service to add their own logic on top of fair 
    // launch by co-signing
    
    pub creator: Pubkey,

    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,

    pub number_of_tokens_issued: u64,
    pub decimals: u8,


    pub use_inscriptions: bool,
    pub deployment_type: u8,
    // to allow modular custom logic around this contract
    pub require_creator_cosign: bool,

    // indicates whether this deployment was migrated from legacy validator
    // true - from legacy
    // false - created directly via libreplex fair launch
    pub migrated_from_legacy: bool,

    // this is used to sanity check that
    // whenever swaps occur, to the maount
    // of fungible and non-fungible in the
    // escrow always remains equal to the total
    // supply.
    pub escrow_non_fungible_count: u64,

    #[max_len(TICKER_LIMIT)]
    pub ticker: String,

    #[max_len(TEMPLATE_LIMIT)]
    pub deployment_template: String,

    #[max_len(TEMPLATE_LIMIT)]
    pub mint_template: String,

    pub fungible_mint: Pubkey, // starts as 111111111111...

    #[max_len(OFFCHAIN_URL_LIMIT)]
    pub offchain_url: String, // pub padding: Vec<u8, EXCESS>
}

impl Deployment {
    pub fn do_thing() -> u64 {
        0
    }
}


#[account]
pub struct DeploymentConfig {
    pub deployment: Pubkey,
    // defined by creator. this is NOT a libreplex fee as libreplex charges no fees.
    pub creator_fee_treasury: Pubkey,
    pub creator_fee_per_mint_lamports: u64,
    pub deflation_rate_per_swap: u16, // in basis points
    // makes it easier to identify the program / endpoints that need to be called
    pub cosigner_program_id: Pubkey

}

impl DeploymentConfig {
    /// leave a bit of extra space at the end in case more config is needed
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 500;
}


#[event]
pub struct NewDeploymentEvent {
    pub ticker: String,
    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
    pub creator: Pubkey,
}

#[event]
pub struct NewDeploymentV2 {
    pub ticker: String,
    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
    pub creator: Pubkey,
    pub off_chain_url: String,
    pub require_co_sign: bool,
}

#[event]
pub struct DeploymentActive {
    pub fungible_mint: Pubkey,
}

#[event]
pub struct MintEvent {
    pub mint: Pubkey,
    pub ticker: String,
    pub tokens_minted: u64,
    pub max_number_of_tokens: u64,
}

impl Deployment {
    pub fn get_fungible_mint_amount(&self) -> u64 {
        self.limit_per_mint
            .checked_mul(10_u64.checked_pow(self.decimals as u32).unwrap())
            .unwrap()
    }

    pub fn get_max_fungible_mint_amount(&self) -> u64 {
        self.max_number_of_tokens
            .checked_mul(self.limit_per_mint)
            .unwrap()
            .checked_mul(10_u64.checked_pow(self.decimals as u32).unwrap())
            .unwrap()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintAndOrder {
    pub mint: Pubkey,
    pub order: u64,
}

// this is a genuine hashlist for the launch
#[account]
pub struct Hashlist {
    pub deployment: Pubkey,
    pub issues: Vec<MintAndOrder>,
}

// Each mint can only be migrated once
#[account]
pub struct MigrationMarker {}

// Tells you whether a mint belongs to a hashlist
#[account]
pub struct HashlistMarker {}

#[account]
pub struct MigrationCounter {
    pub deployment: Pubkey,
    pub migration_count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Redeemable {
    pub asset: Pubkey,
    pub deployment: Pubkey,
}
