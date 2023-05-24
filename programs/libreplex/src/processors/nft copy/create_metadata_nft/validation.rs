use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{Metadata, NFT, MetadataNft};


#[derive(Accounts)]
pub struct CreateMetadataNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        constraint = metadata.authority == authority.key(),
        seeds =[
            NFT.as_ref(),
            metadata.key().as_ref(),
        ], 
        bump)]
    pub metadata_nft: Box<Account<'info, MetadataNft>>,

    #[account()]
    pub metadata: Account<'info, Metadata>,

    #[account()]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}

