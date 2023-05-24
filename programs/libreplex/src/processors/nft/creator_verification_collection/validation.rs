use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{Metadata, NFT, MetadataNft, Collection};


#[derive(Accounts)]
pub struct VerifyCreator<'info> {
    
    /// collection authority or delegate
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account()]
    pub collection: Account<'info, Collection>,

    #[account()]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}

