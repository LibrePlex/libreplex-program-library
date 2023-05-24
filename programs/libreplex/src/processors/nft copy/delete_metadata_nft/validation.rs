use anchor_lang::prelude::*;

use crate::{Metadata, MetadataNft, NFT};


#[derive(Accounts)]
pub struct DeleteMetadataNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        close = authority,
        constraint = metadata.authority == authority.key(),
        constraint = metadata.is_mutable,
        seeds =[
            NFT.as_ref(),
            metadata.key().as_ref(),
        ], 
        bump)]
    pub metadata_nft: Box<Account<'info, MetadataNft>>,

    #[account()]
    pub metadata: Account<'info, Metadata>,

    pub system_program: Program<'info, System>,
    
}


