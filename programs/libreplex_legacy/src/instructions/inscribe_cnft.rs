use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use libreplex_inscriptions::{
    instructions::SignerType, program::LibreplexInscriptions,
};
use bubblegum_proxy::{state::TreeConfig};

use mpl_bubblegum::hash_metadata;
use mpl_bubblegum::state::leaf_schema::LeafSchema;

use mpl_bubblegum::utils::get_asset_id;
use mpl_token_metadata::accounts::Metadata;
// use mpl_token_metadata::types::TokenStandard;
use crate::LegacyInscriptionErrorCode;
use crate::{legacy_inscription::LegacyInscription, instructions::AuthorityType};

pub use bubblegum_proxy::MetadataArgs;

use mpl_bubblegum::state::metaplex_adapter::MetadataArgs as BMetadataArgs;

use super::{check_metadata_uauth, create_legacy_inscription_logic_v3};


// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(    
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    metadata_args: BMetadataArgs,   
    leaf_delegate: Pubkey,
    leaf_owner: Pubkey)]
pub struct InscribeCNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,

    /// CHECK: For PDA signing only
    #[account(
        mut,
        seeds=[
            asset_id.key.as_ref().as_ref(),
        ],
        bump
    )]
    pub legacy_signer: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: Checked by tree authority
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(seeds = [merkle_tree.key().as_ref()], seeds::program = mpl_bubblegum::ID, 
        bump, owner = mpl_bubblegum::ID)]
    pub tree_authority: Account<'info, TreeConfig>,

    #[account(init,
        payer = payer,
        space = LegacyInscription::SIZE,
        seeds=[
            "legacy_inscription".as_bytes(),
            asset_id.key.as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    /// CHECK: Checked by address
    #[account(address = get_asset_id(merkle_tree.key, nonce))]
    pub asset_id: AccountInfo<'info>,

    /// CHECK: Checked in logic
    #[account(
        owner = mpl_token_metadata::ID
    )]
    pub collection_metadata: Option<UncheckedAccount<'info>>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,

    /// CHECK: Checked by address
    #[account(address = spl_account_compression::ID)]
    pub compression_program: UncheckedAccount<'info>,
}

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

pub fn inscribe_cnft(
    ctx: Context<InscribeCNFT>,
    input: Box<InscribeCNFTInput>
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &mut ctx.accounts.inscription_summary;

    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;

    let legacy_inscription = &mut ctx.accounts.legacy_inscription;
    let payer_key = ctx.accounts.payer.key();
    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object
    let payer = &ctx.accounts.payer;
    let legacy_signer = &ctx.accounts.legacy_signer;
    let tree = &ctx.accounts.merkle_tree;

    let authority = &ctx.accounts.authority;

    let expected_bump = ctx.bumps.legacy_signer;


    assert_can_inscribe_cnft(&input, &CNFTCheckAccounts {
        compression_program: &ctx.accounts.compression_program,
        merkle_tree: &ctx.accounts.merkle_tree,
        asset_id: &ctx.accounts.asset_id,
        collection_metadata: ctx.accounts.collection_metadata.as_ref().map(|a| {
            a.as_ref()
        }) ,
        authority: &ctx.accounts.authority,
        remaining_accounts: ctx.remaining_accounts,
    })?;
    

    create_legacy_inscription_logic_v3(
        &ctx.accounts.asset_id,
        legacy_inscription,
        AuthorityType::UpdateAuthority,
        inscription_v3,
        expected_bump,
        inscriptions_program,
        inscription_summary,
        legacy_signer,
        system_program,
        payer,
        inscription_data,
        "".to_string(),
        SignerType::LegacyMetadataSigner,
    )?;

    Ok(())
}

pub struct CNFTCheckAccounts<'a, 'info> {
    compression_program: & 'a AccountInfo<'info>,
    merkle_tree: & 'a AccountInfo<'info>,
    asset_id: & 'a AccountInfo<'info>,
    collection_metadata: Option<& 'a AccountInfo<'info>>,
    authority: & 'a AccountInfo<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],
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

