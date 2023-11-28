use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::{*};

pub const TICKER_LIMIT: usize = 200;
pub const TEMPLATE_LIMIT: usize = 1200;
pub const OFFCHAIN_URL_LIMIT: usize = 1200;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum DeploymentStatus {
    Initialised,
    Deployed,
    MintedOut
}

#[account]
#[derive(InitSpace)]
pub struct Deployment {
    // creates has two purposes - one for historical record
    // but also to link initialise and deploy endpoints together
    // deployment must be performed by the same wallet that initialises 
    // the launch

    pub creator: Pubkey, 

    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
   
    pub number_of_tokens_issued: u64,
    pub decimals: u8,

    // if a ticker is not deployed within 1 hour of initialisation, it becomes
    // available for deletion and reclaim

    pub deployed: bool,
    pub minted_out: bool, 

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
    pub offchain_url: String

    // pub padding: Vec<u8, EXCESS>
    
}  

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintAndOrder {
    pub mint: Pubkey,
    pub order: u64
}

// this is a genuine hashlist for the launch
#[account]
pub struct Hashlist {
    pub deployment: Pubkey,
    pub issues:Vec<MintAndOrder>
}






