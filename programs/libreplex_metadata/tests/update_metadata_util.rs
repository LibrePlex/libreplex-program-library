use anchor_lang::{system_program, InstructionData, Key, ToAccountMetas};

use libreplex_metadata::instructions::UpdateMetadataInput;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;
use solana_sdk::{ signer::Signer, transaction::Transaction};

pub async fn update_metadata_util(
    context: &mut ProgramTestContext,
    metadata: Pubkey,
    name: Option<String>,
    url_json: Option<String>,
    symbol: Option<String>,
) -> Pubkey {
    let collection_authority = context.payer.pubkey();

    let create_metadata_accounts = libreplex_metadata::accounts::UpdateMetadata {
        payer: context.payer.pubkey(),
        metadata: metadata.key(),
        system_program: system_program::ID,
        update_authority: context.payer.pubkey(),
    }
    .to_account_metas(None);

    let update_metadata_input = libreplex_metadata::instruction::UpdateMetadata {
        input: UpdateMetadataInput {
            name,
            url_json,
            symbol,
            update_authority: None
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
