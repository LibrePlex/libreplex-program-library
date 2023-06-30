use crate::state::{Group, Metadata};
use crate::{MetadataExtension, Royalties,
};
use anchor_lang::prelude::*;

use crate::{errors::ErrorCode};

#[event]
struct ExtendMetadataEvent {
    id: Pubkey,
    mint: Pubkey,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ExtendMetadataInput {
    pub attributes: Vec<u8>,  // base: 4
    pub royalties: Option<Royalties>,
}

impl ExtendMetadataInput {
    pub const BASE_SIZE: usize = 4 + 1 + 1;

    pub fn get_size(&self) -> usize {
        ExtendMetadataInput::BASE_SIZE
            + self.attributes.len()
            + match &self.royalties {
                Some(x) => x.get_size(),
                None => 0,
            }
    }
}

pub fn validate_extend_metadata_input(
    metadata_input: &ExtendMetadataInput,
    _group: &Group,
) -> Result<()> {
  
    match &metadata_input.royalties {

        Some(royalties)=>{
            let total_shares: u16 = royalties.shares.iter().map(|x|x.share).sum();
            if total_shares != 10000 {
                return Err(ErrorCode::RoyaltiesBadSum.into())
            }
        },
        None =>{}
    }
    


    /*
        ensure that the initial render mode of the metadata matches the
        currently active render mode of the collection.

        NB: It is possible to change the active render mode of the collection.
        If that happens, it is the responsibility of the update auth holder
        to add the appropriate render mode data to each metadata.

    */

    // render_mode_data.is_compatible_with(&collection.collection_render_mode);

    // Ensure that the lengths of strings do not exceed the maximum allowed length

    Ok(())
}

#[derive(Accounts)]
#[instruction(extend_metadata_input: ExtendMetadataInput)]
pub struct ExtendMetadata<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(has_one = update_authority)]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(init, seeds = [b"metadata_extension".as_ref(), metadata.key().as_ref()],
              bump, payer = payer, space = MetadataExtension::BASE_SIZE + extend_metadata_input.get_size())]
    pub metadata_extended: Box<Account<'info, MetadataExtension>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ExtendMetadata>,
    extend_metadata_input: ExtendMetadataInput,
) -> Result<()> {
    let metadata_extended = &mut ctx.accounts.metadata_extended;

    let ExtendMetadataInput {
        attributes,
        royalties,
    } = extend_metadata_input;


    // ensure that the mint is in fact a mint
    //Account::<Mint>::try_from(&ctx.accounts.mint.to_account_info())?;

    // Update the metadata state account
    // metadata_extended.render_mode_data = vec![extend_metadata_input.render_mode_data];
    metadata_extended.attributes = attributes;
    metadata_extended.royalties = royalties;
    metadata_extended.signers = vec![]; // signers always start out empty

    let metadata = &ctx.accounts.metadata;

    msg!(
        "metadata created for mint with pubkey {}",
        metadata.mint
    );

    emit!(ExtendMetadataEvent {
        id: metadata_extended.key(),
        mint: metadata.mint.key(),
    });

    Ok(())
}
