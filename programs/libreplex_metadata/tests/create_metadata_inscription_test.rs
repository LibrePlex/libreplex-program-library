use anchor_spl::token::Mint as SplMint;
use solana_program_test::*;
use spl_token_2022::ID;

const METADATA_NAME: &str = "MD-inscription";

mod permissions {
    use anchor_lang::{prelude::Account, system_program, InstructionData, Key, ToAccountMetas};
    use libreplex_inscriptions::Inscription;
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

        let inscription = Keypair::new();

        let rent = context.banks_client.get_rent().await.unwrap();

        let initialise_inscription_tx = system_instruction::create_account(
            &context.payer.pubkey(),
            &inscription.pubkey(),
            rent.minimum_balance(Inscription::BASE_SIZE + 1024_usize),
            Inscription::BASE_SIZE as u64 + 1024_u64,
            &libreplex_inscriptions::id(),
        );

        // msg!("inscription {}", inscription.pubkey);

        let create_metadata_accounts = CreateInscriptionMetadata {
            signer: collection_authority,
            metadata: metadata.key(),
            mint: mint.pubkey(),
            inscription: inscription.pubkey(),
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
            &[initialise_inscription_tx, create_metadata],
            Some(&context.payer.pubkey()),
            &[&context.payer, &mint, &inscription],
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
            Asset::Inscription { account_id, .. } => {
                assert_eq!(account_id, inscription.pubkey());
            },
            _ => {
                assert_eq!(true, false);
            }
           
        };
    }
}
