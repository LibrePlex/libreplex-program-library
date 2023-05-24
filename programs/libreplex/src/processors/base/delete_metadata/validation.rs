


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{Metadata, METADATA, NFT};


#[derive(Accounts)]
pub struct DeleteMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        close = authority,
        constraint = authority.key() == metadata.authority,
        constraint = metadata.is_mutable,
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub metadata: Box<Account<'info, Metadata>>,

    /// CHECK: Can only delete metadata if this is empty
    #[account(
        seeds =[
            NFT.as_ref(),
            metadata.key().as_ref(),
        ], 
    bump)]
    pub metadata_nft: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


