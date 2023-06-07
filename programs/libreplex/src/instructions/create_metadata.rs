use crate::state::{Collection, Metadata, MetadataInput};
use crate::{
    validate_metadata_input,
    NftMetadata, Permissions, assert_valid_permissions,
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
    pub permissions: Box<Account<'info, Permissions>>,

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
    let permissions = &ctx.accounts.permissions;
    let authority = &ctx.accounts.signer;

    assert_valid_permissions(permissions, collection.key(), authority.key(), &crate::PermissionType::Admin)?;

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