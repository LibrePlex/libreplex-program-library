#![cfg(feature = "test-bpf")]

use anchor_lang::{prelude::*, InstructionData};
use libreplex_monoswap::{accounts::CreateNiftySwapCtx, instruction::CreateNiftySwap, NiftyMarker};
use nifty_asset::{accounts::Asset, instructions::CreateBuilder, types::Standard};
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

mod nifty_swaps {
    use super::*;

    #[tokio::test]
    async fn can_create_new_nifty_swap_with_legacy_token() {
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

        let nifty_marker = Pubkey::find_program_address(
            &[
                b"nifty_marker",
                authority.as_ref(),
                asset.as_ref(),
                mint.as_ref(),
            ],
            &libreplex_monoswap::ID,
        )
        .0;

        // Monoswap nifty escrow owner pda
        let escrow_owner = Pubkey::find_program_address(
            &[
                b"nifty_escrow",
                authority.as_ref(),
                asset.as_ref(),
                mint.as_ref(),
            ],
            &libreplex_monoswap::ID,
        )
        .0;

        // Associated token accounts for the authority and escrow
        let ata_escrow =
            get_associated_token_address_with_program_id(&escrow_owner, &mint, &spl_token::ID);

        // **Create the Nifty swap account**
        let create_swap_ix = Instruction {
            program_id: libreplex_monoswap::ID,
            accounts: CreateNiftySwapCtx {
                namespace: authority,
                payer: authority,
                nifty_marker,
                asset,
                mint,
                escrow_owner,
                escrow_token_account: ata_escrow,
                source_token_account: ata,
                token_program: spl_token::ID,
                associated_token_program: spl_associated_token_account::ID,
                system_program: system_program::ID,
                nifty_program: nifty_asset::ID,
            }
            .to_account_metas(None),
            data: CreateNiftySwap { amount: 10 }.data(),
        };

        let blockhash = context.get_new_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[create_swap_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &authority_signer],
            blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

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
        assert_eq!(nifty_marker_data.amount, 10);
    }

    #[tokio::test]
    async fn can_create_new_nifty_swap_with_token_2022() {
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
            create_fungible_token(&mut context, &authority_signer, 10, TokenProgram::T22)
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

        let ata_data = unpack::<TokenAccount>(&authority_token_account.data).unwrap();

        assert_eq!(ata_data.amount, 10);
        assert_eq!(ata_data.owner, authority);
        assert_eq!(ata_data.mint, mint);

        context.warp_to_slot(100).unwrap();

        let nifty_marker = Pubkey::find_program_address(
            &[
                b"nifty_marker",
                authority.as_ref(),
                asset.as_ref(),
                mint.as_ref(),
            ],
            &libreplex_monoswap::ID,
        )
        .0;

        // Monoswap nifty escrow owner pda
        let escrow_owner = Pubkey::find_program_address(
            &[
                b"nifty_escrow",
                authority.as_ref(),
                asset.as_ref(),
                mint.as_ref(),
            ],
            &libreplex_monoswap::ID,
        )
        .0;

        // Associated token accounts for the authority and escrow
        let ata_escrow =
            get_associated_token_address_with_program_id(&escrow_owner, &mint, &spl_token_2022::ID);

        // **Create the Nifty swap account**
        let create_swap_ix = Instruction {
            program_id: libreplex_monoswap::ID,
            accounts: CreateNiftySwapCtx {
                namespace: authority,
                payer: authority,
                nifty_marker,
                asset,
                mint,
                escrow_owner,
                escrow_token_account: ata_escrow,
                source_token_account: ata,
                token_program: spl_token_2022::ID,
                associated_token_program: spl_associated_token_account::ID,
                system_program: system_program::ID,
                nifty_program: nifty_asset::ID,
            }
            .to_account_metas(None),
            data: CreateNiftySwap { amount: 10 }.data(),
        };

        println!("source ata: {:?}", ata);
        println!("authority: {:?}", authority);
        println!("mint: {:?}", mint);

        let blockhash = context.get_new_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[create_swap_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &authority_signer],
            blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

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
        assert_eq!(nifty_marker_data.amount, 10);
    }
}
