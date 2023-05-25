use anchor_lang::prelude::*;

use crate::state::{CollectionData, Metadata};
use prog_common::{close_account, TrySub};

#[derive(Accounts)]
#[instruction(bump_collection_data: u8, bump_metadata: u8)]
pub struct DeleteMetadata<'info> {

    pub authority: Signer<'info>,

    #[account(mut, seeds = [b"collection_data".as_ref(), collection_seed.key().as_ref()],
              bump = bump_collection_data, has_one = authority, has_one = collection_seed)]
    pub collection_data: Box<Account<'info, CollectionData>>,

    /// CHECK: Used for seed verification of collection data PDA account
    pub collection_seed: AccountInfo<'info>,

    #[account(mut, seeds = [b"metadata".as_ref(), mint.key().as_ref()],
              bump = bump_metadata, has_one = collection_data, has_one = mint)]
    pub metadata: Box<Account<'info, Metadata>>,

    /// CHECK: Mint address used for seed verification
    pub mint: AccountInfo<'info>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteMetadata>) -> Result<()> {

    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;

    // Close the collection data state account
    let metadata_account_info = &mut (*ctx.accounts.metadata).to_account_info();
    close_account(metadata_account_info, receiver)?;

    // Decrement collection data counter
    let collection_data = &mut ctx.accounts.collection_data;
    collection_data.collection_count.try_sub_assign(1)?;

    msg!("Metadata with pubkey {} now deleted", ctx.accounts.metadata.key());
    Ok(())
}
