use solana_program_test::*;
use anchor_spl::token::{ID, self};
use anchor_spl::token::Mint as SplMint;

mod permissions {
    use anchor_lang::{system_program, ToAccountMetas, InstructionData, Key};
    use anchor_spl::token::spl_token;
    use libreplex_metadata::{GroupInput, CreateMetadataInput, Asset, accounts::CreateMetadata};
    use solana_program::{pubkey::Pubkey, instruction::Instruction, program::invoke_signed, system_instruction, rent::Rent, sysvar::Sysvar};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_metadata_delegated() {
        let program = ProgramTest::new("libreplex_metadata", libreplex_metadata::ID, 
        processor!(libreplex_metadata::entry));

        let mut context = program.start_with_context().await;
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

        let initialize_ix = spl_token::instruction::initialize_mint2(
            &spl_token::id(),
            &mint.pubkey(),
            &context.payer.pubkey(),
            Some(&context.payer.pubkey()),
            0,
        ).unwrap();

        let create_account_tx = Transaction::new_signed_with_payer(
            &[allocate_ix, initialize_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &mint],
            context.last_blockhash
        );

        context.banks_client.process_transaction(create_account_tx).await.unwrap();



        let metadata = Pubkey::find_program_address(
            &[b"metadata", mint.pubkey().as_ref()],
            &libreplex_metadata::ID,
        )
        .0;

        
      

        let create_metadata_accounts = CreateMetadata {
            signer: collection_authority,
            authority: collection_authority,
            metadata:  metadata.key(),
            mint: mint.pubkey(),
            invoked_migrator_program: None,
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        let create_metadata = libreplex_metadata::instruction::CreateMetadata {
            metadata_input: CreateMetadataInput {
                // collection_url: "COOLIO.COM".to_string(),
                name: "COOLIO COLLECTION".to_string(),
                asset: Asset::Json {
                    url: "https://collection-url.com".to_owned()
                },
                update_authority: collection_authority,
                symbol: "COOL".to_string(),
                description: Some("coolio description".to_string()),
            },
        };

        let create_metadata = Instruction {
            data: create_metadata.data(),
            program_id: libreplex_metadata::ID,
            accounts: create_metadata_accounts,
        };




        let transaction = Transaction::new_signed_with_payer(
            &[create_metadata],
            Some(&collection_authority),
            &[&context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap();



    }
}
