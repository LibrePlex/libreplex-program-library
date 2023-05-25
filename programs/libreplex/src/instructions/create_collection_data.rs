use anchor_lang::prelude::*;

use crate::state::{CollectionData, CollectionDataInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH};

use prog_common::{errors::ErrorCode};

#[derive(Accounts)]
#[instruction(collection_data_input: CollectionDataInput)]
pub struct CreateCollectionData<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, seeds = [b"collection_data".as_ref(), collection_seed.key().as_ref()],
      bump, payer = authority, space = 8 + 72 + collection_data_input.get_size())]
    pub collection_data: Box<Account<'info, CollectionData>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub collection_seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCollectionData>,
               collection_data_input: CollectionDataInput,
) -> Result<()> {

    let CollectionDataInput {name, symbol, collection_url, nft_collection_data} = collection_data_input;

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let symbol_length = symbol.len();
    let url_length = collection_url.len();

    if (name_length > MAX_NAME_LENGTH) || (symbol_length > MAX_SYMBOL_LENGTH) || (url_length > MAX_URL_LENGTH) {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    if nft_collection_data.is_some() {
        let nft_collection_data_unwrapped = nft_collection_data.as_ref().unwrap();
        let royalty_bps = nft_collection_data_unwrapped.royalty_bps;

        // Ensure that basis points are between 0-10,000
        if royalty_bps > 10_000 {
            return Err(error!(ErrorCode::InvalidBpsInput));
        }

        let royalty_shares_vec: Vec<u16> = nft_collection_data_unwrapped.royalty_shares.iter().map(|x| x.royalty_share).collect();

        for rs in royalty_shares_vec {
            if rs > 10_000 {
                return Err(error!(ErrorCode::InvalidBpsInput));
            }
        }
    }

    // Update the collection data state account
    let collection_data = &mut ctx.accounts.collection_data;
    collection_data.authority = ctx.accounts.authority.key();
    collection_data.collection_seed = ctx.accounts.collection_seed.key();
    collection_data.name = name;
    collection_data.symbol = symbol;
    collection_data.collection_url = collection_url;
    collection_data.collection_count = 0;
    collection_data.nft_collection_data = nft_collection_data;

    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    Ok(())
}
