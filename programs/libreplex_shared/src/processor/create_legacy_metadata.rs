use anchor_lang::prelude::*;
use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3Builder, CreateV1Builder, CreateMasterEditionV3Builder},
    types::{Creator, PrintSupply, TokenStandard, DataV2},
};
use solana_program::{account_info::AccountInfo, program::invoke_signed};

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
    creators: Option<Vec<Creator>>,
    // new_update_auth: Pubkey,
    is_mutable: bool,
) -> Result<()> {
    
    let mut create_metadata_ix_builder = CreateMetadataAccountV3Builder::new();
    
    let data = DataV2{
        name,
        symbol,
        uri,
        seller_fee_basis_points,
        creators,
        collection: None,
        uses: None,
    };
    create_metadata_ix_builder
        .metadata(accounts.nft_metadata.key())
        .mint(accounts.nft_mint.key())
        .data(data.clone())
        .mint_authority(accounts.nft_mint_authority.key())
        .payer(accounts.payer.key())
        .update_authority(accounts.authority_pda.key(), true)
        .is_mutable(is_mutable);
    let create_ix = create_metadata_ix_builder.instruction();

    let create_infos = vec![
        accounts.nft_metadata.to_account_info(),
        accounts.nft_mint.to_account_info(),
        accounts.nft_mint_authority.to_account_info(),
        accounts.payer.to_account_info(),
        accounts.metadata_program.to_account_info(),
        accounts.token_program.to_account_info(),
        accounts.system_program.to_account_info(),
        accounts.rent.to_account_info(),
        accounts.sysvar_instructions_info.to_account_info()
    ];
    
    invoke_signed(&create_ix, &create_infos, &[&authority_seeds])?;


    Ok(())
}
