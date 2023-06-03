use anchor_lang::prelude::*;

use anchor_lang::solana_program::hash::hash;
use anchor_lang::solana_program::program::{invoke_signed};
use anchor_lang::solana_program::system_instruction::{create_account};

use crate::state::{Attribute, CollectionData, NftMetadata};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH};

use prog_common::{errors::ErrorCode};

#[derive(Accounts)]
#[instruction(bump_collection_data: u8)]
pub struct CreateMetadata<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [b"collection_data".as_ref(), collection_seed.key().as_ref()],
              bump = bump_collection_data, has_one = authority, has_one = collection_seed)]
    pub collection_data: Box<Account<'info, CollectionData>>,

    /// CHECK: Used for seed verification of collection data PDA account
    pub collection_seed: AccountInfo<'info>,

    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    pub mint: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>,
               name: String,
               url: String,
               is_mutable: bool,
               nft_data: Option<NftMetadata>
) -> Result<()> {

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let url_length = url.len();

    if (name_length > MAX_NAME_LENGTH) || (url_length > MAX_URL_LENGTH) {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    // find bump - doing this program-side to reduce amount of info to be passed in (tx size)
    let (_pk, bump) = Pubkey::find_program_address(
        &[
            b"metadata".as_ref(),
            ctx.accounts.mint.key().as_ref()
        ],
        ctx.program_id,
    );

    if ctx.accounts.metadata.data_is_empty() {

        // Calculate data sizes and convert data to slice arrays
        let mut name_buffer: Vec<u8> = Vec::new();
        name.serialize(&mut name_buffer).unwrap();

        let name_buffer_as_slice: &[u8] = name_buffer.as_slice();
        let name_buffer_slice_length: usize = name_buffer_as_slice.len();
        let name_buffer_slice_end_byte = 72 + name_buffer_slice_length;

        let mut url_buffer: Vec<u8> = Vec::new();
        url.serialize(&mut url_buffer).unwrap();

        let url_buffer_as_slice: &[u8] = url_buffer.as_slice();
        let url_buffer_slice_length: usize = url_buffer_as_slice.len();
        let url_buffer_slice_end_byte = name_buffer_slice_end_byte + url_buffer_slice_length;

        let mut nft_data_buffer: Vec<u8> = Vec::new();
        nft_data.serialize(&mut nft_data_buffer).unwrap();

        let nft_data_buffer_as_slice: &[u8] = nft_data_buffer.as_slice();
        let nft_data_buffer_slice_length: usize = nft_data_buffer_as_slice.len();
        let nft_data_buffer_slice_end_byte = url_buffer_slice_end_byte + 1 + nft_data_buffer_slice_length;
    }

    Ok(())

}
