use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct NiftyHybrid {
    pub seed: Pubkey,
    pub bump: u8,
    pub creator: Pubkey,
    pub deployment: Pubkey,
    pub group_mint: Pubkey,
    
    // Single costi
    pub cosigner: Pubkey,

    pub cosigner_program_id: Pubkey, 

    pub padding: [u8; 62]
}


pub mod events {
    use super::*;

    #[event]
    pub struct NiftyHybridCreate {
        pub id: Pubkey,
        pub nifty_hybrid: NiftyHybrid,
    }

    #[event]
    pub struct Mint {
        pub nifty_hybrid: Pubkey,
        pub total_mints: u64,
    }
}

