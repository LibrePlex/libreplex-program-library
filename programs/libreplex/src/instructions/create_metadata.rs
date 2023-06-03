use crate::state::{Collection, Metadata, MetadataInput};
use crate::{
    assert_valid_collection_permissions, validate_metadata_input, CollectionPermissions,
    NftMetadata,
};
use anchor_lang::prelude::*;

use anchor_spl::token::Mint;
use prog_common::{errors::ErrorCode, TryAdd};

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
              bump, payer = signer, space = Metadata::BASE_SIZE + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    /*
        Signer constraint to be relaxed later to allow for migration signatures etc. 
        Q: What to do with mints without metadata?
    */
    
    pub mint: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>, metadata_input: MetadataInput) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let collection = &mut ctx.accounts.collection;
    let user_permissions = &ctx.accounts.signer_collection_permissions;
    let authority = &ctx.accounts.signer;

    assert_valid_collection_permissions(user_permissions, &collection.key(), authority.key)?;

    if !user_permissions.can_create_metadata {
        return Err(ErrorCode::MissingPermissionCreateMetadata.into());
    }

    validate_metadata_input(&metadata_input, collection)?;

    // ensure that the mint is in fact a mint
    Account::<Mint>::try_from(&ctx.accounts.mint.to_account_info())?;

    // Update the metadata state account
    metadata.collection = collection.key();
    metadata.mint = ctx.accounts.mint.key();
    metadata.name = metadata_input.name.clone();
    metadata.render_mode_data = vec![metadata_input.render_mode_data];
    metadata.is_mutable = true;

    // should we do some validation here against collection type (i.e. SPL -v- NFT)?

    match metadata_input.nft_metadata {
        Some(x) => {
            metadata.nft_metadata = Some(NftMetadata {
                attributes: x.attributes,
                signers: vec![],
            });
        }
        None => {}
    }

    // Increment collection data counter
    collection.item_count.try_add_assign(1)?;

    msg!(
        "metadata created for mint with pubkey {}",
        ctx.accounts.mint.key()
    );

    emit!(CreateMetadataEvent {
        collection: collection.key(),
        id: metadata.key(),
        mint: ctx.accounts.mint.key(),
        name: metadata_input.name,
    });

    Ok(())
}
