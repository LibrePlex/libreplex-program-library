use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub const NAME_LIMIT: usize = 400;
pub const SYMBOL_LIMIT: usize = 100;
pub const OFFCHAIN_URL_LIMIT: usize = 1200;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum DeploymentStatus {
    Initialised,
    MintedOut,
}

#[account]
#[derive(InitSpace)]
pub struct EditionsDeployment {
    pub creator: Pubkey,
    // set to 0 for unlimited
    pub max_number_of_tokens: u64,
    pub number_of_tokens_issued: u64,

    // set to system account for no cosign
    pub cosigner_program_id: Pubkey,

    pub group_mint: Pubkey,

    pub group: Pubkey,

    #[max_len(SYMBOL_LIMIT)]
    pub symbol: String,

    #[max_len(NAME_LIMIT)]
    pub name: String,

    #[max_len(OFFCHAIN_URL_LIMIT)]
    pub offchain_url: String, // pub padding: Vec<u8, EXCESS>
   
    pub name_is_template: bool,

    pub url_is_template: bool,
    
    pub padding: [u8; 98]

}


// slightly more extended 
#[account]
pub struct HashlistMarker {
    pub editions_deployment: Pubkey,
    pub mint: Pubkey
}

impl HashlistMarker{
    pub const SIZE: usize = 8 + 32 + 32;
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
