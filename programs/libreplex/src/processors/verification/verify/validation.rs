



use anchor_lang::prelude::*;

use crate::{Verification, VERIFICATION};


#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This can be anything
    pub target: UncheckedAccount<'info>,

    #[account(init,
        payer = signer,
        space = Verification::SIZE,
        seeds =[
            VERIFICATION.as_ref(),
            signer.key().as_ref(),
            target.key().as_ref()
        ], 
        bump)]
    pub verification: Account<'info, Verification>,
    pub system_program: Program<'info, System>,
    
}


