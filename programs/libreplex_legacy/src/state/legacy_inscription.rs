use std::fmt;

use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::instructions::AuthorityType;



#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub enum LegacyType {
    MetaplexMint,
    
}

impl fmt::Display for LegacyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LegacyType::MetaplexMint => {
                write!(f, "{:?}", self)
            }
        }
    }
}

#[account]
pub struct LegacyInscription {
    // each mint is allowed a single metaplex inscription
    pub mint: Pubkey,
    pub inscription: Pubkey,
    pub legacy_type: LegacyType,
    pub authority_type: AuthorityType
    
}

impl LegacyInscription {
    pub const SIZE: usize = 8 + 32 + 32 + 2;
}


