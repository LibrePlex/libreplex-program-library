
use solana_program_test::*;

const METADATA_NAME: &str = "MD1";

const METADATA_NAME_NEW: &str = "MD2";

const METADATA_SYMBOL_NEW: &str = "SYMBOL2";





pub mod create_metadata_update_summary_util;
use create_metadata_update_summary_util::*;
pub mod update_metadata_util;
use update_metadata_util::*;

mod create_metadata_test {

    use std::borrow::BorrowMut;
   
    use anchor_lang::prelude::Account;
    use libreplex_metadata::Metadata;
    use solana_program::account_info::AccountInfo;
    use solana_sdk::signer::Signer;
   
    use super::*;

    #[tokio::test]
    async fn update_metadata() {
        let mut program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            None
        );


        let mut context = program.start_with_context().await;
        let _collection_authority = context.payer.pubkey();

        let metadata = create_metadata_update_summary_util(
            context.borrow_mut(),
            METADATA_NAME.to_string(),
            "https://collection-url.com".to_owned(),
            "COOL".to_string(),
        )
        .await.0;

        // update metadata

        update_metadata_util(
            context.borrow_mut(),
            metadata,
            Some(METADATA_NAME_NEW.to_string()),
            Some( "bla".to_string()),
            Some(METADATA_SYMBOL_NEW.to_string())
        ).await;


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

        assert_eq!(metadata_obj.name, METADATA_NAME_NEW);
    }
}
