use solana_program_test::*;
const METADATA_NAME: &str = "MD1";

const METADATA_NAME_NEW: &str = "MD2";

const METADATA_SYMBOL_NEW: &str = "SYMBOL2";

pub mod create_metadata_util;
use create_metadata_util::*;

mod create_metadata_test {
    use std::borrow::BorrowMut;

    use anchor_lang::prelude::Account;
    use libreplex_metadata::{ Asset, Metadata};
    use solana_program::account_info::AccountInfo;
    use solana_sdk::signer::Signer;

    use super::*;
    #[tokio::test]
    async fn create_metadata() {
        let program = ProgramTest::new(
            "libreplex_metadata",
            libreplex_metadata::ID,
            processor!(libreplex_metadata::entry),
        );

        let mut context = program.start_with_context().await;
        let collection_authority = context.payer.pubkey();

        let metadata = create_metadata_util(
            context.borrow_mut(),
            METADATA_NAME.to_string(),
            Asset::Json {
                url: "https://collection-url.com".to_owned(),
            },
            "COOL".to_string(),
        )
        .await;

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
    }

}
