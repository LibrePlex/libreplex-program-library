use anchor_spl::token::Mint as SplMint;
use solana_program_test::*;
use spl_token_2022::ID;
const METADATA_NAME: &str = "MD1";

const METADATA_NAME_NEW: &str = "MD2";

const METADATA_SYMBOL_NEW: &str = "SYMBOL2";


use anchor_lang::{system_program, InstructionData, Key, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey, system_instruction};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use libreplex_metadata::{Asset, CreateMetadataInput, UpdateMetadataInput};
pub mod create_metadata_util;
use create_metadata_util::*;

pub async fn update_metadata_util(
    context: &mut ProgramTestContext,
    metadata: Pubkey,
    name: String,
    asset: Asset,
    symbol: String,
) -> Pubkey {
    let collection_authority = context.payer.pubkey();

    let create_metadata_accounts = libreplex_metadata::accounts::UpdateMetadata {
        editor: context.payer.pubkey(),
        metadata: metadata.key(),
        system_program: system_program::ID,
        delegated_metadata_specific_permissions: None,
        delegated_group_wide_permissions: None,
        group: None
    }
    .to_account_metas(None);

    let update_metadata_input = libreplex_metadata::instruction::UpdateMetadata {
        input: UpdateMetadataInput {
            name,
            asset,
            symbol,
        },
    };

    let update_metadata_ix = Instruction {
        data: update_metadata_input.data(),
        program_id: libreplex_metadata::ID,
        accounts: create_metadata_accounts,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[update_metadata_ix],
        Some(&collection_authority),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    metadata
}


mod create_metadata_test {

    use std::borrow::BorrowMut;
   
    use anchor_lang::prelude::Account;
    use libreplex_metadata::{ Asset, Metadata};
    use solana_program::account_info::AccountInfo;
    use solana_sdk::signer::Signer;
   
    use super::*;

    #[tokio::test]
    async fn update_metadata() {
        let program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            processor!(libreplex_metadata::entry),
        );

        let mut context = program.start_with_context().await;
        let collection_authority = context.payer.pubkey();

        let metadata = create_metadata_util(
            context.borrow_mut(),
            METADATA_NAME.to_string(),
            Asset::Json {
                url: "https://collection-url.com".to_owned(),
            },
            "COOL".to_string(),
        )
        .await;

        // update metadata

        update_metadata_util(
            context.borrow_mut(),
            metadata,
            METADATA_NAME_NEW.to_string(),
            Asset::Image { url: "bla".to_string(), description: Some("zugululu".to_string()) },
            METADATA_SYMBOL_NEW.to_string()
        ).await;


        let mut metadata_account = context
            .banks_client
            .get_account(metadata)
            .await
            .unwrap()
            .unwrap();

        let metadata_account_info = AccountInfo::new(
            &metadata,
            false,
            false,
            &mut metadata_account.lamports,
            &mut metadata_account.data,
            &metadata_account.owner,
            metadata_account.executable,
            metadata_account.rent_epoch,
        );

        let metadata_obj: Account<Metadata> = Account::try_from(&metadata_account_info).unwrap();

        assert_eq!(metadata_obj.name, METADATA_NAME_NEW);
    }
}
