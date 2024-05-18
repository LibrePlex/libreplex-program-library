use anchor_lang::{Result, system_program, InstructionData, ToAccountMetas};

use libreplex_inscriptions::accounts::MakeInscriptionImmutableV3;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{transaction::Transaction, signer::Signer};

const MINIMUM_INSCRIPTION_SIZE: u32 = 8;

mod inscriptions_tests {
    use anchor_lang::{InstructionData, ToAccountMetas};

    use anchor_lang::prelude::Account;
    
    use libreplex_inscriptions::instructions::{SignerType, CreateInscriptionInputV3};
    use libreplex_inscriptions::{
        accounts::CreateInscriptionV3,
        accounts::MakeInscriptionImmutableV3,
        accounts::ResizeInscriptionV3,
        accounts::WriteToInscriptionV3,
        instructions::{
            create_inscription_v2::CreateInscriptionInput, ResizeInscriptionInput,
            WriteToInscriptionInput,
        },
        Inscription,
    };
    use libreplex_inscriptions::{constants, InscriptionSummary, InscriptionV3};
    use solana_program::account_info::AccountInfo;
    use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program};

    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn inscriptions_v3_test() {
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
            None, // processor!(libreplex_inscriptions::entry),
        );

        let mut context: ProgramTestContext = program.start_with_context().await;

        // let slot = Clock::get().unwrap().slot;

        // resize tests take a while so we need to advance slots in order to
        // avoid RpcError(DeadlineExceeded) on test execution

        let authority = context.payer.pubkey();

        let mint = Keypair::new();

        let root = Keypair::new();

        let root_2 = Keypair::new();

        let (inscription_v3, inscription_data) =
            create_inscription(&mut context, &root).await.unwrap();

        let (inscription_2_v3, inscription_data_2) =
            create_inscription(&mut context, &root_2).await.unwrap();

        let mut account_after_create = context
            .banks_client
            .get_account(inscription_v3)
            .await
            .unwrap()
            .unwrap();

        let inscription_account_info = AccountInfo::new(
            &inscription_v3,
            false,
            false,
            &mut account_after_create.lamports,
            &mut account_after_create.data,
            &account_after_create.owner,
            account_after_create.executable,
            account_after_create.rent_epoch,
        );

        let inscription_obj: Account<InscriptionV3> =
            Account::try_from(&inscription_account_info).unwrap();

        assert_eq!(inscription_obj.size, MINIMUM_INSCRIPTION_SIZE);

        // increase the size to 10MB max incrementally
        let mut applied_increases: u32 = 0;
        let max_increases = 10;
        while applied_increases < max_increases {
            context
                .banks_client
                .process_transaction(Transaction::new_signed_with_payer(
                    &[Instruction {
                        program_id: libreplex_inscriptions::id(),
                        data: libreplex_inscriptions::instruction::ResizeInscriptionV3 {
                            input: ResizeInscriptionInput {
                                change: 8192,
                                expected_start_size: MINIMUM_INSCRIPTION_SIZE
                                    + 8192 * applied_increases,
                                target_size: MINIMUM_INSCRIPTION_SIZE + 8192 * max_increases,
                            },
                        }
                        .data(),
                        accounts: ResizeInscriptionV3 {
                            authority,
                            payer: authority,
                            inscription_v3,
                            inscription_data,
                            system_program: system_program::id(),
                        }
                        .to_account_metas(None),
                    }],
                    Some(&authority),
                    &[&context.payer],
                    context.last_blockhash,
                ))
                .await
                .unwrap();
            let mut account_after_resize = context
                .banks_client
                .get_account(inscription_v3)
                .await
                .unwrap()
                .unwrap();

            let data_account_after_resize = context
                .banks_client
                .get_account(inscription_data)
                .await
                .unwrap()
                .unwrap();

            let inscription_account_info = AccountInfo::new(
                &inscription_v3,
                false,
                false,
                &mut account_after_resize.lamports,
                &mut account_after_resize.data,
                &account_after_resize.owner,
                account_after_resize.executable,
                account_after_resize.rent_epoch,
            );

            let inscription_obj: Account<InscriptionV3> =
                Account::try_from(&inscription_account_info).unwrap();

            assert_eq!(
                inscription_obj.size,
                MINIMUM_INSCRIPTION_SIZE + 8192 * (applied_increases + 1)
            );
            assert_eq!(
                inscription_obj.size as usize,
                data_account_after_resize.data.len()
            );
            assert_eq!(
                data_account_after_resize.lamports,
                context
                    .banks_client
                    .get_rent()
                    .await
                    .unwrap()
                    .minimum_balance(inscription_obj.size as usize)
            );
            applied_increases += 1;
        }
        let mut applied_decreases = 0;
        let max_decreases = 9;
        while applied_decreases < max_decreases {
            context
                .banks_client
                .process_transaction(Transaction::new_signed_with_payer(
                    &[Instruction {
                        program_id: libreplex_inscriptions::id(),
                        data: libreplex_inscriptions::instruction::ResizeInscriptionV3 {
                            input: ResizeInscriptionInput {
                                change: -8192,
                                expected_start_size: MINIMUM_INSCRIPTION_SIZE
                                    + 8192 * applied_increases
                                    - 8192 * applied_decreases,
                                target_size: MINIMUM_INSCRIPTION_SIZE + 8192 * max_increases
                                    - 8192 * max_decreases,
                            },
                        }
                        .data(),
                        accounts: ResizeInscriptionV3 {
                            authority,
                            payer: authority,
                            inscription_v3,
                            inscription_data,
                            system_program: system_program::id(),
                        }
                        .to_account_metas(None),
                    }],
                    Some(&authority),
                    &[&context.payer],
                    context.last_blockhash,
                ))
                .await
                .unwrap();

            let mut account_after_resize = context
                .banks_client
                .get_account(inscription_v3)
                .await
                .unwrap()
                .unwrap();

            let inscription_account_info = AccountInfo::new(
                &inscription_v3,
                false,
                false,
                &mut account_after_resize.lamports,
                &mut account_after_resize.data,
                &account_after_resize.owner,
                account_after_resize.executable,
                account_after_resize.rent_epoch,
            );

            let inscription_obj: Account<InscriptionV3> =
                Account::try_from(&inscription_account_info).unwrap();

            let data_account_after_resize = context
                .banks_client
                .get_account(inscription_data)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                inscription_obj.size,
                MINIMUM_INSCRIPTION_SIZE + 8192 * applied_increases
                    - 8192 * applied_decreases
                    - 8192
            );
            assert_eq!(
                inscription_obj.size as usize,
                data_account_after_resize.data.len()
            );
            assert_eq!(
                data_account_after_resize.lamports,
                std::cmp::max(
                    constants::MINIMUM_INSCRIPTION_LAMPORTS,
                    context
                        .banks_client
                        .get_rent()
                        .await
                        .unwrap()
                        .minimum_balance(inscription_obj.size as usize)
                )
            );
            applied_decreases += 1;
        }

        // test for withdrawing if lamports balance is higher than required for the space
        let mut inscription_data_account = context
            .banks_client
            .get_account(inscription_data)
            .await
            .unwrap()
            .unwrap();
        let inscription_data_account_lamports = inscription_data_account.lamports;
        inscription_data_account.lamports += 1;
        context.set_account(&inscription_data, &inscription_data_account.into());

        assert_eq!(
            context
                .banks_client
                .get_account(inscription_data)
                .await
                .unwrap()
                .unwrap()
                .lamports,
            inscription_data_account_lamports + 1
        );

        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[Instruction {
                    program_id: libreplex_inscriptions::id(),
                    data: libreplex_inscriptions::instruction::ResizeInscriptionV3 {
                        input: ResizeInscriptionInput {
                            change: 0,
                            expected_start_size: MINIMUM_INSCRIPTION_SIZE
                                + 8192 * applied_increases
                                - 8192 * applied_decreases,
                            target_size: MINIMUM_INSCRIPTION_SIZE + 8192 * max_increases
                                - 8192 * max_decreases,
                        },
                    }
                    .data(),
                    accounts: ResizeInscriptionV3 {
                        authority,
                        payer: authority,
                        inscription_v3,
                        inscription_data,
                        system_program: system_program::id(),
                    }
                    .to_account_metas(None),
                }],
                Some(&authority),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let mut account_after_resize = context
            .banks_client
            .get_account(inscription_v3)
            .await
            .unwrap()
            .unwrap();

        let inscription_account_info = AccountInfo::new(
            &inscription_v3,
            false,
            false,
            &mut account_after_resize.lamports,
            &mut account_after_resize.data,
            &account_after_resize.owner,
            account_after_resize.executable,
            account_after_resize.rent_epoch,
        );

        let inscription_obj: Account<InscriptionV3> =
            Account::try_from(&inscription_account_info).unwrap();

        let data_account_after_resize = context
            .banks_client
            .get_account(inscription_data)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            inscription_obj.size,
            MINIMUM_INSCRIPTION_SIZE + 8192 * applied_increases - 8192 * applied_decreases
        );
        assert_eq!(
            inscription_obj.size as usize,
            data_account_after_resize.data.len()
        );
        assert_eq!(
            data_account_after_resize.lamports,
            std::cmp::max(
                constants::MINIMUM_INSCRIPTION_LAMPORTS,
                context
                    .banks_client
                    .get_rent()
                    .await
                    .unwrap()
                    .minimum_balance(inscription_obj.size as usize)
            )
        );

        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[Instruction {
                    program_id: libreplex_inscriptions::id(),
                    data: libreplex_inscriptions::instruction::WriteToInscriptionV3 {
                        input: WriteToInscriptionInput {
                            data: initial_data.clone(),
                            start_pos: 0,
                            media_type: Some("image/png".to_owned()),
                            encoding_type: Some("base64".to_owned()),
                        },
                    }
                    .data(),
                    accounts: WriteToInscriptionV3 {
                        authority,
                        payer: authority,
                        inscription_v3,
                        inscription_data,
                        system_program: system_program::id(),
                    }
                    .to_account_metas(None),
                }],
                Some(&authority),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let write_to_inscription_accounts = WriteToInscriptionV3 {
            authority,
            payer: authority,
            inscription_data,
            inscription_v3,
            system_program: system_program::id(),
        };

        let write_to_inscription_input: libreplex_inscriptions::instruction::WriteToInscriptionV3 =
            libreplex_inscriptions::instruction::WriteToInscriptionV3 {
                input: WriteToInscriptionInput {
                    data: append_data.clone(),
                    start_pos: initial_data.len() as u32,
                    media_type: Some("image/svg+xml".to_owned()),
                    encoding_type: Some("base64".to_owned()),
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

        let final_account = context
            .banks_client
            .get_account(inscription_data)
            .await
            .unwrap()
            .unwrap();

        let expected_data: Vec<u8> = [&initial_data[..], &append_data[..]].concat();
        assert_eq!(
            expected_data.as_slice(),
            &final_account.data[0..(initial_data.len() + append_data.len())]
        );

        let mut inscription_account = context
            .banks_client
            .get_account(inscription_v3)
            .await
            .unwrap()
            .unwrap();

        let inscription_pubkey = inscription_v3;
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

        let inscription_obj: Account<InscriptionV3> =
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

        assert_eq!(inscription_summary_obj.last_inscription, inscription_2_v3);

        assert_eq!(
            inscription_summary_obj.last_inscriber,
            context.payer.pubkey()
        );

        assert_eq!(inscription_obj.order, 1);

        // we invert the order here and check the rank ordering afterwards

        make_inscription_immutable_v3(&mut context, 0, inscription_v3).await;

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

        assert_eq!(inscription_summary_obj.inscription_count_immutables, 1);

        // check that ranks have been updated

        let inscription_key = inscription_v3;
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

        let inscription_obj: Account<InscriptionV3> = Account::try_from(&inscription_info).unwrap();

        assert_eq!(inscription_obj.order, 1);
        assert_eq!(inscription_obj.authority, system_program::ID);

        make_inscription_immutable_v3(&mut context, 0, inscription_2_v3).await;

        let inscription_2_pubkey = inscription_2_v3;
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

        let inscription_2_obj: Account<InscriptionV3> =
            Account::try_from(&inscription_account_2_info).unwrap();

        assert_eq!(inscription_2_obj.order, 2);
        assert_eq!(inscription_2_obj.authority, system_program::ID);
    }

    async fn create_inscription(
        ctx: &mut ProgramTestContext,
        root: &Keypair,
    ) -> Result<(Pubkey, Pubkey)> {
        // let inscription_data = Keypair::new();

        let inscription_v3 = Pubkey::find_program_address(
            &["inscription_v3".as_bytes(), root.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_data = Pubkey::find_program_address(
            &["inscription_data".as_bytes(), root.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let inscription_account = CreateInscriptionV3 {
            payer: ctx.payer.pubkey(),
            signer: root.pubkey(),
            root: root.pubkey(),
            inscription_v3,
            system_program: system_program::id(),
            inscription_data,
            inscription_summary,
        };

        let create_inscription_input = libreplex_inscriptions::instruction::CreateInscriptionV3 {
            inscription_input: CreateInscriptionInputV3 {
                authority: Some(ctx.payer.pubkey()),
                signer_type: SignerType::Root,
                validation_hash: None,
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

        ctx.banks_client
            .process_transaction(create_inscription_tx)
            .await
            .unwrap();
        println!("");
        Ok((inscription_v3, inscription_data))
    }

    pub async fn make_inscription_v3_immutable(
        context: &mut ProgramTestContext,
        inscription_v3: Pubkey,
    ) {
        let make_inscription_immutable_input =
            libreplex_inscriptions::instruction::MakeInscriptionImmutableV3 {};

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let make_inscription_immutable_accounts = MakeInscriptionImmutableV3 {
            payer: context.payer.pubkey(),
            system_program: system_program::ID,
            authority: context.payer.pubkey(),
            inscription_summary,
            inscription_v3,
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


pub async fn make_inscription_immutable_v3(
    context: &mut ProgramTestContext,
    page_index: u32,
    inscription_v3: Pubkey,
) {
    let make_inscription_immutable_input =
        libreplex_inscriptions::instruction::MakeInscriptionImmutableV3 {};

    let inscription_summary =
        Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

    let make_inscription_immutable_accounts = MakeInscriptionImmutableV3 {
        payer: context.payer.pubkey(),
        system_program: system_program::ID,
        authority: context.payer.pubkey(),
        inscription_summary,
        inscription_v3,
    };

    let create_inscription_rank_ix = solana_program::instruction::Instruction {
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