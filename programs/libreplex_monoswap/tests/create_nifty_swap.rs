#![cfg(feature = "test-bpf")]

use anchor_lang::{prelude::*, InstructionData};
use libreplex_monoswap::{accounts::CreateNiftySwapCtx, instruction::CreateNiftySwap};
use nifty_asset::{accounts::Asset, instructions::CreateBuilder, types::Standard};
use solana_program::program_pack::Pack;
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_2022::instruction::{initialize_mint2, mint_to};

const MINT_LAYOUT: u64 = 82;

pub fn program_test() -> ProgramTest {
    let mut test = ProgramTest::new("libreplex_monoswap", libreplex_monoswap::ID, None);
    test.add_program("nifty_asset", nifty_asset::ID, None);
    test.add_program("spl_token_2022", spl_token_2022::ID, None);

    test
}

pub async fn airdrop(
    context: &mut ProgramTestContext,
    receiver: &Pubkey,
    amount: u64,
) -> Result<()> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &context.payer.pubkey(),
            receiver,
            amount,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    Ok(())
}

mod nifty_swaps {
    use libreplex_monoswap::NiftyMarker;

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

        // Mint account for the fungible token
        let mint_signer = Keypair::new();
        let mint = mint_signer.pubkey();

        // Keypair for the nifty asset
        let asset_signer = Keypair::new();
        let asset = asset_signer.pubkey();

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
        let ata_pubkey = get_associated_token_address(&authority, &mint);
        let ata_escrow_pubkey = get_associated_token_address(&escrow_owner, &mint);

        // Fund the authority and user
        airdrop(&mut context, &authority, 1_000_000_000)
            .await
            .unwrap();
        airdrop(&mut context, &user, 1_000_000_000).await.unwrap();

        // **Create Nifty and fungible tokens**

        // Create a simple Nifty asset with no extensions.
        let nifty_ix = CreateBuilder::new()
            .asset(asset)
            .authority(user, true)
            .owner(user)
            .payer(Some(user))
            .system_program(Some(system_program::ID))
            .name("TestNifty".to_string())
            .standard(Standard::NonFungible)
            .mutable(true)
            .instruction();

        let rent = context.banks_client.get_rent().await.unwrap();
        let min_rent = rent.minimum_balance(MINT_LAYOUT as usize);

        // Create mint account
        let create_mint_account_ix = system_instruction::create_account(
            &authority,
            &mint,
            min_rent,
            MINT_LAYOUT,
            &spl_token::ID,
        );

        // Initalize mint ix
        let init_mint_ix =
            initialize_mint2(&spl_token::ID, &mint, &authority, Some(&authority), 0).unwrap();

        // Create associated account instruction
        let create_assoc_account_ix =
            create_associated_token_account(&authority, &authority, &mint, &spl_token::ID);

        // Mint to instruction
        let mint_to_ix = mint_to(&spl_token::ID, &mint, &ata_pubkey, &authority, &[], 10).unwrap();

        // **Compose tranasaction, send it and assert the results**

        let instructions = vec![
            nifty_ix,
            create_mint_account_ix,
            init_mint_ix,
            create_assoc_account_ix,
            mint_to_ix,
        ];

        let signers = vec![
            &context.payer,
            &authority_signer,
            &mint_signer,
            &user_signer,
            &asset_signer,
        ];

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&context.payer.pubkey()),
            &signers,
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

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
            .get_account(ata_pubkey)
            .await
            .expect("get_account")
            .expect("authority_token_account not found");
        let ata_data = spl_token::state::Account::unpack(&authority_token_account.data).unwrap();
        assert_eq!(ata_data.amount, 10);
        assert_eq!(ata_data.owner, authority);
        assert_eq!(ata_data.mint, mint);

        context.warp_to_slot(100).unwrap();

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
                escrow_token_account: ata_escrow_pubkey,
                source_token_account: ata_pubkey,
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
}
