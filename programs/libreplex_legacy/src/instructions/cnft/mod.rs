mod inscribe;
use bubblegum_proxy::hash_metadata;
pub use inscribe::*;

mod resize;
pub use resize::*;

mod write;
pub use write::*;

mod immutable;
pub use immutable::*;

use solana_program::{account_info::AccountInfo, keccak};
use mpl_token_metadata::accounts::Metadata;
use crate::LegacyInscriptionErrorCode;
use anchor_lang::prelude::*;

pub struct CNFTCheckAccounts<'a, 'info> {
    compression_program: & 'a AccountInfo<'info>,
    merkle_tree: & 'a AccountInfo<'info>,
    asset_id: & 'a Pubkey,
    collection_metadata: Option<& 'a AccountInfo<'info>>,
    authority: & 'a AccountInfo<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct InscribeCNFTInput {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    metadata_args: MetadataArgs,
    leaf_delegate: Pubkey,
    leaf_owner: Pubkey,
}


pub fn assert_can_inscribe_cnft(input: &InscribeCNFTInput, accounts: &CNFTCheckAccounts) -> Result<()> {
    let InscribeCNFTInput { root, data_hash, creator_hash, 
        nonce, index, metadata_args,
         leaf_delegate, leaf_owner } = input;

    let CNFTCheckAccounts { compression_program, merkle_tree,
         asset_id, collection_metadata, 
         authority, remaining_accounts } = accounts;

    let verify_leaf_ctx = CpiContext::new(
        compression_program.to_account_info(),
        spl_compression_proxy::cpi::accounts::VerifyLeaf {
            merkle_tree: merkle_tree.to_account_info(),
        },
    )
    .with_remaining_accounts(remaining_accounts.to_vec());

    let asset_id = asset_id.key();
    
    spl_compression_proxy::cpi::verify_leaf(
        verify_leaf_ctx,
        *root,
        keccak::hashv(&[
            &[1],
            asset_id.as_ref(),
            leaf_owner.as_ref(),
            leaf_delegate.as_ref(),
            nonce.to_le_bytes().as_ref(),
            data_hash.as_ref(),
            creator_hash.as_ref(),
        ])
        .to_bytes(),
        *index,
    )?;

    let incoming_data_hash = hash_metadata(metadata_args).expect("Can hash metadata");

    if data_hash != &incoming_data_hash {
        return Err(LegacyInscriptionErrorCode::DataHashMismatch.into());
    }

    if let Some(collection_details) = metadata_args.collection.as_ref() {
        let provided_collecton_metadata = collection_metadata
            .as_ref().ok_or(LegacyInscriptionErrorCode::BadAuthority)?;

        let collection_metadata = Metadata::from_bytes(&provided_collecton_metadata.try_borrow_data()?[..])?;

        if collection_metadata.mint !=  collection_details.key || 
            &collection_metadata.update_authority != authority.key {
            return Err(LegacyInscriptionErrorCode::BadAuthority.into());
        }

        return Ok(())
    }

    // if there is no collection, then use the leaf_owner to compare against

    if leaf_owner != authority.key {
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }

    Ok(())
}