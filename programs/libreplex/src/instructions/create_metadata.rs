use anchor_lang::prelude::*;

use crate::state::{Collection, Metadata, MetadataInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH, COLLECTION, METADATA};

use prog_common::{TryAdd, errors::ErrorCode};

#[derive(Accounts)]
#[instruction(metadata_input: MetadataInput, bump_collection_data: u8)]
pub struct CreateMetadata<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [COLLECTION.as_ref(), collection_seed.key().as_ref()],
              bump = bump_collection_data, has_one = authority, has_one = collection_seed)]
    pub collection_data: Box<Account<'info, Collection>>,

    /// CHECK: Used for seed verification of collection data PDA account
    pub collection_seed: AccountInfo<'info>,

    #[account(init, seeds = [METADATA.as_ref(), mint.key().as_ref()],
              bump, payer = authority, space = 8 + 65 + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    pub mint: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>,
               metadata_input: MetadataInput,
) -> Result<()> {

    let MetadataInput {name, symbol, metadata_url, nft_metadata} = metadata_input;

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let symbol_length = symbol.len();
    let url_length = metadata_url.len();

    if (name_length > MAX_NAME_LENGTH)  || (symbol_length > MAX_SYMBOL_LENGTH) || (url_length > MAX_URL_LENGTH) {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    // Update the metadata state account
    let metadata = &mut ctx.accounts.metadata;
    metadata.collection_data = ctx.accounts.collection_data.key();
    metadata.mint = ctx.accounts.mint.key();
    metadata.name = name;
    metadata.url = metadata_url;
    metadata.is_mutable = true;
    metadata.nft_data = nft_metadata;

    // Increment collection data counter
    let collection_data = &mut ctx.accounts.collection_data;
    collection_data.collection_count.try_add_assign(1)?;

    msg!("metadata created for mint with pubkey {}", ctx.accounts.mint.key());

    Ok(())

}
