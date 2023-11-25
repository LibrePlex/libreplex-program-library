use anchor_lang::Result;
use anchor_spl::token::Mint as SplMint;
use solana_program_test::*;

use anchor_lang::prelude::*;

mod legacy_inscribers {
    use anchor_lang::{system_program, InstructionData, ToAccountMetas};
    use anchor_spl::{
        associated_token::get_associated_token_address_with_program_id,
        token::{spl_token, ID},
    };
    use libreplex_inscriptions::{
        accounts::CreateInscriptionRank, instructions::CreateInscriptionRankInput,
    };

    
    use mpl_token_metadata::{
        instructions::CreateV1Builder,
        types::TokenStandard,
    };
    use solana_program::{instruction::Instruction, system_instruction};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_metadata_inscription() {
        let mut program = ProgramTest::new(
            "libreplex_inscriptions",
            libreplex_inscriptions::ID,
            None
        );

        program.prefer_bpf(true);
        program.set_transaction_account_lock_limit(10_000_000_000);

        program.set_compute_max_units(5_000_000);
        let mut context: ProgramTestContext = program.start_with_context().await; //program.start_with_context().await;

        let mint = Keypair::new();

        // CREATE MINT

        let rent = context.banks_client.get_rent().await.unwrap();

        println!("Create inscription rank page (current)");

        let inscription_ranks_current_page = create_inscription_rank_and_wait(&mut context, 0)
            .await
            .unwrap();

        let inscription_ranks_next_page = create_inscription_rank_and_wait(&mut context, 1)
            .await
            .unwrap();

        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[
                    system_instruction::create_account(
                        &context.payer.pubkey(),
                        &mint.pubkey(),
                        rent.minimum_balance(SplMint::LEN),
                        SplMint::LEN as u64,
                        &ID,
                    ),
                    spl_token::instruction::initialize_mint2(
                        &spl_token::ID,
                        &mint.pubkey(),
                        &context.payer.pubkey(),
                        Some(&context.payer.pubkey()),
                        0,
                    )
                    .unwrap(),
                ],
                Some(&context.payer.pubkey()),
                &[&context.payer, &mint],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        // spl_associated_token_account::create_associated_token_account(funding_address, wallet_address, token_mint_address)
        println!("Create associated token account");
        // create associated token account
        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[
                    spl_associated_token_account::instruction::create_associated_token_account(
                        &context.payer.pubkey(),
                        &context.payer.pubkey(),
                        &mint.pubkey(),
                        &spl_token::ID,
                    ),
                ],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let token_account = get_associated_token_address_with_program_id(
            &context.payer.pubkey().key(),
            &mint.pubkey(),
            &spl_token::ID,
        );

        println!("Mint to token account");

        // context.warp_to_slot(first_normal_slot + 200).unwrap();

        // mint to token account
        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[spl_token::instruction::mint_to(
                    &spl_token::id(),
                    &mint.pubkey(),
                    &token_account,
                    &context.payer.pubkey(),
                    &[&context.payer.pubkey()],
                    1,
                )
                .unwrap()],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        // create legacy metadata

        let legacy_metadata = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                &mpl_token_metadata::ID.as_ref(),
                mint.pubkey().as_ref(),
            ],
            &mpl_token_metadata::ID,
        )
        .0;

        let master_edition = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                mpl_token_metadata::ID.as_ref(),
                mint.pubkey().as_ref(),
                "edition".as_bytes(),
            ],
            &mpl_token_metadata::ID,
        )
        .0;

        // println!("Warping to slot 300");
        // context.warp_to_slot(first_normal_slot + 300).unwrap();

        let last_blockhash = context
            .banks_client
            .get_new_latest_blockhash(&context.last_blockhash)
            .await
            .unwrap();


        /* 
            
            FOR SOME REASON ALL CALLS TO METADATA PROGRAM FAIL WITH 
        
            thread 'legacy_inscribers::create_metadata_inscription' panicked at 
            'called `Result::unwrap()` on an `Err` value: RpcError(DeadlineExceeded)
    
            NEEDS FURTHER INVESTIGATION. PRs invited!!!!!
         

        let create_metadata_ix = CreateV1Builder::new()
            .metadata(legacy_metadata)
            .mint(mint.pubkey(), true)
            .authority(context.payer.pubkey())
            .payer(context.payer.pubkey())
            .update_authority(context.payer.pubkey(), true)
            .master_edition(Some(master_edition))
            .primary_sale_happened(false)
            .token_standard(TokenStandard::NonFungible)
            .seller_fee_basis_points(0)
            .is_mutable(true)
            .name("bla".to_owned())
            .symbol("bla".to_owned())
            .uri("bla".to_owned())
            .decimals(0)
            .instruction();
        // Create metadata
        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[create_metadata_ix],
                Some(&context.payer.pubkey()),
                &[&context.payer, &mint],
                last_blockhash,
            ))
            .await
            .unwrap();

        */  
     
        // let inscription = Pubkey::find_program_address(
        //     &["inscription".as_bytes(), mint.pubkey().as_ref()],
        //     &libreplex_inscriptions::ID,
        // )
        // .0;

        // let inscription_data = Keypair::new();

        // let initialise_inscription_data_tx = system_instruction::create_account(
        //     &context.payer.pubkey(),
        //     &inscription_data.pubkey(),
        //     rent.minimum_balance(Inscription::SIZE + 1024_usize),
        //     Inscription::SIZE as u64 + 1024_u64,
        //     &libreplex_inscriptions::id(),
        // );

        // let inscription_summary =
        //     Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        // let legacy_inscription = Pubkey::find_program_address(
        //     &["inscription".as_bytes(), mint.pubkey().as_ref()],
        //     &libreplex_inscriptions::ID,
        // )
        // .0;

        // let inscribe_legacy_metadata_ix = Instruction {
        //     data: libreplex_legacy_inscribers::instruction::InscribeLegacyMetadata {
        //         input: libreplex_legacy_inscribers::instructions::InscribeLegacyInput {
        //             legacy_type: LegacyType::MetaplexMint,
        //         },
        //     }
        //     .data(),
        //     program_id: libreplex_legacy_inscribers::ID,
        //     accounts: libreplex_legacy_inscribers::accounts::InscribeLegacyMetadata {
        //         authority: context.payer.pubkey(),
        //         mint: mint.pubkey(),
        //         inscription,
        //         inscription_data: inscription_data.pubkey(),
        //         inscription_summary,
        //         inscription_ranks_current_page,
        //         inscription_ranks_next_page,
        //         token_account,
        //         legacy_inscription,
        //         legacy_mint: mint.pubkey(),
        //         token_program: spl_token::ID,
        //         system_program: system_program::ID,
        //         inscriptions_program: libreplex_inscriptions::ID,
        //     }
        //     .to_account_metas(None),
        // };

        // context
        //     .banks_client
        //     .process_transaction(Transaction::new_signed_with_payer(
        //         &[initialise_inscription_data_tx, inscribe_legacy_metadata_ix],
        //         Some(&context.payer.pubkey()),
        //         &[&context.payer, &inscription_data],
        //         context.last_blockhash,
        //     ))
        //     .await;

        // let inscription = Pubkey::find_program_address(
        //     &["inscription".as_bytes(), mint.pubkey().as_ref()],
        //     &libreplex_inscriptions::ID,
        // )
        // .0;

        // let inscription_data = Keypair::new();

        // let rent = context.banks_client.get_rent().await.unwrap();

        // let initialise_inscription_data_tx = system_instruction::create_account(
        //     &context.payer.pubkey(),
        //     &inscription_data.pubkey(),
        //     rent.minimum_balance(Inscription::SIZE + 1024_usize),
        //     Inscription::SIZE as u64 + 1024_u64,
        //     &libreplex_inscriptions::id(),
        // );

        // let create_metadata_accounts = InscribeLegacyMetadata {
        //     authority: collection_authority,
        //     token_account,
        //     metadata: metadata.key(),
        //     mint: mint.pubkey(),
        //     inscription_summary,
        //     inscription_ranks_current_page,
        //     inscription_ranks_next_page,
        //     inscription,
        //     inscription_data: inscription_data.pubkey(),
        //     inscriptions_program: libreplex_inscriptions::ID,
        //     system_program: system_program::ID,
        // }
        // .to_account_metas(None);

        // let create_metadata = libreplex_metadata::instruction::CreateInscriptionMetadata {
        //     metadata_input: CreateMetadataInscriptionInput {
        //         description: None,
        //         data_type: "".to_string(),
        //         name: METADATA_NAME.to_string(),
        //         update_authority: collection_authority,
        //         symbol: "COOL".to_string(),
        //         extensions: vec![],
        //     },
        // };

        // let create_metadata = Instruction {
        //     data: create_metadata.data(),
        //     program_id: libreplex_metadata::ID,
        //     accounts: create_metadata_accounts,
        // };

        // let transaction = Transaction::new_signed_with_payer(
        //     &[initialise_inscription_data_tx, create_metadata],
        //     Some(&context.payer.pubkey()),
        //     &[&context.payer, &mint, &inscription_data],
        //     context.last_blockhash,
        // );

        // context
        //     .banks_client
        //     .process_transaction(transaction)
        //     .await
        //     .unwrap();

        // let mut metadata_account = context
        //     .banks_client
        //     .get_account(metadata)
        //     .await
        //     .unwrap()
        //     .unwrap();

        // let metadata_account_info = AccountInfo::new(
        //     &metadata,
        //     false,
        //     false,
        //     &mut metadata_account.lamports,
        //     &mut metadata_account.data,
        //     &metadata_account.owner,
        //     metadata_account.executable,
        //     metadata_account.rent_epoch,
        // );

        // let metadata_obj: Account<Metadata> = Account::try_from(&metadata_account_info).unwrap();

        // assert_eq!(metadata_obj.name, METADATA_NAME);

        // match metadata_obj.asset {
        //     Asset::Inscription { base_data_account_id, inscription_id, .. } => {
        //         assert_eq!(base_data_account_id, inscription_data.pubkey());
        //         assert_eq!(inscription_id, inscription);
        //     },
        //     _ => {
        //         assert_eq!(true, false);
        //     }

        // };
    }

    pub async fn create_inscription_rank_and_wait(
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
            system_program: system_program::ID,
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
}
