use solana_program_test::*;

mod metadata_tests {
    use anchor_lang::{InstructionData, Key, ToAccountMetas};
    use libreplex::{
        accounts::AppendToOrdinal,
        accounts::CreateMetadata,
        accounts::CreateOrdinal,
        instructions::{append_to_ordinal, AppendToOrdinalInput, CreateOrdinalInput},
        Asset, CreateMetadataInput, Ordinal,
    };
    use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program, system_instruction, program::invoke, rent::Rent};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_metadata_ordinal_test() {
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
        let program = ProgramTest::new("metadata", libreplex::ID, processor!(libreplex::entry));

        let mut context = program.start_with_context().await;
        let authority = context.payer.pubkey();

        let mint = Keypair::new();
     
        let ordinal = Keypair::new();

        let ordinal_input = libreplex::instruction::CreateOrdinal {
            ordinal_input: CreateOrdinalInput {
                max_data_length: 500,
                initial_data: initial_data.clone(),
            },
        };

        let ordinal_account = CreateOrdinal {
            payer: context.payer.pubkey(),
            ordinal: ordinal.pubkey(),
            system_program: system_program::id(),
        };



        // CREATE MINT
        let rent = context.banks_client.get_rent().await.unwrap();

        let create_account_tx = Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &context.payer.pubkey(),
                &ordinal.pubkey(),
                rent.minimum_balance(Ordinal::BASE_SIZE + ordinal_input.ordinal_input.max_data_length as usize),
                
                Ordinal::BASE_SIZE as u64 + ordinal_input.ordinal_input.max_data_length as u64,
                &libreplex::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer, &ordinal],
            context.last_blockhash
        );

        
        context.banks_client.process_transaction(create_account_tx).await.unwrap();
     
        let ordinal_ix = Instruction {
            program_id: libreplex::id(),
            data: ordinal_input.data(),
            accounts: ordinal_account.to_account_metas(None),
        };

        let ordinal_tx = Transaction::new_signed_with_payer(
            &[ordinal_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );

        let result = context
            .banks_client
            .process_transaction(ordinal_tx)
            .await
            .unwrap();

        // append a bit more data to the ordinal

        let append_to_ordinal_accounts = AppendToOrdinal {
            signer: authority,
            ordinal: ordinal.pubkey(),
            system_program: system_program::id(),
        };

        let append_to_ordinal_input = libreplex::instruction::AppendToOrdinal {
            input: AppendToOrdinalInput {
                append_data: append_data.clone(),
            },
        };

        let append_to_ordinal_ix = Instruction {
            program_id: libreplex::id(),
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
            Pubkey::find_program_address(&[b"metadata", mint.pubkey().as_ref()], &libreplex::ID).0;

        let permissions = Pubkey::find_program_address(
            &[b"permissions", metadata.as_ref(), authority.as_ref()],
            &libreplex::ID,
        )
        .0;

        let metadata_account = CreateMetadata {
            signer: authority,
            permissions,
            metadata,
            mint: mint.pubkey(),
            system_program: system_program::id(),
        };

        let name = "ordinal".to_owned();
        let symbol = "ORD".to_owned();
        let asset = Asset::Ordinal {
            account_id: ordinal.pubkey(),
        };

        let metadata_input = libreplex::instruction::CreateMetadata {
            metadata_input: CreateMetadataInput {
                name,
                symbol,
                asset,
                description: None,
            },
        };

        let metadata_ix = Instruction {
            data: metadata_input.data(),
            program_id: libreplex::ID,
            accounts: metadata_account.to_account_metas(None),
        };

        let create_metadata_tx = Transaction::new_signed_with_payer(
            &[metadata_ix],
            Some(&authority),
            &[&context.payer, &mint],
            context.last_blockhash,
        );

        let result = context
            .banks_client
            .process_transaction(create_metadata_tx)
            .await
            .unwrap();

        let final_account = context
            .banks_client
            .get_account(ordinal.pubkey())
            .await
            .unwrap();

        let expected_data: Vec<u8> = [&initial_data[..], &append_data[..]].concat();

        match final_account {
            Some(x) => {
                let l = u32::from_le_bytes(x.data[40..44].try_into().unwrap());
                let endidx = 48 + l;
                assert_eq!(
                    expected_data.as_slice(),
                    // first 48 bytes are
                    // 8 - discriminant
                    // 32 - authority
                    // 4 - data length (current)
                    // 4 - data length (max)
                    &x.data[48..endidx as usize]
                );
            }
            None => {
                assert_eq!(true, false);
            }
        }
    }
}
