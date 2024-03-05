use anchor_lang::prelude::*;
use crate::LegacyInscription;

use super::{InscribeCNFTInput, assert_can_inscribe_cnft, CNFTCheckAccounts};
use libreplex_inscriptions::{cpi::accounts::MakeInscriptionImmutableV3, InscriptionV3, program::LibreplexInscriptions};


#[derive(Accounts)]
#[instruction(compression_input: Box<InscribeCNFTInput>)]
pub struct MakeImmutableCNFT<'info> {
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

    #[account(seeds = [merkle_tree.key().as_ref()], seeds::program = bubblegum_proxy::ID, 
        bump, owner = bubblegum_proxy::ID)]
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

pub fn make_immutable<'info>(
    ctx: Context<'_, '_, '_, 'info, MakeImmutableCNFT<'info>>, 
    compression_input: Box<InscribeCNFTInput>) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let system_program = &ctx.accounts.system_program;
    let asset: &AccountInfo<'_> = &ctx.accounts.asset;
    let authority = &ctx.accounts.authority;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let inscription_summary = &ctx.accounts.inscription_summary;
    let tree = &ctx.accounts.merkle_tree;

    let asset_id = get_asset_id(tree.key, compression_input.nonce);


    assert_can_inscribe_cnft(&compression_input, &CNFTCheckAccounts {
        compression_program: &ctx.accounts.compression_program,
        merkle_tree: tree,
        asset_id: &asset_id,
        collection_metadata: ctx.accounts.collection_metadata.as_ref().map(|a| {
            a.as_ref()
        }) ,
        authority: &ctx.accounts.authority,
        remaining_accounts: ctx.remaining_accounts,
    })?;

    let inscription_auth_seeds: &[&[u8]] = &[
        "legacy_inscription".as_bytes(),
        asset.key.as_ref(),
        &[ctx.bumps.legacy_inscription],
    ];


    libreplex_inscriptions::cpi::make_inscription_immutable_v3(CpiContext::new_with_signer(
        inscriptions_program.to_account_info(),
        MakeInscriptionImmutableV3 {
            authority: legacy_inscription.to_account_info(),
            inscription_v3:inscription_v3.to_account_info(),
            system_program: system_program.to_account_info(),
            payer: authority.to_account_info(),
            inscription_summary: inscription_summary.to_account_info(),
        },
        &[inscription_auth_seeds],
    ))?;


    Ok(())
}