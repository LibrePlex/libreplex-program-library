mod inscribe;
use bubblegum_proxy::TreeConfig;
pub use inscribe::*;

mod resize;
use libreplex_inscriptions::InscriptionV3;
use libreplex_inscriptions::program::LibreplexInscriptions;
use mpl_bubblegum::utils::get_asset_id;
pub use resize::*;

mod write;
pub use write::*;

mod immutable;
pub use immutable::*;

use solana_program::account_info::AccountInfo;
use mpl_token_metadata::accounts::Metadata;
use crate::{LegacyInscriptionErrorCode, LegacyInscription};
use mpl_bubblegum::hash_metadata;
use mpl_bubblegum::state::leaf_schema::LeafSchema;
use mpl_bubblegum::state::metaplex_adapter::MetadataArgs as BMetadataArgs;
use anchor_lang::prelude::*;


#[derive(Accounts)]
#[instruction(compression_input: Box<InscribeCNFTInput>)]
pub struct ModifyInscription<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked by address
    #[account(address = get_asset_id(merkle_tree.key, compression_input.nonce))]
    pub asset: AccountInfo<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut, seeds = [b"inscription_v3", 
        asset.key.as_ref()], bump)]
    pub inscription_v3: Account<'info, InscriptionV3>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// CHECK: Checked by tree authority
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(seeds = [merkle_tree.key().as_ref()], seeds::program = mpl_bubblegum::ID, 
        bump, owner = mpl_bubblegum::ID)]
    pub tree_authority: Account<'info, TreeConfig>,

    /// CHECK: Checked in logic
    #[account(
        owner = mpl_token_metadata::ID
    )]
    pub collection_metadata: Option<UncheckedAccount<'info>>,
    

    #[account(mut,
        seeds=[
            "legacy_inscription".as_bytes(),
            asset.key().as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,

    /// CHECK: Checked by address
    #[account(address = spl_account_compression::ID)]
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,
}

pub struct CNFTCheckAccounts<'a, 'info> {
    compression_program: & 'a AccountInfo<'info>,
    merkle_tree: & 'a AccountInfo<'info>,
    asset_id: & 'a AccountInfo<'info>,
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
    metadata_args: BMetadataArgs,
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
    let leaf_schema = LeafSchema::new_v0(
        asset_id,
        leaf_owner.clone(),
        leaf_delegate.clone(),
        *nonce,
        data_hash.clone(),
        creator_hash.clone(),
    );

    spl_compression_proxy::cpi::verify_leaf(
        verify_leaf_ctx,
        root.clone(),
        leaf_schema.to_node(),
        *index,
    )?;

    let incoming_data_hash = hash_metadata(metadata_args).expect("Can hash metadata");

    if data_hash != &incoming_data_hash {
        return Err(LegacyInscriptionErrorCode::DataHashMismatch.into());
    }

    if let Some(collection_details) = metadata_args.collection {
        let provided_collecton_metadata = collection_metadata
            .as_ref().ok_or(LegacyInscriptionErrorCode::BadAuthority)?;

        let collection_metadata = Metadata::from_bytes(&provided_collecton_metadata.try_borrow_data()?[..])?;

        if collection_metadata.mint !=  collection_details.key || 
            &collection_metadata.update_authority != authority.key {
            return Err(LegacyInscriptionErrorCode::BadAuthority.into());
        }

        return Ok(())
    }

    if leaf_owner != authority.key {
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }

    return Ok(());
}