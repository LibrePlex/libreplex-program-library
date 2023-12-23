use anchor_lang::{prelude::*, system_program};

use crate::{Collection, Metadata};

use crate::errors::ErrorCode;

// Adds a metadata to a group
#[derive(Accounts)]
pub struct AddMetadataToCollection<'info> {
    pub metadata_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        constraint = metadata.update_authority == metadata_authority.key())]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(mut)]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddMetadataToCollection>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    if !metadata.collection.eq(&system_program::ID) {
        return Err(ErrorCode::MetadataBelongsToCollection.into());
    }

    let collection = &ctx.accounts.collection;

    metadata.collection = collection.key();
    metadata.update_authority = collection.key();

    Ok(())
}
