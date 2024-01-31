use anchor_lang::prelude::*;
use bubblegum_proxy::TreeConfig;
use libreplex_inscriptions::{InscriptionV3, program::LibreplexInscriptions, instructions::WriteToInscriptionInput};
use mpl_bubblegum::utils::get_asset_id;

use crate::LegacyInscription;

use libreplex_inscriptions::cpi::accounts::WriteToInscriptionV3;
use super::{InscribeCNFTInput, assert_can_inscribe_cnft, CNFTCheckAccounts};

#[derive(Accounts)]
#[instruction(compression_input: Box<InscribeCNFTInput>, 
    write_input: WriteToInscriptionInput)]
pub struct WriteCNFT<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in logic
    #[account(
        constraint = legacy_metadata.owner.key() == mpl_token_metadata::ID
    )]
    pub legacy_metadata: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut, seeds = [b"inscription_v3", 
        get_asset_id(merkle_tree.key, compression_input.nonce).as_ref()], bump)]
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
            get_asset_id(merkle_tree.key, compression_input.nonce).as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,

    /// CHECK: Checked by address
    #[account(address = spl_account_compression::ID)]
    pub compression_program: UncheckedAccount<'info>,
}

pub fn write<'info>(ctx: Context<'_, '_, '_, 'info, WriteCNFT<'info>>, 
    compression_input: Box<InscribeCNFTInput>, 
    write_input: WriteToInscriptionInput) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let tree = &ctx.accounts.merkle_tree;
    let asset_id = get_asset_id(tree.key, compression_input.nonce);

    assert_can_inscribe_cnft(&compression_input, &CNFTCheckAccounts {
        compression_program: &ctx.accounts.compression_program,
        merkle_tree: &ctx.accounts.merkle_tree,
        asset_id: &asset_id,
        collection_metadata: ctx.accounts.collection_metadata.as_ref().map(|a| {
            a.as_ref()
        }) ,
        authority: &ctx.accounts.authority,
        remaining_accounts: ctx.remaining_accounts,
    })?;

    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        asset_id.as_ref(),
        &[ctx.bumps.legacy_inscription],
    ];

    libreplex_inscriptions::cpi::write_to_inscription_v3(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            WriteToInscriptionV3 {
                authority: legacy_inscription.to_account_info(),
                payer: authority.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        write_input,
    )?;

    

    Ok(())
}