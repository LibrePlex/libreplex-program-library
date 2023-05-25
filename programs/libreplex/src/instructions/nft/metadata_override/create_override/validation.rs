


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{METADATA, Metadata, Collection, COLLECTION, OVERRIDE, MetadataNftOverride};


#[derive(Accounts)]
pub struct CreateOverride<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        constraint = metadata_nft.collection == collection.key(),
        constraint = metadata_nft.is_mutable,
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub metadata_nft: Box<Account<'info, Metadata>>,


    #[account(mut,
        constraint = collection.authority == authority.key(),
        seeds =[
            COLLECTION.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub collection: Box<Account<'info, Collection>>,


    #[account(init,
        payer = authority,
        space = MetadataNftOverride::BASE_SIZE,
        constraint = metadata_override.metadata_nft.key() == metadata_nft.key(),
        seeds =[
            OVERRIDE.as_ref(),
            metadata_nft.key().as_ref(),
        ], 
        bump)]

    pub metadata_override: Box<Account<'info, MetadataNftOverride>>,


    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


