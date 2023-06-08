use crate::state::{Metadata};
use crate::{ CreateMetadataInput, Permissions, PermissionType, MetadataEvent, MetadataEventType};
use anchor_lang::prelude::*;


#[derive(Accounts)]
#[instruction(metadata_input: CreateMetadataInput)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"metadata", mint.key().as_ref()],
              bump, payer = signer, space = Metadata::BASE_SIZE + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(init, seeds = [b"permissions", metadata.key().as_ref(), signer.key().as_ref()],
            // all permissions start out with one permission, hence the +1
              bump, payer = signer, space = Permissions::BASE_SIZE + 1)] 
    pub permissions: Box<Account<'info, Permissions>>,

    /*
        Signer constraint to be relaxed later
        to allow for migration signatures etc.

        Currently this signer does not need to be a mint,
        but you can tag metadata onto anything.
    */
    pub mint: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>, metadata_input: CreateMetadataInput) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let authority = &ctx.accounts.signer;
    let permissions = &mut ctx.accounts.permissions;

    // Update the metadata state account
    metadata.mint = ctx.accounts.mint.key();
    metadata.is_mutable = true;
    metadata.symbol = metadata_input.symbol.clone();
    metadata.name = metadata_input.name.clone();
    metadata.creator = authority.key();
    metadata.description = metadata_input.description;
    metadata.asset = metadata_input.asset;

    permissions.bump = *ctx.bumps.get("permissions").unwrap();
    permissions.user = authority.key();
    permissions.reference = metadata.key();
    permissions.permissions = vec![PermissionType::Admin];

    msg!(
        "metadata created for mint with pubkey {}",
        ctx.accounts.mint.key()
    );

    emit!(MetadataEvent {
        id: metadata.key(),
        mint: ctx.accounts.mint.key(),
        event_type: MetadataEventType::Create
    });

    Ok(())
}
