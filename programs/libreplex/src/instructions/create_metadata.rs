use anchor_lang::prelude::*;
use crate::state::{Collection, Metadata, MetadataInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH, CollectionPermissions, assert_valid_user_permissions, MetadataRenderMode};


use prog_common::{TryAdd, errors::ErrorCode};

#[event]
struct CreateMetadataEvent {
    id: Pubkey,
    collection: Pubkey,
    mint: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(metadata_input: MetadataInput)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub signer_collection_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(mut)]
    pub collection: Box<Account<'info, Collection>>,

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
    let collection = &ctx.accounts.collection;
    let user_permissions = &ctx.accounts.signer_collection_permissions;
    let authority = &ctx.accounts.signer;

    assert_valid_user_permissions(user_permissions, &collection.key(), authority.key)?;

    if !user_permissions.can_create_metadata {
        return Err(ErrorCode::MissingPermissionCreateMetadata.into());
    }


    let MetadataInput {name, symbol, render_mode_data, nft_metadata} = metadata_input;

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let symbol_length = symbol.len();


    

    if (name_length > MAX_NAME_LENGTH)  || (symbol_length > MAX_SYMBOL_LENGTH) {
        return Err(error!(ErrorCode::InvalidStringInput));
    }


    /* 
        ensure that the initial render mode of the metadata matches the 
        currently active render mode of the collection.

        NB: It is possible to change the active render mode of the collection.
        If that happens, it is the responsibility of the update auth holder
        to add the appropriate render mode data to each metadata.

    */ 
    
    render_mode_data.is_compatible_with(&collection.collection_render_mode);

    // Update the metadata state account
    metadata.collection = ctx.accounts.collection.key();
    metadata.mint = ctx.accounts.mint.key();
    metadata.name = name.clone();
    metadata.render_mode_data = vec![render_mode_data];
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
