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
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::{
    extension::{BaseState, StateWithExtensions},
    instruction::{initialize_mint2, mint_to},
};

pub const MINT_LAYOUT: u64 = 82;

pub fn unpack<S: BaseState>(account_data: &[u8]) -> Result<S> {
    Ok(StateWithExtensions::<S>::unpack(account_data)?.base)
}

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

pub struct FungibleTest {
    pub mint: Pubkey,
    pub ata: Pubkey,
}

pub struct AssetTest {
    pub asset: Pubkey,
}

pub enum TokenProgram {
    Legacy,
    T22,
}

// Create a simple Nifty asset with no extensions.
pub async fn create_nifty_asset(
    context: &mut ProgramTestContext,
    authority_signer: &Keypair,
    owner: Pubkey,
) -> Result<AssetTest> {
    let asset_signer = Keypair::new();
    let asset = asset_signer.pubkey();
    let authority = authority_signer.pubkey();

    let nifty_ix = CreateBuilder::new()
        .asset(asset)
        .authority(authority, true)
        .owner(owner)
        .payer(Some(authority))
        .system_program(Some(system_program::ID))
        .name("TestNifty".to_string())
        .standard(Standard::NonFungible)
        .mutable(true)
        .instruction();

    let signers = vec![&context.payer, &authority_signer, &asset_signer];

    let tx = Transaction::new_signed_with_payer(
        &[nifty_ix],
        Some(&context.payer.pubkey()),
        &signers,
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    return Ok(AssetTest { asset });
}

pub async fn create_fungible_token(
    context: &mut ProgramTestContext,
    authority_signer: &Keypair,
    amount: u64,
    token_program: TokenProgram,
) -> Result<FungibleTest> {
    let mint_signer = Keypair::new();
    let mint = mint_signer.pubkey();

    let rent = context.banks_client.get_rent().await.unwrap();
    let min_rent = rent.minimum_balance(MINT_LAYOUT as usize);

    let authority = authority_signer.pubkey();

    let token_program = match token_program {
        TokenProgram::Legacy => spl_token::ID,
        TokenProgram::T22 => spl_token_2022::ID,
    };

    let ata = get_associated_token_address_with_program_id(&authority, &mint, &token_program);

    // Create mint account
    let create_mint_account_ix = system_instruction::create_account(
        &authority,
        &mint,
        min_rent,
        MINT_LAYOUT,
        &token_program,
    );

    // Initalize mint ix
    let init_mint_ix =
        initialize_mint2(&token_program, &mint, &authority, Some(&authority), 0).unwrap();

    // Create associated account instruction
    let create_assoc_account_ix =
        create_associated_token_account(&authority, &authority, &mint, &token_program);

    // Mint to instruction
    let mint_to_ix = mint_to(&token_program, &mint, &ata, &authority, &[], 10).unwrap();

    // **Compose tranasaction, send it and assert the results**

    let instructions = vec![
        create_mint_account_ix,
        init_mint_ix,
        create_assoc_account_ix,
        mint_to_ix,
    ];

    let signers = vec![&context.payer, &authority_signer, &mint_signer];

    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&context.payer.pubkey()),
        &signers,
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    return Ok(FungibleTest { mint, ata });
}

pub struct NiftySwapInput<'a> {
    pub authority_signer: &'a Keypair,
    pub asset: Pubkey,
    pub mint: Pubkey,
    pub ata: Pubkey,
}

pub struct NiftySwapTest {
    pub nifty_marker: Pubkey,
    pub escrow_owner: Pubkey,
    pub escrow_ata: Pubkey,
}

pub async fn create_nifty_swap<'a>(
    context: &mut ProgramTestContext,
    input: NiftySwapInput<'a>,
) -> Result<NiftySwapTest> {
    let NiftySwapInput {
        authority_signer,
        asset,
        mint,
        ata,
    } = input;

    let authority = authority_signer.pubkey();

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
    let escrow_ata =
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
            escrow_token_account: escrow_ata,
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

    Ok(NiftySwapTest {
        nifty_marker,
        escrow_owner,
        escrow_ata,
    })
}
