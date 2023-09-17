use anchor_lang::{system_program, InstructionData, Key, ToAccountMetas};

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;
use solana_sdk::{ signer::Signer, transaction::Transaction};

use libreplex_metadata::{Asset, UpdateMetadataInput};

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
        collection: None
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
