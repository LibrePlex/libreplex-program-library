use anchor_lang::prelude::*;
use crate::state::{Collection, Metadata, MetadataInput};
use crate::{CollectionPermissions, assert_valid_collection_permissions, validate_metadata_input};

use crate::state::{Collection, Metadata, MetadataInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH, COLLECTION, METADATA};

use prog_common::{TryAdd, errors::ErrorCode};

#[event]
struct CreateMetadataEvent {
    id: Pubkey,
    collection: Pubkey,
    mint: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(metadata_input: MetadataInput, bump_collection_data: u8)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection_data.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub signer_collection_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(mut)]
    pub collection_data: Box<Account<'info, CollectionData>>,

    #[account(init, seeds = [b"metadata".as_ref(), mint.key().as_ref()],
              bump, payer = signer, space = 8 + 65 + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,
    pub mint: Signer<'info>,


    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>,
               metadata_input: MetadataInput,
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let collection_data = &ctx.accounts.collection_data;
    let user_permissions = &ctx.accounts.signer_collection_permissions;
    let authority = &ctx.accounts.signer;

    assert_valid_collection_permissions(user_permissions, &collection.key(), authority.key)?;

    if !user_permissions.can_create_metadata {
        return Err(ErrorCode::MissingPermissionCreateMetadata.into());
    }

    validate_metadata_input(&metadata_input)?;

    let MetadataInput {name, metadata_url, nft_metadata} = metadata_input;

    // Update the metadata state account
    metadata.collection_data = ctx.accounts.collection_data.key();
    metadata.mint = ctx.accounts.mint.key();
    metadata.name = name.clone();
    metadata.url = metadata_url;
    metadata.is_mutable = true;
    metadata.nft_data = nft_metadata;

    // Increment collection data counter
    let collection = &mut ctx.accounts.collection;
    collection.item_count.try_add_assign(1)?;

    msg!("metadata created for mint with pubkey {}", ctx.accounts.mint.key());

    emit!(CreateMetadataEvent {
        collection: collection.key(),
        id: metadata.key(),
        mint: ctx.accounts.mint.key(),
        name: name,
    });

    Ok(())

}
