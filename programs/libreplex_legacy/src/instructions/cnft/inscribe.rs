use anchor_lang::prelude::*;

use libreplex_inscriptions::{
    instructions::SignerType, program::LibreplexInscriptions,
};
use bubblegum_proxy::state::TreeConfig;
use mpl_bubblegum::utils::get_asset_id;
use crate::instructions::{create_legacy_inscription_logic_v3, RootType};
use crate::{legacy_inscription::LegacyInscription, instructions::AuthorityType};
pub use bubblegum_proxy::MetadataArgs;
use super::{assert_can_inscribe_cnft, CNFTCheckAccounts, InscribeCNFTInput};


// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(input: Box<InscribeCNFTInput>)]
pub struct InscribeCNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: For PDA signing only
    #[account(
        mut,
        seeds=[
            get_asset_id(merkle_tree.key, input.nonce).as_ref(),
        ],
        bump
    )]
    pub legacy_signer: UncheckedAccount<'info>,
    

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
            get_asset_id(merkle_tree.key, input.nonce).as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

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

pub fn inscribe<'info>(
    ctx: Context<'_, '_, '_, 'info, InscribeCNFT<'info>>,
    input: Box<InscribeCNFTInput>
) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &mut ctx.accounts.inscription_summary;

    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;

    let legacy_inscription = &mut ctx.accounts.legacy_inscription;
    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object
    let payer = &ctx.accounts.payer;
    let legacy_signer = &ctx.accounts.legacy_signer;
    let tree = &ctx.accounts.merkle_tree;
    let expected_bump = ctx.bumps.legacy_signer;


    let asset_id = get_asset_id(tree.key, input.nonce);


    assert_can_inscribe_cnft(&input, &CNFTCheckAccounts {
        compression_program: &ctx.accounts.compression_program,
        merkle_tree: &ctx.accounts.merkle_tree,
        asset_id: &asset_id,
        collection_metadata: ctx.accounts.collection_metadata.as_ref().map(|a| {
            a.as_ref()
        }) ,
        authority: &ctx.accounts.authority,
        remaining_accounts: ctx.remaining_accounts,
    })?;
    

    create_legacy_inscription_logic_v3(
        RootType::Ghost(&asset_id),
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

