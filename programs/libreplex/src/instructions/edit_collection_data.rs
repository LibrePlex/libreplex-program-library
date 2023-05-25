use anchor_lang::prelude::*;

use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction;

use crate::state::{CollectionData, CollectionDataInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH};

use prog_common::{TrySub, errors::ErrorCode};

#[derive(Accounts)]
#[instruction(new_collection_data_input: CollectionDataInput, bump_collection_data: u8)]
pub struct EditCollectionData<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [b"collection_data".as_ref(), collection_seed.key().as_ref()],
              bump = bump_collection_data, has_one = authority)]
    pub collection_data: Box<Account<'info, CollectionData>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub collection_seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> EditCollectionData<'info> {

    fn pay_lamports_difference(&self, lamports: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(self.authority.key, &self.collection_data.key(), lamports),
            &[
                self.authority.to_account_info(),
                self.collection_data.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )
            .map_err(Into::into)
    }
}

pub fn handler(ctx: Context<EditCollectionData>,
               new_collection_data_input: CollectionDataInput,
) -> Result<()> {

    let new_collection_data_input_size = new_collection_data_input.get_size();
    let CollectionDataInput {name, symbol, collection_url, nft_collection_data} = new_collection_data_input;

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

    // Calculate the total space required for the addition of the new data
    let old_data_bytes_amount = ctx.accounts.collection_data.to_account_info().data_len();
    let new_data_bytes_amount = 8 + 72 + new_collection_data_input_size;

    if new_data_bytes_amount > old_data_bytes_amount {

        let minimum_balance_for_rent_exemption: u64 = Rent::get()?.minimum_balance(new_data_bytes_amount);
        let lamports_difference: u64 = minimum_balance_for_rent_exemption.try_sub(ctx.accounts.collection_data.to_account_info().lamports())?;

        // Transfer the required difference in Lamports to accommodate this increase in space
        ctx.accounts.pay_lamports_difference(lamports_difference)?;

        // Reallocate the question pda account with the proper byte data size
        ctx.accounts.collection_data.to_account_info().realloc(new_data_bytes_amount, false)?;
    }

    // Update the collection data state account
    let collection_data = &mut ctx.accounts.collection_data;
    collection_data.name = name;
    collection_data.symbol = symbol;
    collection_data.collection_url = collection_url;
    collection_data.collection_count = 0;
    collection_data.nft_collection_data = nft_collection_data;

    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    Ok(())
}

