use anchor_lang::Result;

use solana_program_test::*;

const MINIMUM_INSCRIPTION_SIZE: u32 = 8;

mod inscriptions_tests {
    use anchor_lang::{InstructionData, ToAccountMetas};

    use anchor_lang::prelude::Account;
    use anchor_spl::associated_token::{get_associated_token_address_with_program_id, self};
    use anchor_spl::token::{spl_token, Token};
    use libreplex_inscriptions::accounts::CreateInscriptionRank;
    use libreplex_inscriptions::constants;
    use libreplex_inscriptions::instructions::{CreateInscriptionRankInput, SignerType};
    use libreplex_inscriptions::InscriptionSummary;
    use libreplex_inscriptions::{
        accounts::CreateInscription,
        accounts::MakeInscriptionImmutable,
        accounts::ResizeInscription,
        accounts::WriteToInscription,
        instructions::{
            create_inscription::CreateInscriptionInput, ResizeInscriptionInput,
            WriteToInscriptionInput,
        },
        Inscription,
    };
    use libreplex_shared::sysvar_instructions_program;
    use solana_program::account_info::AccountInfo;
    use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program};

    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn deploy_test() {
        let program = ProgramTest::new(
            "libreplex_src20",
            libreplex_inscriptions::ID,
            None, // processor!(libreplex_inscriptions::entry),
        );

        let mut context: ProgramTestContext = program.start_with_context().await;

        // let slot = Clock::get().unwrap().slot;

        // resize tests take a while so we need to advance slots in order to
        // avoid RpcError(DeadlineExceeded) on test execution

        let authority = context.payer.pubkey();

        let fungible_mint = Keypair::new();
        let non_fungible_mint = Keypair::new();

        let root = Keypair::new();

        let root_2 = Keypair::new();

        let ticker = "test_ticker";

        let deployment =
            Pubkey::find_program_address(&[b"deployment", ticker.as_ref()], &libreplex_src20::ID).0;

        let fungible_escrow_token_account = get_associated_token_address_with_program_id(
            &context.payer.pubkey(),
            &fungible_mint.pubkey(),
            &spl_token::ID,
        );

        let inscription = Pubkey::find_program_address(
            &[b"inscription", non_fungible_mint.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_v3 = Pubkey::find_program_address(
            &[b"inscription_v3", non_fungible_mint.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_summary = Pubkey::find_program_address(
            &[b"inscription_summary", ticker.as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_data = Pubkey::find_program_address(
            &[b"inscription_data", ticker.as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        // let deployment =
        //     Pubkey::find_program_address(&[b"deployment", ticker.as_ref()], &libreplex_src20::ID).0;

        let process_transaction = context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[Instruction {
                    program_id: libreplex_inscriptions::id(),
                    data: libreplex_src20::instruction::Deploy {
                        input: libreplex_src20::DeployInput {
                            limit_per_mint: 1000,
                            max_number_of_tokens: 21000,
                            ticker: "dodo".to_owned(),
                            image_: "dodo".to_owned(),
                            decimals: 9,
                            /// decimals of the underlying
                            deployment_template: "{\"op\":\"deploy\"}".to_owned(),
                            mint_template: "{\"op\":\"mint\"}".to_owned(),
                            offchain_url: "https://a.com".to_owned(),
                        },
                    }
                    .data(),
                    accounts: libreplex_src20::accounts::DeployCtx {
                        deployment,
                        payer: context.payer.pubkey(),
                        fungible_mint: fungible_mint.pubkey(),
                        fungible_escrow_token_account,
                        non_fungible_mint: non_fungible_mint.pubkey(),
                        inscription_summary,
                        inscription,
                        inscription_v3,
                        inscription_data,
                        token_program: spl_token::ID,
                        associated_token_program: associated_token::ID,
                        inscriptions_program: libreplex_inscriptions::ID,
                        system_program: system_program::ID,
                        sysvar_instructions: sysvar_instructions_program::ID
                    }
                    .to_account_metas(None),
                }],
                Some(&authority),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

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

        
       
        Ok(())
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
        inscription_v2: Pubkey,
    ) {
        let make_inscription_immutable_input =
            libreplex_inscriptions::instruction::MakeInscriptionImmutable {};

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

        let make_inscription_immutable_accounts = MakeInscriptionImmutable {
            payer: context.payer.pubkey(),
            system_program: system_program::ID,
            authority: context.payer.pubkey(),
            inscription_summary,
            inscription,
            inscription2: Some(inscription_v2),
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
