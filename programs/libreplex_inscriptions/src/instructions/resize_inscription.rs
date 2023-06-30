use std::cmp::{Ordering, self};

use crate::{Inscription, InscriptionEvent, InscriptionEventType};
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

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Resize
    });

    Ok(())
}