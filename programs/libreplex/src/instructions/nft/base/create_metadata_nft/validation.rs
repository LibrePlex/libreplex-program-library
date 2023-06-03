


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{METADATA, Metadata, Collection};


#[derive(Accounts)]
pub struct CreateMetadataNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
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
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub collection: Box<Account<'info, Collection>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


