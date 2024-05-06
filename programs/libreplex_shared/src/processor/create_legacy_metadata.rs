use anchor_lang::prelude::*;
use mpl_token_metadata::{
    instructions::CreateV1CpiBuilder,
    types::{Creator, TokenStandard},
};
use solana_program::account_info::AccountInfo;

/// Accounts to mint an NFT.
pub struct CreateMetadataAccounts<'info> {
    pub authority_pda: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub nft_mint_authority: AccountInfo<'info>,
    pub nft_metadata: AccountInfo<'info>,
    pub nft_master_edition: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub sysvar_instructions_info: AccountInfo<'info>,
    pub metadata_program: AccountInfo<'info>,
    pub token_program:  AccountInfo<'info>,
    // pub(crate) recent_slothashes: AccountInfo<'info>,
}

/// Creates the metadata accounts and mint a new token.
pub fn create_legacy_metadata(
    accounts: CreateMetadataAccounts,
    authority_seeds: &[&[u8]],
    name: String,
    symbol: String,
    seller_fee_basis_points: u16,
    uri: String,
    creators: Vec<Creator>,
    // new_update_auth: Pubkey,
    is_mutable: bool,
) -> Result<()> {
    let mut create_metadata_ix_builder = CreateV1CpiBuilder::new(&accounts.metadata_program);
    

    create_metadata_ix_builder
    .metadata(&accounts.nft_metadata)
    .mint(&accounts.nft_mint, false)
    .authority(&accounts.nft_mint_authority)
    .payer(&accounts.payer)
    .update_authority(&accounts.authority_pda, true)
    .system_program(&accounts.system_program)
    .sysvar_instructions(&accounts.sysvar_instructions_info)
    .spl_token_program(&accounts.token_program)
    .name(name).symbol(symbol)
    .uri(uri)
    .seller_fee_basis_points(seller_fee_basis_points)
    .creators(creators)
    .primary_sale_happened(true)
    .is_mutable(is_mutable)
    .token_standard(TokenStandard::Fungible).invoke_signed(&[&authority_seeds])?;

    Ok(())
}
