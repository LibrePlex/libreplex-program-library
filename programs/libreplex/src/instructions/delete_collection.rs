use anchor_lang::prelude::*;

use crate::{state::{Collection}, COLLECTION};
use prog_common::{close_account};

#[derive(Accounts)]
#[instruction(bump_collection_data: u8)]
pub struct DeleteCollection<'info> {

    pub authority: Signer<'info>,

    #[account(mut, seeds = [COLLECTION.as_ref(), collection_seed.key().as_ref()],
              bump = bump_collection_data, has_one = authority, has_one = collection_seed)]
    pub collection_data: Box<Account<'info, Collection>>,

    /// CHECK: Used for seed verification of collection data PDA account
    pub collection_seed: AccountInfo<'info>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteCollection>) -> Result<()> {

    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;

    // Close the collection data state account
    let collection_data_account_info = &mut (*ctx.accounts.collection_data).to_account_info();
    close_account(collection_data_account_info, receiver)?;

    msg!("Collection data with pubkey {} now deleted", ctx.accounts.collection_data.key());
    Ok(())
}
