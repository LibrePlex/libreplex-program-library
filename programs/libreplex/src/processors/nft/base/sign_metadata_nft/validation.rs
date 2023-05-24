


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{ METADATA, Metadata, NFT_OVERRIDE, Collection};


#[derive(Accounts)]
pub struct SignMetadataNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,
        constraint = metadata_nft.collection == collection.key(),
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub metadata_nft: Box<Account<'info, Metadata>>,


    /// CHECK: This is allowed to be empty. Hence deserialized in logic
    #[account(mut,
        constraint = metadata_nft.collection == collection.key(),
        seeds =[
            NFT_OVERRIDE.as_ref(),
            metadata_nft.key().as_ref(),
        ], 
        bump)]
    pub metadata_nft_override: UncheckedAccount<'info>,


    #[account(mut,
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub collection: Box<Account<'info, Collection>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


