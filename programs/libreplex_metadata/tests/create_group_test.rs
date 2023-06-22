use solana_program_test::*;

mod permissions {
    use anchor_lang::{system_program, ToAccountMetas, InstructionData};
    use libreplex_metadata::GroupInput;
    use solana_program::{pubkey::Pubkey, instruction::Instruction};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_metadata_delegated() {
        let program = ProgramTest::new("libreplex_metadata", libreplex_metadata::ID, 
        processor!(libreplex_metadata::entry));

        let mut context = program.start_with_context().await;
        let collection_authority = context.payer.pubkey();
        let collection_seed_kp = Keypair::new();
        let collection = Pubkey::find_program_address(
            &[b"group", collection_seed_kp.pubkey().as_ref()],
            &libreplex_metadata::ID,
        )
        .0;
      

        let create_collection_accounts = libreplex_metadata::accounts::CreateGroup {
            authority: collection_authority,
            seed: collection_seed_kp.pubkey(),
            group: collection,
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        let create_collection = libreplex_metadata::instruction::CreateGroup {
            collection_input: GroupInput {
                // collection_url: "COOLIO.COM".to_string(),
                name: "COOLIO COLLECTION".to_string(),
                url: "https://collection-url.com".to_owned(),
                symbol: "COOL".to_string(),
                template_configuration: libreplex_metadata::TemplateConfiguration::None,
                
                // libreplex_metadata::TemplateConfiguration::Template {
                //     name: "bla bla #{{name}}".to_owned(),
                //     image_url: "http://doodoo.com/mycollection/{{image_url}}".to_owned(),
                //     description: "bla bla {{description}}".to_owned(),
                // },
                attribute_types: vec![],
                royalties: None,
                permitted_signers: vec![],
                description: "coolio description".to_string(),
            },
        };

        let create_group = Instruction {
            data: create_collection.data(),
            program_id: libreplex_metadata::ID,
            accounts: create_collection_accounts,
        };




        let transaction = Transaction::new_signed_with_payer(
            &[create_group],
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
