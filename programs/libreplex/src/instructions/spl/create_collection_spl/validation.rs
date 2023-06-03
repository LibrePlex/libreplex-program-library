use anchor_lang::prelude::*;

use crate::{Collection, COLLECTION, RoyaltyShare, CREATOR_SIZE};


#[derive(Accounts)]
#[instruction(
    seed: Pubkey,
    creators: Vec<RoyaltyShare>,
)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = Collection::BASE_SIZE + 4 + creators.len() * CREATOR_SIZE,
        seeds =[
            COLLECTION.as_ref(),
            seed.as_ref(),
        ], 
        bump)]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
    
}

