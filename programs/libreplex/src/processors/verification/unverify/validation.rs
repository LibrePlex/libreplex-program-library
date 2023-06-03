


use anchor_spl::token::{Mint};

use anchor_lang::prelude::*;

use crate::{Metadata, METADATA};


#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,
        seeds =[
            METADATA.as_ref(),
            mint.key().as_ref(),
        ], 
        bump)]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    
}


