use anchor_lang::prelude::*;
use super::{InscribeCNFTInput, assert_can_inscribe_cnft, CNFTCheckAccounts, ModifyInscription};
use libreplex_inscriptions::cpi::accounts::MakeInscriptionImmutableV3;

pub fn make_immutable(ctx: Context<ModifyInscription>, 
    compression_input: Box<InscribeCNFTInput>, wooo: String) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let asset: &AccountInfo<'_> = &ctx.accounts.asset;
    let authority = &ctx.accounts.authority;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let payer = &ctx.accounts.payer;
    let inscription_summary = &ctx.accounts.inscription_summary;


    assert_can_inscribe_cnft(&compression_input, &CNFTCheckAccounts {
        compression_program: &ctx.accounts.compression_program,
        merkle_tree: &ctx.accounts.merkle_tree,
        asset_id: &ctx.accounts.asset,
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