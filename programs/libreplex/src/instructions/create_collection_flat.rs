use anchor_lang::prelude::*;

use crate::state::{Collection, CollectionInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH, COLLECTION, NftCollectionData};

use prog_common::{errors::ErrorCode};

#[derive(Accounts)]
#[instruction(
    seed: Pubkey,
)]
pub struct CreateCollectionFlat<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, seeds = [COLLECTION.as_ref(), seed.key().as_ref()],
      bump, payer = authority, space = 8 + 72 + 1000
    //   collection_input.get_size()
    )
      ]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCollectionFlat>,
               seed: Pubkey,
               symbol: String,
               name: String,
               collection_url: String,
               nft_collection_data: Option<NftCollectionData>,
) -> Result<()> {

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

        let royalty_shares_vec: Vec<u16> = nft_collection_data_unwrapped.royalty_shares.iter().map(|x| x.share).collect();

        for rs in royalty_shares_vec {
            if rs > 10_000 {
                return Err(error!(ErrorCode::InvalidBpsInput));
            }
        }
    }

    let collection = &mut ctx.accounts.collection;
    collection.authority = ctx.accounts.authority.key();
    collection.collection_seed = ctx.accounts.seed.key();
    collection.name = name;
    collection.symbol = symbol;
    collection.collection_url = collection_url;
    collection.collection_count = 0;
    collection.nft_collection_data = None; //nft_collection_data;

    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    Ok(())
}
