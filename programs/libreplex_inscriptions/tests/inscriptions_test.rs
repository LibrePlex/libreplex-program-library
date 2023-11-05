use std::{cell::RefCell, rc::Rc};

use anchor_lang::Result;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use solana_program_test::*;

mod inscriptions_tests {
    use anchor_lang::{InstructionData, Key, ToAccountInfo, ToAccountMetas};

    use anchor_lang::prelude::Account;
    use libreplex_inscriptions::accounts::CreateInscriptionRank;
    use libreplex_inscriptions::instructions::{CreateInscriptionRankInput, SignerType};
    use libreplex_inscriptions::{
        accounts::CreateInscription,
        accounts::MakeInscriptionImmutable,
        accounts::WriteToInscription,
        instructions::{create_inscription::CreateInscriptionInput, WriteToInscriptionInput},
        Inscription,
    };
    use libreplex_inscriptions::{InscriptionRankPage, InscriptionSummary};
    use solana_program::account_info::AccountInfo;
    use solana_program::{
        instruction::Instruction, pubkey::Pubkey, system_instruction, system_program,
    };

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
        let program = ProgramTest::new(
            "libreplex_inscriptions",
            libreplex_inscriptions::ID,
            processor!(libreplex_inscriptions::entry),
        );

        let mut context: ProgramTestContext = program.start_with_context().await;
        let authority = context.payer.pubkey();

        let mint = Keypair::new();

        let root = Keypair::new();

        let root_2 = Keypair::new();

    
        let inscription_ranks_current_page =
            create_inscription_rank_page(&mut context, 0).await.unwrap();

        let inscription_ranks_next_page =
            create_inscription_rank_page(&mut context, 1).await.unwrap();

        println!(
            "Current page: {}, next page: {}",
            inscription_ranks_current_page, inscription_ranks_next_page
        );

        let (inscription, inscription_data) = create_inscription(
            &mut context,
            &root,
            (initial_data.len() + append_data.len()) as u32,
            0,
        )
        .await
        .unwrap();

        println!("created inscription 1");

        let (inscription_2, inscription_data_2) = create_inscription(
            &mut context,
            &root_2,
            (initial_data.len() + append_data.len()) as u32,
            0,
        )
        .await
        .unwrap();

        let mut account_after_create = context
            .banks_client
            .get_account(inscription)
            .await
            .unwrap()
            .unwrap();


        let inscription_account_info = AccountInfo::new(
            &inscription,
            false,
            false,
            &mut account_after_create.lamports,
            &mut account_after_create.data,
            &account_after_create.owner,
            account_after_create.executable,
            account_after_create.rent_epoch,
        );

        let inscription_obj: Account<Inscription> =
            Account::try_from(&inscription_account_info).unwrap();


        assert_eq!(
            inscription_obj.size,
            (initial_data.len() + append_data.len()) as u32
        );

        // WRITE SOME INITIAL DATA AT POS 0
        let append_to_inscription_accounts = WriteToInscription {
            authority,
            inscription: inscription,
            inscription_data: inscription_data.pubkey(),
            system_program: system_program::id(),
        };

        let append_to_inscription_input: libreplex_inscriptions::instruction::WriteToInscription =
            libreplex_inscriptions::instruction::WriteToInscription {
                input: WriteToInscriptionInput {
                    data: initial_data.clone(),
                    start_pos: 0,
                },
            };

        let append_to_inscription_ix = Instruction {
            program_id: libreplex_inscriptions::id(),
            data: append_to_inscription_input.data(),
            accounts: append_to_inscription_accounts.to_account_metas(None),
        };

