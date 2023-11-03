use anchor_spl::token::Mint as SplMint;
use anchor_lang::Result;
use solana_program_test::*;
use spl_token_2022::ID;

const METADATA_NAME: &str = "MD-inscription";

mod permissions {
    use anchor_lang::{prelude::Account, system_program, InstructionData, Key, ToAccountMetas};
    use libreplex_inscriptions::{Inscription, instructions::CreateInscriptionRankInput, accounts::CreateInscriptionRank};
    use libreplex_metadata::{
        accounts::CreateInscriptionMetadata, instructions::CreateMetadataInscriptionInput, Metadata, Asset,
    };
    use solana_program::{
        account_info::AccountInfo, instruction::Instruction, pubkey::Pubkey, system_instruction,
    };
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_metadata_inscription() {
        let mut program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            processor!(libreplex_metadata::entry),
        );

        program.add_program(
            "libreplex_inscriptions",
            libreplex_inscriptions::ID,
            processor!(libreplex_inscriptions::entry),
        );
        let mut context: ProgramTestContext = program.start_with_context().await;

        let collection_authority = context.payer.pubkey();

        let mint = Keypair::new();

        // CREATE MINT

        let rent = context.banks_client.get_rent().await.unwrap();

        let allocate_ix = system_instruction::create_account(
            &context.payer.pubkey(),
            &mint.pubkey(),
            rent.minimum_balance(SplMint::LEN),
            SplMint::LEN as u64,
            &ID,
        );


        let inscription_ranks_current_page = create_inscription_rank_and_wait(
            &mut context,
            0
        ).await.unwrap();

        let inscription_ranks_next_page = create_inscription_rank_and_wait(
            &mut context,
            1
        ).await.unwrap();

        let initialize_ix = spl_token_2022::instruction::initialize_mint2(
            &ID,
            &mint.pubkey(),
            &context.payer.pubkey(),
            Some(&context.payer.pubkey()),
            0,
        )
        .unwrap();

        let create_account_tx = Transaction::new_signed_with_payer(
            &[allocate_ix, initialize_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &mint],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(create_account_tx)
            .await
            .unwrap();

        let metadata = Pubkey::find_program_address(
            &[b"metadata", mint.pubkey().as_ref()],
            &libreplex_metadata::ID,
        )
        .0;

        let inscription = Pubkey::find_program_address(
            &["inscription".as_bytes(), mint.pubkey().as_ref()],
            &libreplex_inscriptions::ID,
        )
        .0;

        let inscription_data = Keypair::new();

        let rent = context.banks_client.get_rent().await.unwrap();

        let initialise_inscription_data_tx = system_instruction::create_account(
            &context.payer.pubkey(),
            &inscription_data.pubkey(),
            rent.minimum_balance(Inscription::SIZE + 1024_usize),
            Inscription::SIZE as u64 + 1024_u64,
            &libreplex_inscriptions::id(),
        );

        // msg!("inscription {}", inscription.pubkey);

        let inscription_summary =
            Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;


        let create_metadata_accounts = CreateInscriptionMetadata {
            signer: collection_authority,
            metadata: metadata.key(),
            mint: mint.pubkey(),
            inscription_summary,
            inscription_ranks_current_page,
            inscription_ranks_next_page,
            inscription,
            inscription_data: inscription_data.pubkey(),
            inscriptions_program: libreplex_inscriptions::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        let create_metadata = libreplex_metadata::instruction::CreateInscriptionMetadata {
            metadata_input: CreateMetadataInscriptionInput {
                description: None,
                data_type: "".to_string(),
                name: METADATA_NAME.to_string(),
                update_authority: collection_authority,
                symbol: "COOL".to_string(),
                extensions: vec![],
            },
        };

        let create_metadata = Instruction {
            data: create_metadata.data(),
            program_id: libreplex_metadata::ID,
            accounts: create_metadata_accounts,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialise_inscription_data_tx, create_metadata],
            Some(&context.payer.pubkey()),
            &[&context.payer, &mint, &inscription_data],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap();

        let mut metadata_account = context
            .banks_client
            .get_account(metadata)
            .await
            .unwrap()
            .unwrap();

        let metadata_account_info = AccountInfo::new(
            &metadata,
            false,
            false,
            &mut metadata_account.lamports,
            &mut metadata_account.data,
            &metadata_account.owner,
            metadata_account.executable,
            metadata_account.rent_epoch,
        );

        let metadata_obj: Account<Metadata> = Account::try_from(&metadata_account_info).unwrap();

        assert_eq!(metadata_obj.name, METADATA_NAME);

        match metadata_obj.asset {
            Asset::Inscription { base_data_account_id, inscription_id, .. } => {
                assert_eq!(base_data_account_id, inscription_data.pubkey());
                assert_eq!(inscription_id, inscription);
            },
            _ => {
                assert_eq!(true, false);
            }
           
        };
    }

    pub async fn create_inscription_rank_and_wait(
        context: &mut ProgramTestContext,
        page_index: u32,
    ) -> Result<Pubkey>{
        let page = Pubkey::find_program_address(
            &["inscription_rank".as_bytes(), &(page_index as u32).to_le_bytes()],
            &libreplex_inscriptions::id(),
        )
        .0;
      
        let create_inscription_rank_input = libreplex_inscriptions::instruction::CreateInscriptionRankPage {
            input: CreateInscriptionRankInput {
                page_index
            }
        };


        let create_inscription_rank_accounts = CreateInscriptionRank {
            payer: context.payer.pubkey(),
            page,
            system_program: system_program::ID
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
