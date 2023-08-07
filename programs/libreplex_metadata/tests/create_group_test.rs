use solana_program_test::*;


const GROUP_NAME: &str = "coolio";
const GROUP_DESCRIPTION: &str = "coolio description";
const GROUP_SYMBOL: &str = "symbol";
const GROUP_URL: &str = "https://bla.com";

const COLOR: &str = "color";
const COLOR_GREEN: &str = "green";
const COLOR_RED: &str = "red";

mod permissions {
  
    use anchor_lang::{system_program, InstructionData, ToAccountMetas, prelude::Account};
    use libreplex_metadata::{AttributeType, AttributeValue, GroupInput, Group};
    use solana_program::{instruction::Instruction, pubkey::Pubkey, account_info::AccountInfo};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_group() {
        let program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            processor!(libreplex_metadata::entry),
        );

        let mut context = program.start_with_context().await;
        let collection_authority = context.payer.pubkey();
        let collection_seed_kp = Keypair::new();
        let group = Pubkey::find_program_address(
            &[b"group", collection_seed_kp.pubkey().as_ref()],
            &libreplex_metadata::ID,
        )
        .0;

     
        let aa = vec![1,2,3];

        let permitted_values = vec![
            AttributeValue::Word {
                value: COLOR_GREEN.to_string(),
            },
            AttributeValue::Word {
                value: COLOR_RED.to_string(),
            },
            AttributeValue::U32 { value: 3200000000 },
        ];

        let attribute_types = vec![ AttributeType {
            name: COLOR.to_string(),
            permitted_values,
            deleted: false,
            continued_at_index: None,
            continued_from_index: None,
        }];


        let create_group_instruction = libreplex_metadata::instruction::CreateGroup {
            group_input: GroupInput {
                name: GROUP_NAME.to_string(),
                url: GROUP_URL.to_owned(),
                symbol: GROUP_SYMBOL.to_string(),
                template_configuration: libreplex_metadata::TemplateConfiguration::None,
                attribute_types,
                royalties: None,
                permitted_signers: vec![],
                description: GROUP_DESCRIPTION.to_string(),
            },
        };

        let create_group_accounts = libreplex_metadata::accounts::CreateGroup {
            authority: collection_authority,
            seed: collection_seed_kp.pubkey(),
            group,
            system_program: system_program::ID,
        }
        .to_account_metas(None);


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
            group
        ).await.unwrap().unwrap();

        let group_account_info = AccountInfo::new(
            &group,
            false,
            false,
            &mut group_account.lamports,
            &mut group_account.data,
            &group_account.owner,
            group_account.executable,
            group_account.rent_epoch,
        );


        
        let group: Account<Group> = Account::try_from(&group_account_info).unwrap();

        assert_eq!(
            group.description,
            GROUP_DESCRIPTION
        );

        assert_eq!(
            group.name,
            GROUP_NAME
        );

        assert_eq!(
            group.symbol,
            GROUP_SYMBOL
        );


        assert_eq!(
            group.url,
            GROUP_URL
        );

        assert_eq!(
            group.name,
            GROUP_NAME
        );


        assert_eq!(
            group.attribute_types[0].name,
            COLOR
        );
        

    }
}
