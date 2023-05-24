use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{Metadata, NFT, MetadataNft, Collection};


#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    
    /// collection authority or delegate
    #[account(mut)]
    pub collection_authority: Signer<'info>,

    #[account(mut)]
    pub metadata_nft: Box<Account<'info, MetadataNft>>,

    #[account()]
    pub collection: Account<'info, Collection>,

    #[account()]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}

