use anchor_lang::prelude::*;
use libreplex_inscriptions::Inscription;
use libreplex_inscriptions::program::LibreplexInscriptions;

use crate::instructions::DeleteEvent;
use crate::{Metadata, DelegatePermissions, PermissionType, Asset};

use crate::{errors::ErrorCode};


// Adds a metadata to a group
#[derive(Accounts)]
pub struct DeleteMetadataInscription<'info> {
    pub metadata_authority: Signer<'info>,

    #[account(
        mut, 
        close = metadata_authority,
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    // Derived from the editor, the metadata's update auth and the the metadata itself
    #[account(seeds = ["permissions".as_ref(), 
                        metadata_authority.key().as_ref(), 
                        metadata.update_authority.as_ref(), 
                        metadata.key().as_ref()], 
                        bump)]
    pub delegated_metadata_specific_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    pub inscription: Account<'info, Inscription>,

    pub inscription_authority: Signer<'info>,


    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>
}

pub fn handler(ctx: Context<DeleteMetadataInscription>
) -> Result<()> {

    

    let metadata = &mut ctx.accounts.metadata;
    let _inscriptions_program = &mut ctx.accounts.inscriptions_program;

    let  _inscription_authority = &ctx.accounts.inscription_authority;
    match &metadata.asset {
        Asset::Inscription {
            account_id: _,
            data_type: _,
            description: _
        } =>  {
            
        },
        _ => {
            return Err(ErrorCode::OnlyUsedForInscriptionMetadata.into())    
        }
    };

    if metadata.collection.is_some() {
        return Err(ErrorCode::MetadataBelongsToCollection.into())
    }

    if !metadata.is_mutable {
        return Err(ErrorCode::MetadataIsNotMutable.into())
    }

    let metadata_authority = &ctx.accounts.metadata_authority;

    let mut can_delete_metadata = &metadata.update_authority == metadata_authority.key;
    
    if let Some(delegated_metadata_specific_permissions_account) 
        = &ctx.accounts.delegated_metadata_specific_permissions {
            can_delete_metadata = can_delete_metadata || delegated_metadata_specific_permissions_account.permissions.contains(&PermissionType::Delete)
    }

    if !can_delete_metadata {
        return Err(ErrorCode::MissingPermissionDeleteMetadata.into())
    }
    
    // libreplex_inscriptions::cpi::delete_inscription(
    //     CpiContext::new(
    //         inscriptions_program.to_account_info(),
    //         DeleteInscription {
    //             // raffle is the owner of the pod
    //             inscription: inscription.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //             payer: signer.to_account_info()
    //         }
    //     ),
    //     libreplex_inscriptions::instructions::CreateInscriptionInput {
    //         authority: metadata_input.inscription_input.authority,
    //         max_data_length: metadata_input.inscription_input.max_data_length,
    //     }
    // )?;

    emit!(DeleteEvent {
        id: metadata.mint.key(),
    });

    Ok(())
}
