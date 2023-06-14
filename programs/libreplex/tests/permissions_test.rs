
use solana_program_test::*;

mod permissions {
    use anchor_lang::{InstructionData, system_program, ToAccountMetas};
    use libreplex::{GroupInput, BaseUrlConfiguration};
    use solana_program::{instruction::Instruction, pubkey::Pubkey};
    use solana_sdk::{transaction::Transaction, signer::Signer, signature::Keypair};

    use super::*;
    #[tokio::test]
    async fn create_metadata_delegated() {
        let program = ProgramTest::new("libreplex", libreplex::ID, processor!(libreplex::entry));
    
        let mut context =  program.start_with_context().await;
        let collection_authority = context.payer.pubkey();
        let collection_seed_kp = Keypair::new();
        let collection = Pubkey::find_program_address(&[b"group", collection_seed_kp.pubkey().as_ref()], &libreplex::ID).0;
        let collection_authority_permissions = Pubkey::find_program_address(&[b"permissions", collection.as_ref(), collection_authority.as_ref()], &libreplex::ID).0;

  
        let create_collection_accounts = libreplex::accounts::CreateGroup {
            authority: collection_authority,
            seed: collection_seed_kp.pubkey(),
            group: collection,
            system_program: system_program::ID,
            permissions: collection_authority_permissions
        }.to_account_metas(None);


        let base_url_configuration = None;

        let create_collection = libreplex::instruction::CreateGroup {
            group_input: GroupInput {
                    // collection_url: "COOLIO.COM".to_string(),
                    name: "COOLIO COLLECTION".to_string(),
                    url: "https://collection-url.com".to_owned(),
                    symbol: "COOL".to_string(),
                    metadata_render_mode: libreplex::MetadataRenderMode::Url { base_url_configuration },
                    attribute_types: vec![],
                    royalties: None,
                    permitted_signers: vec![],
                    description: "coolio description".to_string()
            }
        };

        let create_ix = Instruction {
            data: create_collection.data(),
            program_id: libreplex::ID,
            accounts: create_collection_accounts
        };
        

        let transaction = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&collection_authority),
            &[&context.payer],
            context.last_blockhash,
        );    


        context
        .banks_client
        .process_transaction(transaction)
        .await.unwrap();
    }
}