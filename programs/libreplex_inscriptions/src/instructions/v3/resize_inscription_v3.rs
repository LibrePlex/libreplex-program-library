use std::cmp::{self};

use crate::instructions::{ResizeInscriptionInput, InscriptionV3EventUpdate};
use crate::{InscriptionV3, constants, InscriptionV3EventData};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use std::cmp::Ordering;

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
    size: u32,
}

#[derive(Accounts)]
#[instruction(update_input: ResizeInscriptionInput)]
pub struct ResizeInscriptionV3<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut,
        constraint = inscription_v3.authority == authority.key())]
    pub inscription_v3: Account<'info, InscriptionV3>,

    /// CHECK: validated in logic
    #[account(
        mut,
    seeds=[
        "inscription_data".as_bytes(),
        inscription_v3.root.as_ref()
    ],bump)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ResizeInscriptionV3>,
    inscription_input: ResizeInscriptionInput,
) -> Result<()> {
    let inscription_v3 = &mut ctx.accounts.inscription_v3;

    let inscription_data = &mut ctx.accounts.inscription_data;

    let system_program = &mut ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;

    let size_old = inscription_v3.size;
    inscription_v3.size = match inscription_input.change.cmp(&0) {
        Ordering::Greater => cmp::min(
            (inscription_v3.size as i64 + inscription_input.change as i64) as u32,
            inscription_input.target_size,
        ),
        Ordering::Less => cmp::max(
            (inscription_v3.size as i64 + inscription_input.change as i64) as u32,
            inscription_input.target_size,
        ),
        Ordering::Equal => inscription_v3.size,
    };
    let size_new = inscription_v3.size;

    let rent = Rent::get()?;
    let new_minimum_balance = cmp::max(
        constants::MINIMUM_INSCRIPTION_LAMPORTS,
        rent.minimum_balance(size_new as usize));
    msg!("Size before: {} New size {}",size_old, size_new);

    match size_new.cmp(&size_old) {
        Ordering::Greater => {
            let lamports_diff = new_minimum_balance.saturating_sub(inscription_data.lamports());
            msg!("Increasing inscription capacity by {} and debiting {} lamports for rent exemption", size_new-size_old, lamports_diff);
            // transferring lamport diff to inscription_data
            if lamports_diff > 0 {
                invoke(
                    &system_instruction::transfer(&payer.key(), inscription_data.key, lamports_diff),
                    &[
                        payer.to_account_info(),
                        inscription_data.to_account_info(),
                        system_program.to_account_info(),
                    ],
                )?;
            }
            
            inscription_data.realloc(inscription_v3.size as usize, false)?;
        }
        Ordering::Less => {
            let lamports_diff = inscription_data
                .lamports()
                .saturating_sub(new_minimum_balance);
            msg!("Decreasing inscription capacity by {} and crediting {} lamports for rent exemption", size_old-size_new, lamports_diff);
            
            // transferring lamport diff to payer
            inscription_data.sub_lamports(lamports_diff)?;
            payer.add_lamports(lamports_diff)?;

            inscription_data.realloc(inscription_v3.size as usize, false)?;

            // we cannot use realloc macro because inscription_data account discriminator
            // has been overwritten by data.
        
        }
        _ => {

            let lamports_diff = inscription_data
            .lamports()
            .saturating_sub(new_minimum_balance);
            
            msg!("Inscription capacity unchanged. Crediting {} lamports for rent exemption", lamports_diff);
            if lamports_diff > 0 {
                // transferring lamport diff to payer
                inscription_data.sub_lamports(lamports_diff)?;
                payer.add_lamports(lamports_diff)?;
                    // transfer lamport diff to payer if > 0
            }
        }
    }

    if inscription_v3.size == inscription_input.target_size {
        emit!(InscriptionV3EventUpdate {
            id: inscription_v3.key(),
            data: InscriptionV3EventData {
                authority: inscription_v3.authority,
                root: inscription_v3.root,
                content_type: inscription_v3.content_type.clone(),
                encoding: inscription_v3.encoding.clone(),
                inscription_data: inscription_v3.inscription_data,
                order: inscription_v3.order,
                size: inscription_v3.size,
                validation_hash: inscription_v3.validation_hash.clone()
            },
        });
    } else {
        emit!(InscriptionResizeEvent {
            id: inscription_v3.key(),
            size: inscription_v3.size,
        });
    }

    Ok(())
}
