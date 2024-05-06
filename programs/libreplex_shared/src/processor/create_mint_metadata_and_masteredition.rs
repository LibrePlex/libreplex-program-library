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

use crate::SharedError;

/// Accounts to mint an NFT.
pub struct MintAccounts<'info> {
    pub authority_pda: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub nft_owner: AccountInfo<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub nft_mint_authority: AccountInfo<'info>,
    pub nft_metadata: AccountInfo<'info>,
    pub nft_master_edition: Option<AccountInfo<'info>>,
    pub token: Option<AccountInfo<'info>>,
    pub token_metadata_program: AccountInfo<'info>,
    pub spl_token_program: AccountInfo<'info>,
    pub spl_ata_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub sysvar_instructions: AccountInfo<'info>,
    // pub(crate) recent_slothashes: AccountInfo<'info>,
}

/// Creates the metadata accounts and mint a new token.
pub fn create_mint_with_metadata_and_masteredition(
    accounts: MintAccounts,
    authority_seeds: &[&[u8]],
    name: String,
    symbol: String,
    seller_fee_basis_points: u16,
    uri: String,
    creators: Option<Vec<Creator>>,
    max_supply: u64,
    // new_update_auth: Pubkey,
    is_mutable: bool,
    mint_amount: u64,
    decimals: u8,
    token_standard: TokenStandard,
) -> Result<()> {
    let sysvar_instructions_info = accounts.sysvar_instructions;

    let master_edition = accounts.nft_master_edition;

    let me_key = master_edition.clone().map(|x|x.key());
    let mut create_ix_builder = CreateV1Builder::new();
    create_ix_builder
        .metadata(accounts.nft_metadata.key())
        .mint(accounts.nft_mint.key(), accounts.nft_mint.is_signer)
        .authority(accounts.nft_mint_authority.key())
        .payer(accounts.payer.key())
        .update_authority(accounts.authority_pda.key(), true)
        .master_edition(me_key)
        .seller_fee_basis_points(seller_fee_basis_points)
        .is_mutable(is_mutable)
        .name(name)
        .symbol(symbol)
        .uri(uri)
        .decimals(decimals)
        .token_standard(token_standard)
        .print_supply(if max_supply == 0 {
            PrintSupply::Zero
        } else {
            PrintSupply::Limited(max_supply)
        });
    if let Some(x) = &creators {
        create_ix_builder.creators(x.to_vec());
    };
    let create_ix = create_ix_builder.instruction();

    let mut create_infos = vec![
        accounts.nft_metadata.to_account_info(),
        accounts.nft_mint.to_account_info(),
        accounts.nft_mint_authority.to_account_info(),
        accounts.payer.to_account_info(),
        accounts.authority_pda.to_account_info(),
    ];
    if let Some(x) = &master_edition {
        create_infos.push(x.to_account_info());
    }
    create_infos.push(accounts.system_program.to_account_info());
    create_infos.push(sysvar_instructions_info.to_account_info());
    create_infos.push(accounts.spl_token_program.to_account_info());

    invoke_signed(&create_ix, &create_infos, &[&authority_seeds])?;

    // mints one token

    let token_info = accounts
        .token
        .as_ref()
        .ok_or(SharedError::MissingTokenAccount)?;

    let spl_ata_program_info = accounts.spl_ata_program.as_ref();

    if mint_amount > 0 {
        let mut mint_builder = MintV1Builder::new();
        let me_key = master_edition.clone().map(|x| x.key());
        mint_builder
            .token(token_info.key())
            .token_owner(Some(accounts.nft_owner.key()))
            .metadata(accounts.nft_metadata.key())
            .master_edition(me_key)
            .mint(accounts.nft_mint.key())
            .payer(accounts.payer.key())
            .authority(accounts.authority_pda.key())
            .amount(mint_amount);

        let mut mint_infos = vec![
            token_info.to_account_info(),
            accounts.nft_owner.to_account_info(),
            accounts.nft_metadata.to_account_info(),
        ];
        if let Some(x) = &master_edition {
            mint_infos.push(x.to_account_info());
        }

        let mut remaining_accounts = vec![
            accounts.nft_mint.to_account_info(),
            accounts.payer.to_account_info(),
            accounts.authority_pda.to_account_info(),
            accounts.system_program.to_account_info(),
            sysvar_instructions_info.to_account_info(),
            accounts.spl_token_program.to_account_info(),
            spl_ata_program_info.to_account_info(),
        ];
        mint_infos.append(&mut remaining_accounts);

        let mint_ix = mint_builder.amount(mint_amount).instruction();

        invoke_signed(&mint_ix, &mint_infos, &[&authority_seeds])?;
    }

    Ok(())
}
