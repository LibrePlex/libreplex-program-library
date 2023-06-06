use anchor_lang::prelude::*;
use crate::instructions::has_permission;
use crate::state::{Collection};
use crate::{UpdateMetadataExtendedInput, Metadata, validate_metadata_input, NftMetadata, Permissions, PermissionType};


use prog_common::{errors::ErrorCode};

#[event]
struct EditMetadataEvent {
    id: Pubkey,
    collection: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(metadata_input: UpdateMetadataExtendedInput)]
pub struct EditMetadataExtended<'info> {
    #[account(mut)]
    pub editor: Signer<'info>,

    #[account()]
    pub permissions: Box<Account<'info, Permissions>>,

    // may be empty
    pub collection: UncheckedAccount<'info>,

    pub metadata: Box<Account<'info, Metadata>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EditMetadata>,
    metadata_input: UpdateMetadataExtendedInput,
) -> Result<()> {
    let editor = &ctx.accounts.editor;
    let collection = &ctx.accounts.collection;
    let metadata = &ctx.accounts.metadata;
    let permissions = &ctx.accounts.permissions;
    

    let UpdateMetadataExtendedInput {name, 
        render_mode_data, 
        nft_metadata:_, 
        invoked_permission } = metadata_input;

    if invoked_permission != PermissionType::Admin && invoked_permission != PermissionType::Edit {
        return Err(ErrorCode::InvalidPermissions.into())
    }
    
    let metadata_permissions_path = &[b"permissions", metadata.key().as_ref(), editor.key().as_ref()];
    let (metadata_permissions_key, metadata_bump) = Pubkey::find_program_address(metadata_permissions_path, &crate::id());

    if metadata_permissions_key == permissions.key() {
        if permissions.bump != metadata_bump  {
            return Err(ErrorCode::InvalidBump.into())
        }

        if has_permission(permissions.permissions, invoked_permission).is_none() {
            // TODO: Add support for edit as well as admin
            return Err(ErrorCode::InvalidPermissions.into())
        }
        
    }
    
    let editor_collection_permissions = &ctx.accounts.editor_collection_permissions;
    let editor_metadata_permissions = &ctx.accounts.editor_metadata_permissions;
    let collection = &ctx.accounts.collection;
    let metadata = &mut ctx.accounts.metadata;

    validate_metadata_input(&metadata_input, collection)?;


    // Update the metadata state account
    metadata.name = name.clone();
    metadata.render_mode_data = vec![render_mode_data];
   
    update_nft_metadata(metadata, metadata_input.nft_metadata)?;
    
    emit!(EditMetadataEvent{
        collection: collection.key(),
        id: metadata.key(),
        name
    });

    Ok(())
}

pub fn update_nft_metadata(metadata: &mut Account<Metadata>, input: Option<UpdateMetadataExtendedInput>) -> std::result::Result<(), Error> {
    match &metadata.nft_metadata {
        Some(metadata_old) => {
            match input {
                Some(metadata_new) => {
                    metadata.nft_metadata = Some(NftMetadata {
                        attributes: metadata_new.attributes,
                        signers: metadata_old.signers.clone()
                    });
                },
                None => {
                    // return Err(ErrorCode::IncompatibleMetadataType.into());
                }
            }
        }, None => {
            match input {
                Some(_) => {
                    // return Err(ErrorCode::IncompatibleMetadataType.into());
                },
                None => {
                    // do nothing - this is an SPL, there is no need to edit NFT metadata
                }
            }
        
        }
    }
    Ok(())
}