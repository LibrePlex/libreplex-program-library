use std::cmp::{self};

use crate::Inscription;
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Change {
    Reduce {
        amount: u32
    },
    Increase {
        amount: u32
    }
}


/*
  inscription size changes are limited by the maximum increase per call.
  hence, inscription sizes need to be chunked and this means for large 
  increases you require multiple calls. 

  We have two different events as it helps keep the client logic tidy. 
  Typically you would only be interested in InscriptionResizeFinalEvent although
  you could use the interim InscriptionResizeEvent for progress monitoring etc.
*/ 

// fired whenever inscription size is changed, whether it hits the target or not.
#[event]
pub struct InscriptionResizeEvent {
    id: Pubkey,
    size: u32
}


// fired when inscription size hits the target (i.e. this is the final increase / decrease)
#[event]
pub struct InscriptionResizeFinal {
    id: Pubkey,
    size: u32
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ResizeInscriptionInput {
    pub change: Change,
    /* 
        This only exists to show solana
        that each of the resize inputs is 
        in fact a separate transaction
    */
    pub expected_start_size: u32, 
    /*
        target size is specified
        to make sure that multiple resizes
        executed concurrently never increase / decrease
        the size beyond target size
    */
    pub target_size: u32,
}

#[derive(Accounts)]
#[instruction(update_input: ResizeInscriptionInput)]
pub struct ResizeInscription<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut,
        constraint = inscription.authority == authority.key(),

        realloc = Inscription::BASE_SIZE + match update_input.change {
                Change::Increase {amount} => cmp::min(inscription.size + amount, update_input.target_size) as usize,
                Change::Reduce {amount} => cmp::max(inscription.size - amount, update_input.target_size) as usize
        
        },
        realloc::payer = authority,
        realloc::zero = false)]
    pub inscription: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ResizeInscription>,
inscription_input: ResizeInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    inscription.size = match inscription_input.change {
        Change::Increase {amount} => cmp::min(inscription.size + amount, inscription_input.target_size),
        Change::Reduce {amount} => cmp::max(inscription.size - amount, inscription_input.target_size) ,
    };
    
  
    if inscription.size == inscription_input.target_size {
        emit!(InscriptionResizeFinal {
            id: inscription.key(),
            size: inscription_input.target_size,
        });
    } else {
        emit!(InscriptionResizeEvent {
            id: inscription.key(),
            size: inscription.size,
        });
    }

    Ok(())
}