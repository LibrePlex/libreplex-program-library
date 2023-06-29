
use solana_program_test::*;

mod permissions {
    use anchor_lang::{InstructionData, system_program, ToAccountMetas, Key};
    use libreplex_creator::{accounts, instruction, instructions::CreateCreatorInput};
    use libreplex_metadata::{accounts as libreaccounts, instruction as libreinstruction, GroupInput};
    use libreplex_metadata::GROUP;
    use solana_program::{instruction::Instruction, pubkey::Pubkey};
    use solana_sdk::{transaction::Transaction, signer::Signer, signature::Keypair};

    use super::*;
    #[tokio::test]
    async fn create_creator() {
        let mut program = ProgramTest::new("creator", libreplex_creator::ID, processor!(libreplex_creator::entry));
        
        program.add_program("libreplex", libreplex_metadata::ID, processor!(libreplex_metadata::entry));
        
        let mut context =  program.start_with_context().await;
        let authority = context.payer.pubkey();
        
        let group_seed = Keypair::new();
        
        let group 
            = Pubkey::find_program_address(&[GROUP.as_ref(), group_seed.pubkey().as_ref()], &libreplex_metadata::ID).0;


        let group_permissions = Pubkey::find_program_address(&[b"permissions", group.as_ref(), authority.as_ref()], &libreplex_metadata::ID).0;

        let create_group_accounts = libreaccounts::CreateGroup {
            authority,
            group,
            seed: group_seed.pubkey(),
            system_program: system_program::ID
        }.to_account_metas(None);

        let create_group_data = libreinstruction::CreateGroup {
            group_input: GroupInput {
                name: "".to_owned(),
                symbol: "".to_owned(),
                url: "".to_owned(),
                description: "".to_owned(),
                royalties: None,
                attribute_types: vec![],
                permitted_signers: vec![],
                template_configuration: libreplex_metadata::TemplateConfiguration::None
            }
        };

        let create_group_ix =  Instruction {
            data: create_group_data.data(),
            program_id: libreplex_metadata::ID,
            accounts: create_group_accounts
        };

        let create_group_tx = Transaction::new_signed_with_payer(
            &[create_group_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );  

        context
        .banks_client
        .process_transaction(create_group_tx)
        .await.unwrap();

  
        // let creator_seed = Keypair::new();
        // let creator = Pubkey::find_program_address(&[b"creator", creator_seed.pubkey().as_ref()], &librecreator::ID).0;
        // let permissions = Pubkey::find_program_address(&[b"permissions", creator.as_ref(), authority.as_ref()], &librecreator::ID).0;

        // let create_collection_accounts = accounts::CreateCreator {
        //     signer: authority,
        //     permissions        ,
        //     creator,
        //     group,
        //     system_program: system_program::ID,  
        // }.to_account_metas(None);

        // let creator_seed = Keypair::new();

        // let data = instruction::CreateCreator {
        //     creator_input: CreateCreatorInput {
        //         max_mints: 1000,
        //         seed: creator_seed.pubkey(),
        //         phases: vec![]
        //     }
        // };

        // let ix = Instruction {
        //     data: data.data(),
        //     program_id: librecreator::ID,
        //     accounts: create_collection_accounts
        // };

        // let create_creator_tx = Transaction::new_signed_with_payer(
        //     &[ix],
        //     Some(&authority),
        //     &[&context.payer],
        //     context.last_blockhash,
        // );   
        // context
        // .banks_client
        // .process_transaction(create_creator_tx)
        // .await.unwrap();
    }
}