        let append_to_inscription_tx = Transaction::new_signed_with_payer(
            &[append_to_inscription_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(append_to_inscription_tx)
            .await
            .unwrap();

        let write_to_inscription_accounts = WriteToInscription {
            authority,
            inscription_data: inscription_data.pubkey(),
            inscription: inscription,
            system_program: system_program::id(),
        };

        let write_to_inscription_input: libreplex_inscriptions::instruction::WriteToInscription =
            libreplex_inscriptions::instruction::WriteToInscription {
                input: WriteToInscriptionInput {
                    data: append_data.clone(),
                    start_pos: initial_data.len() as u32,
                },
            };

        let write_to_inscription_ix = Instruction {
            program_id: libreplex_inscriptions::id(),
            data: write_to_inscription_input.data(),
            accounts: write_to_inscription_accounts.to_account_metas(None),
        };

        let write_to_inscription_tx = Transaction::new_signed_with_payer(
            &[write_to_inscription_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(write_to_inscription_tx)
            .await
            .unwrap();

        let metadata = Pubkey::find_program_address(
            &[b"metadata", mint.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let _permissions = Pubkey::find_program_address(
            &[b"permissions", metadata.as_ref(), authority.as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let mut final_account = context
            .banks_client
            .get_account(inscription_data.pubkey())
            .await
            .unwrap().unwrap();

        let expected_data: Vec<u8> = [&initial_data[..], &append_data[..]].concat();
        assert_eq!(expected_data.as_slice(), &final_account.data[0..(initial_data.len()+append_data.len())]);

        let mut inscription_account = context
            .banks_client
            .get_account(inscription)
            .await
            .unwrap().unwrap();

       
              
        let inscription_pubkey = inscription;
        let inscription_account_info = AccountInfo::new(
            &inscription_pubkey,
            false,
            false,
            &mut inscription_account.lamports,
            &mut inscription_account.data,
            &inscription_account.owner,
            inscription_account.executable,
            inscription_account.rent_epoch,
        );

        let inscription_obj: Account<Inscription> =
            Account::try_from(&inscription_account_info).unwrap();

        assert_eq!(inscription_obj.root, root.pubkey());
    
        
        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let mut account_summary = context
            .banks_client
            .get_account(inscription_summary)
            .await
            .unwrap()
            .unwrap();

        let inscription_summary_info = AccountInfo::new(
            &inscription_summary,
            false,
            false,
            &mut account_summary.lamports,
            &mut account_summary.data,
            &account_summary.owner,
            account_summary.executable,
            account_summary.rent_epoch,
        );

        let inscription_summary_obj: Account<InscriptionSummary> =
            Account::try_from(&inscription_summary_info).unwrap();

        assert_eq!(inscription_summary_obj.inscription_count_total, 2);

        assert_eq!(inscription_summary_obj.inscription_count_immutables, 0);

        assert_eq!(
            inscription_summary_obj.last_inscription,
            inscription_2
        );

        assert_eq!(
            inscription_summary_obj.last_inscriber,
            context.payer.pubkey()
        );

        let mut inscription_ranks_current_page_account = context
            .banks_client
            .get_account(inscription_ranks_current_page)
            .await
            .unwrap()
            .unwrap();

        let inscription_ranks_current_page_info = AccountInfo::new(
            &inscription_ranks_current_page,
            false,
            false,
            &mut inscription_ranks_current_page_account.lamports,
            &mut inscription_ranks_current_page_account.data,
            &inscription_ranks_current_page_account.owner,
            inscription_ranks_current_page_account.executable,
            inscription_ranks_current_page_account.rent_epoch,
        );

        let inscription_ranks_current_page_obj: Account<InscriptionRankPage> =
            Account::try_from(&inscription_ranks_current_page_info).unwrap();
        let account_info = inscription_ranks_current_page_obj.to_account_info();
        let inscription_slice: Vec<Pubkey> =
            InscriptionRankPage::get_inscriptions(&account_info.data.borrow_mut(), 0, 2).collect();

        // nothing has been made immutable yet, so inscription slice should have length = 0
        assert_eq!(inscription_slice.len(), 2);

        assert_eq!(inscription_obj.rank, 1);

        // we invert the order here and check the rank ordering afterwards
        make_inscription_immutable(&mut context, 0, inscription_2).await;

        make_inscription_immutable(&mut context, 0, inscription).await;

        let mut account_summary = context
            .banks_client
            .get_account(inscription_summary)
            .await
            .unwrap()
            .unwrap();

        let inscription_summary_info = AccountInfo::new(
            &inscription_summary,
            false,
            false,
            &mut account_summary.lamports,
            &mut account_summary.data,
            &account_summary.owner,
            account_summary.executable,
            account_summary.rent_epoch,
        );

        let inscription_summary_obj: Account<InscriptionSummary> =
            Account::try_from(&inscription_summary_info).unwrap();

        assert_eq!(inscription_summary_obj.inscription_count_total, 2);

        assert_eq!(inscription_summary_obj.inscription_count_immutables, 2);

        // check that ranks have been updated

        let inscription_key = inscription;
        let mut inscription_account = context
            .banks_client
            .get_account(inscription_key)
            .await
            .unwrap()
            .unwrap();

        let inscription_info = AccountInfo::new(
            &inscription_key,
            false,
            false,
            &mut inscription_account.lamports,
            &mut inscription_account.data,
            &inscription_account.owner,
            inscription_account.executable,
            inscription_account.rent_epoch,
        );

        let inscription_obj: Account<Inscription> = Account::try_from(&inscription_info).unwrap();

      

        let inscription_2_pubkey = inscription_2;
        let mut inscription_account_2 = context
            .banks_client
            .get_account(inscription_2_pubkey)
            .await
            .unwrap()
            .unwrap();

        let inscription_account_2_info = AccountInfo::new(
            &inscription_2_pubkey,
            false,
            false,
            &mut inscription_account_2.lamports,
            &mut inscription_account_2.data,
            &inscription_account_2.owner,
            inscription_account_2.executable,
            inscription_account_2.rent_epoch,
        );

        let inscription_2_obj: Account<Inscription> =
            Account::try_from(&inscription_account_2_info).unwrap();

        assert_eq!(inscription_2_obj.rank, 2);
    }

    async fn create_inscription(
        ctx: &mut ProgramTestContext,
        root: &Keypair,
        max_data_length: u32,
        current_page_index: u32,
    ) -> Result<(
        Pubkey,
        solana_sdk::signature::Keypair,
    )> {
     
        let inscription_data = Keypair::new();

        let inscription = Pubkey::find_program_address(
            &["inscription".as_bytes(), root.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let rent = ctx.banks_client.get_rent().await.unwrap();

        let create_inscription_data_account_tx = Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &ctx.payer.pubkey(),
                &inscription_data.pubkey(),
                rent.minimum_balance(max_data_length as usize),
                max_data_length as u64,
                &libreplex_inscriptions::id(),
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &inscription_data],
            ctx.last_blockhash,
        );

        

        ctx
            .banks_client
            .process_transaction(create_inscription_data_account_tx)
            .await
            .unwrap();
        println!("Created inscription data");


        let inscription_ranks_current_page =
            create_inscription_rank_page(ctx, 0).await.unwrap();

        let inscription_ranks_next_page =
            create_inscription_rank_page(ctx, 1).await.unwrap();


        let inscription_account = CreateInscription {
            inscription_summary,
            inscription_ranks_current_page,
            inscription_ranks_next_page,
            payer: ctx.payer.pubkey(),
            signer: root.pubkey(),
            root: root.pubkey(),
            inscription,
            system_program: system_program::id(),
            inscription_data: inscription_data.pubkey(),
        };

        let create_inscription_input = libreplex_inscriptions::instruction::CreateInscription {
            inscription_input: CreateInscriptionInput {
                max_data_length,
                authority: Some(ctx.payer.pubkey()),
                current_rank_page: current_page_index as u32,
                signer_type: SignerType::Root
            },
        };

        let create_inscription_ix = Instruction {
            program_id: libreplex_inscriptions::id(),
            data: create_inscription_input.data(),
            accounts: inscription_account.to_account_metas(None),
        };

        println!("Creating tx");
        let create_inscription_tx = Transaction::new_signed_with_payer(
            &[create_inscription_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &root],
            ctx.last_blockhash,
        );
        println!("Executing tx");

        ctx
            .banks_client
            .process_transaction(create_inscription_tx)
            .await
            .unwrap();
        println!("");
        Ok((inscription, inscription_data))
    }

    pub async fn create_inscription_rank_page(
        context: &mut ProgramTestContext,
        page_index: u32,
    ) -> Result<Pubkey> {
        let page = Pubkey::find_program_address(
            &[
                "inscription_rank".as_bytes(),
                &(page_index as u32).to_le_bytes(),
            ],
            &libreplex_inscriptions::id(),
        )
        .0;

        let create_inscription_rank_input =
            libreplex_inscriptions::instruction::CreateInscriptionRankPage {
                input: CreateInscriptionRankInput { page_index },
            };

        let create_inscription_rank_accounts = CreateInscriptionRank {
            payer: context.payer.pubkey(),
            page,
            system_program: system_program::id(),
        };

        let create_inscription_rank_ix = Instruction {
            program_id: libreplex_inscriptions::id(),
            data: create_inscription_rank_input.data(),
            accounts: create_inscription_rank_accounts.to_account_metas(None),
        };

        let create_inscription_tx = Transaction::new_signed_with_payer(
            &[create_inscription_rank_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(create_inscription_tx)
            .await
            .unwrap();
        Ok(page)
    }

    pub async fn make_inscription_immutable(
        context: &mut ProgramTestContext,
        page_index: u32,
        inscription: Pubkey,
    ) {
     
        let make_inscription_immutable_input =
            libreplex_inscriptions::instruction::MakeInscriptionImmutable {
               
            };

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let make_inscription_immutable_accounts = MakeInscriptionImmutable {
            payer: context.payer.pubkey(),
            system_program: system_program::ID,
            authority: context.payer.pubkey(),
            inscription_summary,
            inscription,
        };

        let create_inscription_rank_ix = Instruction {
            program_id: libreplex_inscriptions::id(),
            data: make_inscription_immutable_input.data(),
            accounts: make_inscription_immutable_accounts.to_account_metas(None),
        };

        let create_inscription_tx = Transaction::new_signed_with_payer(
            &[create_inscription_rank_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(create_inscription_tx)
            .await
            .unwrap();
    }
}
