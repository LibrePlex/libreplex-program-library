use anchor_lang::prelude::*;
use crate::state::{Collection};
use crate::{CollectionPermissions, MetadataInput, Metadata, MetadataPermissions, assert_valid_collection_permissions, assert_valid_metadata_permissions, validate_metadata_input, NftMetadata, NftMetadataInput};


use prog_common::{errors::ErrorCode};

#[event]
struct EditMetadataEvent {
    id: Pubkey,
    collection: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(metadata_input: MetadataInput)]
pub struct EditMetadata<'info> {
    #[account(mut)]
    pub editor: Signer<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection.key().as_ref(), editor.key().as_ref()], 
        bump)]
    pub editor_collection_permissions: Option<Box<Account<'info, CollectionPermissions>>>,

    #[account(
        seeds = ["permissions".as_ref(), 
        collection.key().as_ref(), 
        editor.key().as_ref(), 
        metadata.key().as_ref()], 
        bump)]
    pub editor_metadata_permissions: Option<Box<Account<'info, MetadataPermissions>>>,

    pub collection: Box<Account<'info, Collection>>,

    #[account(mut, has_one = collection)]
    pub metadata: Box<Account<'info, Metadata>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EditMetadata>,
    metadata_input: MetadataInput,
) -> Result<()> {
    let editor = &ctx.accounts.editor;
    let editor_collection_permissions = &ctx.accounts.editor_collection_permissions;
    let editor_metadata_permissions = &ctx.accounts.editor_metadata_permissions;
    let collection = &ctx.accounts.collection;
    let metadata = &mut ctx.accounts.metadata;

    validate_metadata_input(&metadata_input)?;

    let has_metadata_permission = match editor_metadata_permissions {
        Some(metadata_permissions) => {
            assert_valid_metadata_permissions(&metadata_permissions, &metadata.key(), editor.key)
                .map(|_| metadata_permissions.can_modify)
        },
        None => Ok(false),
    }?;


    let has_collection_permission = match editor_collection_permissions {
        Some(collection_permissions) => {
            assert_valid_collection_permissions(&collection_permissions, &collection.key(), editor.key)
                .map(|_| collection_permissions.can_edit_metadata)
        },
        None => Ok(false),
    }?;

    if !has_metadata_permission && !has_collection_permission {
          return Err(error!(ErrorCode::MissingPermissionEditMetadata));
    }

<<<<<<< HEAD
    let MetadataInput {name, metadata_url, nft_metadata} = metadata_input;

    // Update the metadata state account
    metadata.name = name.clone();
    metadata.url = metadata_url;
    metadata.nft_data = nft_metadata;

=======
    let MetadataInput {name, render_mode_data, nft_metadata:_} = metadata_input;

    // Update the metadata state account
    metadata.name = name.clone();
    metadata.render_mode_data = vec![render_mode_data];
   
    update_nft_metadata(metadata, metadata_input.nft_metadata)?;
    
>>>>>>> ab3118e (Fix metadata creation, account sizes and attributes)
    emit!(EditMetadataEvent{
        collection: collection.key(),
        id: metadata.key(),
        name
    });

    Ok(())
}

pub fn update_nft_metadata(metadata: &mut Account<Metadata>, input: Option<NftMetadataInput>) -> std::result::Result<(), Error> {
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
