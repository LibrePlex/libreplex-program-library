#![cfg(feature = "test-bpf")]

use anchor_lang::{prelude::*, InstructionData};
use libreplex_monoswap::{accounts::NiftySwapCtx, MarkerState, NiftyMarker, SwapDirection};
use nifty_asset::{accounts::Asset, types::Standard};
use solana_program::program_pack::Pack;
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::{
    instruction::{initialize_mint2, mint_to},
    state::Account as TokenAccount,
};

pub mod helpers;
use helpers::*;

mod nifty_swap_tests {
    use super::*;

    #[tokio::test]
    async fn can_swap_both_directions() {
        let mut context = program_test().start_with_context().await;

        // **Set up keypairs and funding**

        // Authority creates the fungible and the nifty swap account
        let authority_signer = Keypair::new();
        let authority = authority_signer.pubkey();

        // User owns the Nifty asset and does the swapping
        let user_signer = Keypair::new();
        let user = user_signer.pubkey();

        // Fund the authority and user
        airdrop(&mut context, &authority, 1_000_000_000)
            .await
            .unwrap();
        airdrop(&mut context, &user, 1_000_000_000).await.unwrap();

        // **Create Nifty and fungible tokens**

        let FungibleTest { mint, ata } =
            create_fungible_token(&mut context, &authority_signer, 10, TokenProgram::Legacy)
                .await
                .unwrap();

        let AssetTest { asset } = create_nifty_asset(&mut context, &user_signer, user)
            .await
            .unwrap();

        let asset_account = context
            .banks_client
            .get_account(asset)
            .await
            .expect("get_account")
            .expect("asset_account not found");

        let asset_data = Asset::deserialize(&mut asset_account.data.as_slice()).unwrap();
        assert_eq!(asset_data.owner, user);
        assert_eq!(asset_data.authority, user);
        assert_eq!(asset_data.standard, Standard::NonFungible);
        assert_eq!(asset_data.mutable, true);

        let authority_token_account = context
            .banks_client
            .get_account(ata)
            .await
            .expect("get_account")
            .expect("authority_token_account not found");

        let ata_data = spl_token::state::Account::unpack(&authority_token_account.data).unwrap();
        assert_eq!(ata_data.amount, 10);
        assert_eq!(ata_data.owner, authority);
        assert_eq!(ata_data.mint, mint);

        context.warp_to_slot(100).unwrap();

        let NiftySwapTest {
            nifty_marker,
            escrow_owner,
            escrow_ata,
        } = create_nifty_swap(
            &mut context,
            NiftySwapInput {
                authority_signer: &authority_signer,
                asset,
                mint,
                ata,
            },
        )
        .await
        .unwrap();

        let nifty_marker_account = context
            .banks_client
            .get_account(nifty_marker)
            .await
            .expect("get_account")
            .expect("nifty_marker_account not found");

        let nifty_marker_data =
            NiftyMarker::deserialize(&mut &nifty_marker_account.data.as_slice()[8..]).unwrap();

        assert_eq!(nifty_marker_data.namespace, authority);
        assert_eq!(nifty_marker_data.mint, mint);
        assert_eq!(nifty_marker_data.asset, asset);
        assert_eq!(nifty_marker_data.amount, 10);
        assert_eq!(nifty_marker_data.state, MarkerState::FungibleEscrowed);

        context.warp_to_slot(200).unwrap();

        // **Perform the swap**

        let external_ata =
            get_associated_token_address_with_program_id(&user, &mint, &spl_token::ID);

        let create_external_ata_ix =
            create_associated_token_account(&user, &user, &mint, &spl_token::ID);

        let swap_ix = Instruction {
            program_id: libreplex_monoswap::ID,
            accounts: NiftySwapCtx {
                nifty_marker,
                asset,
                mint,
                escrow_owner,
                escrow_token_account: escrow_ata,
                external_token_account: external_ata,
                payer: user,
                token_program: spl_token::ID,
                associated_token_program: spl_associated_token_account::ID,
                system_program: system_program::ID,
                nifty_program: nifty_asset::ID,
            }
            .to_account_metas(None),
            data: libreplex_monoswap::instruction::NiftySwap {
                direction: SwapDirection::AssetIn,
            }
            .data(),
        };

        let blockhash = context.get_new_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[create_external_ata_ix, swap_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user_signer],
            blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // **Check the results**

        // The asset should be owned by escrow_owner.
        let asset_account = context
            .banks_client
            .get_account(asset)
            .await
            .expect("get_account")
            .expect("asset_account not found");

        let asset_data = Asset::deserialize(&mut asset_account.data.as_slice()).unwrap();
        assert_eq!(asset_data.owner, escrow_owner);

        // The fungibles should be transfered to the user's external ATA.
        let external_ata_account = context
            .banks_client
            .get_account(external_ata)
            .await
            .expect("get_account")
            .expect("ata_account not found");

        let external_ata_data = TokenAccount::unpack(&authority_token_account.data).unwrap();
        assert_eq!(external_ata_data.amount, 10);
        assert_eq!(external_ata_data.owner, authority);
        assert_eq!(external_ata_data.mint, mint);

        // The escrow ata should be empty.
        let escrow_ata_account = context
            .banks_client
            .get_account(escrow_ata)
            .await
            .expect("get_account")
            .expect("escrow_ata_account not found");

        let escrow_ata_data = TokenAccount::unpack(&escrow_ata_account.data).unwrap();
        assert_eq!(escrow_ata_data.amount, 0);

        println!("escrow_ata_data: {:?}", escrow_ata);
        println!("external_ata_data: {:?}", external_ata);
        println!("asset_data: {:?}", asset);
        println!("nifty marker: {:?}", nifty_marker);
        println!("user: {:?}", user);

        // **Swap back**
        let swap_ix = Instruction {
            program_id: libreplex_monoswap::ID,
            accounts: NiftySwapCtx {
                nifty_marker,
                asset,
                mint,
                escrow_owner,
                escrow_token_account: escrow_ata,
                external_token_account: external_ata,
                payer: user,
                token_program: spl_token::ID,
                associated_token_program: spl_associated_token_account::ID,
                system_program: system_program::ID,
                nifty_program: nifty_asset::ID,
            }
            .to_account_metas(None),
            data: libreplex_monoswap::instruction::NiftySwap {
                direction: SwapDirection::AssetOut,
            }
            .data(),
        };

        let blockhash = context.get_new_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[swap_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user_signer],
            blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // **Check the results**

        // The asset should be owned by user.
        let asset_account = context
            .banks_client
            .get_account(asset)
            .await
            .expect("get_account")
            .expect("asset_account not found");

        let asset_data = Asset::deserialize(&mut asset_account.data.as_slice()).unwrap();
        assert_eq!(asset_data.owner, user);

        // The fungibles should be transfered back to the escrow ATA.
        let escrow_ata_account = context
            .banks_client
            .get_account(escrow_ata)
            .await
            .expect("get_account")
            .expect("ata_account not found");

        let escrow_ata_data = TokenAccount::unpack(&authority_token_account.data).unwrap();
        assert_eq!(escrow_ata_data.amount, 10);

        // The user's external ata should be empty.
        let external_ata_account = context
            .banks_client
            .get_account(external_ata)
            .await
            .expect("get_account")
            .expect("escrow_ata_account not found");

        let external_ata_data = TokenAccount::unpack(&external_ata_account.data).unwrap();
        assert_eq!(external_ata_data.amount, 0);
    }
}
