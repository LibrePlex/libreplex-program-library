use anchor_lang::{system_program, InstructionData, Key, ToAccountMetas};
use anchor_spl::token::Mint as SplMint;
use solana_program::{instruction::Instruction, pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::ID;

use libreplex_metadata::{Asset, CreateMetadataInput};

pub async fn create_metadata_util(
    context: &mut ProgramTestContext,
    name: String,
    asset: Asset,
    symbol: String,
) -> Pubkey {
    let collection_authority = context.payer.pubkey();

    let mint = Keypair::new();

    // CREATE MINT

    let rent = context.banks_client.get_rent().await.unwrap();

    let allocate_ix = system_instruction::create_account(
        &context.payer.pubkey(),
        &mint.pubkey(),
        rent.minimum_balance(SplMint::LEN),
        SplMint::LEN as u64,
        &ID,
    );

    let initialize_ix = spl_token_2022::instruction::initialize_mint2(
        &ID,
        &mint.pubkey(),
        &context.payer.pubkey(),
        Some(&context.payer.pubkey()),
        0,
    )
    .unwrap();

    let create_account_tx = Transaction::new_signed_with_payer(
        &[allocate_ix, initialize_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mint],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(create_account_tx)
        .await
        .unwrap();

    let metadata = Pubkey::find_program_address(
        &[b"metadata", mint.pubkey().as_ref()],
        &libreplex_metadata::ID,
    )
    .0;

    let initialize_extension = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        &ID,
        &mint.pubkey(),
        Some(collection_authority),
        Some(metadata.key()),
    ).unwrap();

    

    let create_metadata_accounts = libreplex_metadata::accounts::CreateMetadata {
        payer: collection_authority,
        authority: collection_authority,
        metadata: metadata.key(),
        mint: mint.pubkey(),
        invoked_migrator_program: None,
        system_program: system_program::ID,
    }
    .to_account_metas(None);

    let create_metadata = libreplex_metadata::instruction::CreateMetadata {
        metadata_input: CreateMetadataInput {
            name,
            asset,
            update_authority: collection_authority,
            symbol,
            extension: libreplex_metadata::MetadataExtension::None,
        },
    };

    let create_metadata = Instruction {
        data: create_metadata.data(),
        program_id: libreplex_metadata::ID,
        accounts: create_metadata_accounts,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[initialize_extension, create_metadata],
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
