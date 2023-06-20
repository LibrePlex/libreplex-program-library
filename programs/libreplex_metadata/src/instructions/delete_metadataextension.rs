use anchor_lang::prelude::*;


use crate::{state::{Metadata}, MetadataExtension};


#[derive(Accounts)]
pub struct DeleteMetadataExtension<'info> {
    pub update_authority: Signer<'info>,

    #[account(has_one = update_authority)]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(mut, close = update_authority, seeds = [b"metadata_extension".as_ref(), metadata.key().as_ref()],bump)]
    pub metadata_extension: Box<Account<'info, MetadataExtension>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteMetadataExtension>) -> Result<()> {
    msg!("Metadata extension with pubkey {} now deleted", ctx.accounts.metadata_extension.key());
    Ok(())
}