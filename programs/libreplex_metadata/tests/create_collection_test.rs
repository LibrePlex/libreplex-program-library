use solana_program_test::*;


const GROUP_NAME: &str = "coolio";
const GROUP_DESCRIPTION: &str = "coolio description";
const GROUP_SYMBOL: &str = "symbol";
const GROUP_URL: &str = "https://bla.com";

const COLOR: &str = "color";
const COLOR_GREEN: &str = "green";
const COLOR_RED: &str = "red";

mod permissions {
  
    use anchor_lang::{system_program, ToAccountMetas, prelude::Account, InstructionData};
    use libreplex_metadata::{Collection, COLLECTION, instructions::CreateCollectionInput};
    use solana_program::{instruction::Instruction, pubkey::Pubkey, account_info::AccountInfo};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_collection_test() {
        let program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            None
        );

        let mut context = program.start_with_context().await;
        let collection_authority = context.payer.pubkey();
        let collection_seed_kp = Keypair::new();
        let collection = Pubkey::find_program_address(
            &[COLLECTION.as_bytes(), collection_seed_kp.pubkey().as_ref()],
            &libreplex_metadata::ID,
        )
        .0;

     
        let _aa = vec![1,2,3];

        let create_group_instruction = libreplex_metadata::instruction::CreateCollection {
            collection_input: CreateCollectionInput {
                name: GROUP_NAME.to_string(),
                symbol: GROUP_SYMBOL.to_string(),
                description: GROUP_DESCRIPTION.to_string(),
            },
        };

        let create_group_accounts = libreplex_metadata::accounts::CreateCollection {
            authority: collection_authority,
            seed: collection_seed_kp.pubkey(),
            collection,
            system_program: system_program::ID,
        }.to_account_metas(None);


        let create_group = Instruction {
            data: create_group_instruction.data(),
            program_id: libreplex_metadata::ID,
            accounts: create_group_accounts,
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

        
        let mut group_account = context.banks_client.get_account(
            collection
        ).await.unwrap().unwrap();

        let group_account_info = AccountInfo::new(
            &collection,
            false,
            false,
            &mut group_account.lamports,
            &mut group_account.data,
            &group_account.owner,
            group_account.executable,
            group_account.rent_epoch,
        );


        
        let collection: Account<Collection> = Account::try_from(&group_account_info).unwrap();

        assert_eq!(
            collection.description,
            GROUP_DESCRIPTION
        );

        assert_eq!(
            collection.name,
            GROUP_NAME
        );

        assert_eq!(
            collection.symbol,
            GROUP_SYMBOL
        );


        
        assert_eq!(
            collection.name,
            GROUP_NAME
        );


        

    }
}
