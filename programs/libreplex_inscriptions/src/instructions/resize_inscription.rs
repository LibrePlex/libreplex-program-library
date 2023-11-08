use std::cmp::{self};

use crate::instructions::InscriptionEventUpdate;
use crate::{Inscription, InscriptionEventData};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
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

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ResizeInscriptionInput {
    pub change: i32,
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

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated in logic
    #[account(mut,
        constraint = inscription.authority == authority.key())]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: validated in logic
    #[account(mut,
    seeds=[
        "inscription_data".as_bytes(),
        inscription.root.as_ref()
    ],bump)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ResizeInscription>,
    inscription_input: ResizeInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;

    let inscription_data = &mut ctx.accounts.inscription_data;

    let system_program = &mut ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;

    let size_old = inscription.size;
    msg!("before: {}", size_old);
    inscription.size = match inscription_input.change.cmp(&0) {
        Ordering::Greater => cmp::min(
            (inscription.size as i64 + inscription_input.change as i64) as u32,
            inscription_input.target_size,
        ),
        Ordering::Less => cmp::max(
            (inscription.size as i64 + inscription_input.change as i64) as u32,
            inscription_input.target_size,
        ),
        Ordering::Equal => inscription.size,
    };

    let size_new = inscription.size;

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(size_new as usize);

    let inscription_root = inscription.root.key();
    match size_new.cmp(&size_old) {
        Ordering::Greater => {
            msg!("Increasing");
            let lamports_diff = new_minimum_balance.saturating_sub(inscription_data.lamports());
            // reducing
            invoke(
                &system_instruction::transfer(&payer.key(), inscription_data.key, lamports_diff),
                &[
                    payer.to_account_info(),
                    inscription_data.to_account_info(),
                    system_program.to_account_info(),
                ],
            )?;
        }
        Ordering::Less => {
            msg!("Decreasing");
            let auth_seeds = [
                "inscription_data".as_bytes(),
                inscription_root.as_ref(),
                &[ctx.bumps["inscription_data"]],
            ];

            let lamports_diff = inscription_data
                .lamports()
                .saturating_sub(new_minimum_balance);
            // increasing
            invoke_signed(
                &system_instruction::transfer(inscription_data.key, &payer.key(), lamports_diff),
                &[
                    inscription_data.to_account_info(),
                    payer.to_account_info(),
                    system_program.to_account_info(),
                ],
                &[&auth_seeds],
            )?;
        }
        _ => {
            // do nothing - already at target
        }
    }

    inscription_data.realloc(inscription.size as usize, false)?;

    if inscription.size == inscription_input.target_size {
        emit!(InscriptionEventUpdate {
            id: inscription.key(),
            data: InscriptionEventData {
                authority: inscription.authority,
                root: inscription.root,
                media_type: inscription.media_type.clone(),
                encoding_type: inscription.encoding_type.clone(),
                inscription_data: inscription.inscription_data,
                order: inscription.order,
                size: inscription.size,
                validation_hash: inscription.validation_hash.clone()
            },
        });
    } else {
        emit!(InscriptionResizeEvent {
            id: inscription.key(),
            size: inscription.size,
        });
    }

    Ok(())
}
