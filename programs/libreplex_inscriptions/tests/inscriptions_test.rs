use solana_program_test::*;

mod metadata_tests {
    use anchor_lang::{InstructionData,  ToAccountMetas};
    
    use inscriptions::{
        accounts::AppendToInscription,
        accounts::CreateInscription,
        instructions::{AppendToInscriptionInput, CreateInscriptionInput}, Inscription,
    };
    use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program, system_instruction, program::invoke, rent::Rent};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn inscriptions_test() {
        let initial_data = vec![
            66, 77, 246, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 40, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 1,
            0, 24, 0, 0, 0, 0, 0, 192, 0, 0, 0, 232, 3, 0, 0, 232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            184, 184, 56, 112, 80, 0, 96, 232, 104, 128, 128, 16, 32, 24, 248, 144, 176, 128, 56,
            72, 152, 64, 224, 240, 184, 136, 56, 16, 48, 0, 16, 216, 104, 192, 128, 16, 80, 40,
            248, 192, 208, 128, 232, 120, 152, 192, 32, 240, 184, 224, 56, 192, 64, 0, 24, 160,
            104, 0, 0, 16, 24, 96, 248, 80, 192, 128, 224, 32, 152, 64, 128, 240, 184, 152, 56, 48,
            144, 0, 136, 136, 104, 192, 128, 16, 24, 120, 248, 208, 112, 128, 112, 104, 152, 0, 96,
            240, 184, 24, 56, 48, 144, 0, 216, 8, 104, 128, 128,
        ];
        let append_data = vec![
            16, 112, 248, 248, 128, 112, 128, 144, 232, 152, 0, 96, 240, 184, 88, 56, 176, 16, 0,
            136, 200, 104, 160, 128, 16, 56, 56, 248, 176, 240, 128, 208, 168, 152, 0, 96, 240,
            184, 0, 56, 0, 0, 0, 200, 0, 104, 224, 0, 16, 56, 0, 248, 64, 0, 128, 248, 0, 152, 64,
            0, 240, 184, 120, 56, 240, 208, 0, 40, 40, 104, 96, 128, 16, 152, 216, 248, 208, 48,
            128, 8, 136, 152, 64, 224, 240,
        ];
        let program = ProgramTest::new("inscriptions", inscriptions::ID, processor!(inscriptions::entry));

        let mut context = program.start_with_context().await;
        let authority = context.payer.pubkey();

        let mint = Keypair::new();

        let root = Keypair::new();
     
        let inscription = Keypair::new();

        let create_ordinal_input = inscriptions::instruction::CreateInscription {
            ordinal_input: CreateInscriptionInput {
                max_data_length: 500,
                initial_data: initial_data.clone(),
            },
        };

        let ordinal_account = CreateInscription {
            payer: context.payer.pubkey(),
            root: root.pubkey(),
            ordinal: inscription.pubkey(),
            system_program: system_program::id(),
        };


        

        // CREATE MINT
        let rent = context.banks_client.get_rent().await.unwrap();

        let create_account_tx = Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &context.payer.pubkey(),
                &inscription.pubkey(),
                rent.minimum_balance(Inscription::BASE_SIZE + create_ordinal_input.ordinal_input.max_data_length as usize),
                
                Inscription::BASE_SIZE as u64 + create_ordinal_input.ordinal_input.max_data_length as u64,
                &inscriptions::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer, &inscription],
            context.last_blockhash
        );
        
        
        context.banks_client.process_transaction(create_account_tx).await.unwrap();
     
        let create_ordinal_ix = Instruction {
            program_id: inscriptions::id(),
            data: create_ordinal_input.data(),
            accounts: ordinal_account.to_account_metas(None),
        };

        let create_ordinal_tx = Transaction::new_signed_with_payer(
            &[create_ordinal_ix],
            Some(&authority),
            &[&context.payer, &root],
            context.last_blockhash,
        );
        
        println!("Creating ordinal");
        
        let result = context
            .banks_client
            .process_transaction(create_ordinal_tx)
            .await
            .unwrap();

        // append a bit more data to the ordinal

        let append_to_ordinal_accounts = AppendToInscription {
            signer: authority,
            inscription: inscription.pubkey(),
            system_program: system_program::id(),
        };

        let append_to_ordinal_input: inscriptions::instruction::AppendToInscription = inscriptions::instruction::AppendToInscription {
            input: AppendToInscriptionInput {
                append_data: append_data.clone(),
            },
        };

        let append_to_ordinal_ix = Instruction {
            program_id: inscriptions::id(),
            data: append_to_ordinal_input.data(),
            accounts: append_to_ordinal_accounts.to_account_metas(None),
        };

        let append_to_ordinal_tx = Transaction::new_signed_with_payer(
            &[append_to_ordinal_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );

        let result = context
            .banks_client
            .process_transaction(append_to_ordinal_tx)
            .await
            .unwrap();

        let metadata =
            Pubkey::find_program_address(&[b"metadata", mint.pubkey().as_ref()], &inscriptions::ID).0;

        let permissions = Pubkey::find_program_address(
            &[b"permissions", metadata.as_ref(), authority.as_ref()],
            &inscriptions::ID,
        )
        .0;

        

        let final_account = context
            .banks_client
            .get_account(inscription.pubkey())
            .await
            .unwrap();

        let expected_data: Vec<u8> = [&initial_data[..], &append_data[..]].concat();

        match final_account {
            Some(x) => {
                let l = u32::from_le_bytes(x.data[72..76].try_into().unwrap());

                println!("{:?}",l);
                let endidx = 80 + l;
                assert_eq!(
                    expected_data.as_slice(),
                    // first 80 bytes are
                    // 8 - discriminant
                    // 32 - authority
                    // 32 - root
                    // 4 - data length (current)
                    // 4 - data length (max)
                    &x.data[80..endidx as usize]
                );
            }
            None => {
                assert_eq!(true, false);
            }
        }
    }
}
