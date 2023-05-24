


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{Metadata, Collection, METADATA, COLLECTION};


#[derive(Accounts)]
pub struct DeleteMetadata<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        close = authority,
        constraint = metadata.collection == collection.key(),
        constraint = metadata.is_mutable,
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub metadata: Box<Account<'info, Metadata>>,


    #[account(mut,
        constraint = collection.authority == authority.key(),
        constraint = metadata.is_mutable,
        seeds =[
            COLLECTION.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: Checked in logic. If not empty, block delete of the nft metadata
    #[account()]
    pub metadata_override: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


