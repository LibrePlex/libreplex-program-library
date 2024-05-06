use anchor_lang::prelude::*;

use mpl_token_metadata::{
    instructions::{
        CreateV1Builder,
        MintV1Builder, // builders::{CreateBuilder, MintBuilder, UpdateBuilder, VerifyBuilder},
                                 // create_master_edition_v3, create_metadata_accounts_v3, set_and_verify_collection,
                                 // set_and_verify_sized_collection_item, update_metadata_accounts_v2, CreateArgs,
                                 // InstructionBuilder, MintArgs, RuleSetToggle, UpdateArgs, VerificationArgs,
    },
    types::{Creator, PrintSupply, TokenStandard},
    // state::{AssetData, Collection, Metadata, PrintSupply, TokenMetadataAccount, TokenStandard},
};
use solana_program::program::invoke_signed;


/// Accounts to mint an NFT.
pub struct MintAccounts<'info> {
    pub authority_pda: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub nft_owner: AccountInfo<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub nft_mint_authority: AccountInfo<'info>,
    pub nft_metadata: AccountInfo<'info>,
    pub nft_master_edition: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub token_metadata_program: AccountInfo<'info>,
    pub spl_token_program: AccountInfo<'info>,
    pub spl_ata_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub sysvar_instructions: Option<AccountInfo<'info>>,
    pub rule_set: Option<AccountInfo<'info>>,
}

/// Creates the metadata accounts and mint a new token.
pub fn create_and_mint(
    accounts: MintAccounts,
    authority_seeds: &[&[u8]],
    name: String,
    symbol: String,
    seller_fee_basis_points: u16,
    uri: String,
    creators: Vec<Creator>,
    max_supply: u64,
    is_mutable: bool
) -> Result<()> {
    // create metadata accounts

    let sysvar_instructions_info = accounts
        .sysvar_instructions
        .as_ref().unwrap();
        

    let create_ix = CreateV1Builder::new()
        .metadata(accounts.nft_metadata.key())
        .mint(accounts.nft_mint.key(), accounts.nft_mint.is_signer)
        .authority(accounts.nft_mint_authority.key())
        .payer(accounts.payer.key())
        .update_authority(accounts.authority_pda.key(), true)
        .master_edition(Some(accounts.nft_master_edition.key()))
        .creators(creators)
        .seller_fee_basis_points(seller_fee_basis_points)
        .token_standard(TokenStandard::NonFungible)
        .is_mutable(is_mutable) // starts off as mutable so we can do an update later
        .name(name)
        .symbol(symbol)
        .uri(uri)
        .decimals(0)
        .print_supply(if max_supply == 0 {
            PrintSupply::Zero
        } else {
            PrintSupply::Limited(max_supply)
        })
        .instruction();

    let create_infos = vec![
        accounts.nft_metadata.to_account_info(),
        accounts.nft_mint.to_account_info(),
        accounts.nft_mint_authority.to_account_info(),
        accounts.payer.to_account_info(),
        accounts.authority_pda.to_account_info(),
        accounts.nft_master_edition.to_account_info(),
        accounts.system_program.to_account_info(),
        sysvar_instructions_info.to_account_info(),
        accounts.spl_token_program.to_account_info(),
    ];

    invoke_signed(&create_ix, &create_infos, &[&authority_seeds])?;

    // mints one token

    let token_info = accounts
        .token_account
        .as_ref();
    
    let spl_ata_program_info = accounts.spl_ata_program.as_ref();

    let mut mint_builder = MintV1Builder::new();
    mint_builder
        .token(token_info.key())
        .token_owner(Some(accounts.nft_owner.key()))
        .metadata(accounts.nft_metadata.key())
        .master_edition(Some(accounts.nft_master_edition.key()))
        .mint(accounts.nft_mint.key())
        .payer(accounts.payer.key())
        .authority(accounts.authority_pda.key());

    let mint_infos = vec![
        token_info.to_account_info(),
        accounts.nft_owner.to_account_info(),
        accounts.nft_metadata.to_account_info(),
        accounts.nft_master_edition.to_account_info(),
        accounts.nft_mint.to_account_info(),
        accounts.payer.to_account_info(),
        accounts.authority_pda.to_account_info(),
        accounts.system_program.to_account_info(),
        sysvar_instructions_info.to_account_info(),
        accounts.spl_token_program.to_account_info(),
        spl_ata_program_info.to_account_info(),
    ];

    
    let mint_ix = mint_builder.amount(1).instruction();

    invoke_signed(&mint_ix, &mint_infos, &[&authority_seeds])?;

    Ok(())
}
