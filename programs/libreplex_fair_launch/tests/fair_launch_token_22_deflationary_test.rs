use std::str::FromStr;

use anchor_lang::prelude::*;

use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::associated_token::{
    self, get_associated_token_address_with_program_id, AssociatedToken,
};

use libreplex_fair_launch::{Deployment, DeploymentConfig, TOKEN2022_DEPLOYMENT_TYPE};
use libreplex_shared::sysvar_instructions_program;
use solana_program::hash::Hash;
use solana_program::program_pack::Pack;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::extension::StateWithExtensions;

const TICKER: &str = "hedgehog";
const CREATOR_FEE_IN_LAMPORTS: u64 = 10_000_000;
const LIMIT_PER_MINT: u64 = 1000;
const MAX_NUMBER_OF_TOKENS: u64 = 2;
const DEFLATION_RATE: u16 = 200; // 2% - this is in basis points
                                 // pick a silly number to make sure we haven't hard coded a 9 in there
const DECIMALS: u8 = 5;
mod fair_launch_deflationary_test {

    use anchor_lang::prelude::Account;
    use libreplex_fair_launch::{Deployment, TOKEN2022_DEPLOYMENT_TYPE};

    use solana_program::account_info::AccountInfo;
    use solana_program::pubkey::Pubkey;

    use solana_program::system_instruction::transfer;
    use solana_program_test::tokio::time::sleep;
    use spl_token_2022::processor::Processor;

    use super::*;

    #[tokio::test]
    async fn fair_launch_test() {
        let mut program = ProgramTest::new(
            "libreplex_fair_launch",
            libreplex_fair_launch::ID,
            None, // processor!(libreplex_inscriptions::entry),
        );

        program.set_compute_max_units(5_000_000);
        program.add_program(
            "libreplex_inscriptions",
            Pubkey::from_str("inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp").unwrap(),
            None,
        );

        // program.add_program(
        //     "mpl_token_metadata",
        //     Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
        //     None,
        // );
        program.prefer_bpf(false);
        program.add_program(
            "spl_token_2022",
            spl_token_2022::id(),
            processor!(Processor::process),
        );
        let mut context: ProgramTestContext = program.start_with_context().await;

        let (deployment, deployment_config, creator_fee_treasury) =
            initialise_token_2022(&mut context, TICKER).await.unwrap();

        let mut deployment_account = context
            .banks_client
            .get_account(deployment)
            .await
            .unwrap()
            .unwrap();

        let deployment_account_info = AccountInfo::new(
            &deployment,
            false,
            false,
            &mut deployment_account.lamports,
            &mut deployment_account.data,
            &deployment_account.owner,
            deployment_account.executable,
            deployment_account.rent_epoch,
        );

        let deployment_account_obj: Account<Deployment> =
            Account::try_from(&deployment_account_info).unwrap();

        let deploy_template = format!("{}-deploy", TICKER);

        let mint_template = format!("{}-mint", TICKER);

        let offchain_url = format!("https://dummy.com/{}.jpg", TICKER);

        assert_eq!(deployment_account_obj.deployment_template, deploy_template);

        assert_eq!(deployment_account_obj.mint_template, mint_template);

        assert_eq!(deployment_account_obj.offchain_url, offchain_url);

        assert_eq!(
            deployment_account_obj.deployment_type,
            TOKEN2022_DEPLOYMENT_TYPE
        );

        assert_eq!(deployment_account_obj.limit_per_mint, LIMIT_PER_MINT);

        let (fungible_mint, fungible_mint_escrow) =
            deploy_2022(&mut context, &deployment_account_obj)
                .await
                .unwrap();

        let mut fungible_mint_account = context
            .banks_client
            .get_account(fungible_mint)
            .await
            .unwrap()
            .unwrap();

        let fungible_mint_account_info = AccountInfo::new(
            &fungible_mint,
            false,
            false,
            &mut fungible_mint_account.lamports,
            &mut fungible_mint_account.data,
            &fungible_mint_account.owner,
            fungible_mint_account.executable,
            fungible_mint_account.rent_epoch,
        );

        let fungible_mint_account_obj = spl_token_2022::state::Mint::unpack_from_slice(
            &fungible_mint_account_info.try_borrow_data().unwrap(),
        )
        .unwrap();

        // check total supply is as expected
        assert_eq!(
            fungible_mint_account_obj.supply,
            deployment_account_obj.max_number_of_tokens
                * deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        );

        assert_eq!(fungible_mint_account_obj.decimals, DECIMALS);

        // check_mint_state(
        //     &mut context.banks_client,
        //     fungible_mint,
        //     None, //Some(deployment.key()), // deployment must retain the update auth so that we can add items to groups later
        //     None, // freeze auth must be None
        //     // check that this mint belongs to the fungible mint group
        //     None,
        //     DECIMALS
        // )
        // .await;

        check_token_account_state(
            &mut context.banks_client,
            fungible_mint_escrow,
            fungible_mint,
            deployment,
            deployment_account_obj.max_number_of_tokens
                * deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        )
        .await;
        let mut banks_client = &mut context.banks_client;

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            0,
            0,
            DECIMALS,
        )
        .await;

        let minter_wallet = Keypair::new();

        let minter_wallet_key = minter_wallet.pubkey();

        banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[transfer(
                    &context.payer.pubkey(),
                    &minter_wallet.pubkey(),
                    1_000_000_000,
                )],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let mut non_fungible_mints: Vec<Pubkey> = vec![];

        let (mint_1, _, banks_client_error) = mint_token_2022(
            banks_client,
            &deployment_account_obj.ticker,
            Some(&minter_wallet),
            creator_fee_treasury,
            fungible_mint,
            &context.payer,
            context.last_blockhash,
        )
        .await;

        assert_eq!(banks_client_error.is_none(), true);

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            0,
            1,
            DECIMALS,
        )
        .await;

        non_fungible_mints.push(mint_1);

        let (mint_2, _, banks_client_error) = mint_token_2022(
            banks_client,
            &deployment_account_obj.ticker,
            Some(&minter_wallet),
            creator_fee_treasury,
            fungible_mint,
            &context.payer,
            context.last_blockhash,
        )
        .await;

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            0,
            2,
            DECIMALS,
        )
        .await;

        assert_eq!(banks_client_error.is_none(), true);

        non_fungible_mints.push(mint_2);

        // 3rd mint should throwan error
        let (_, _, banks_client_error) = mint_token_2022(
            banks_client,
            &deployment_account_obj.ticker,
            Some(&minter_wallet),
            creator_fee_treasury,
            fungible_mint,
            &context.payer,
            context.last_blockhash,
        )
        .await;

        // deployment should be unchanged
        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            0,
            2,
            DECIMALS,
        )
        .await;

        assert_eq!(banks_client_error.unwrap().to_string(), "transport transaction error: Error processing Instruction 0: custom program error: 0x1774");

        for m in non_fungible_mints.iter() {
            check_mint_state(
                banks_client,
                m.clone(),
                None, // mint auth must be None
                None, // freeze auth must be None
                // check that this mint belongs to the fungible mint group
                Some(fungible_mint.clone()),
            )
            .await;
        }

        check_token_account_state(
            banks_client,
            fungible_mint_escrow,
            fungible_mint,
            deployment.key(),
            deployment_account_obj.max_number_of_tokens
                * deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        )
        .await;

        println!("Swapping NFT #1");
        // see if we can swap
        let minter_fungible_token_account = swap_to_fungible_2022(
            banks_client,
            &deployment_account_obj.ticker,
            non_fungible_mints[0],
            Some(&minter_wallet),
            fungible_mint,
            context.payer.pubkey(),
            context.last_blockhash,
        )
        .await
        .unwrap();

        check_token_account_state(
            banks_client,
            fungible_mint_escrow,
            fungible_mint,
            deployment.key(),
            (deployment_account_obj.max_number_of_tokens - 1)
                * deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        )
        .await;

        let post_swap_1_balance = deployment_account_obj
            .limit_per_mint
            .checked_mul(10000_u64 - DEFLATION_RATE as u64)
            .unwrap()
            .checked_div(10000_u64)
            .unwrap()
            * (10_u64.pow(deployment_account_obj.decimals as u32));

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            minter_wallet_key,
            post_swap_1_balance,
        )
        .await;

        println!("post swap balance: {}", post_swap_1_balance);

        // deployment should be unchanged
        check_deployment_account_state(
            &mut banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            1,
            2,
            DECIMALS,
        )
        .await;

        println!("Swapping NFT #2");
        // need to swap another. otherwise not enough funds for a swap to non-fungible
        let minter_fungible_token_account = swap_to_fungible_2022(
            banks_client,
            &deployment_account_obj.ticker,
            non_fungible_mints[1],
            Some(&minter_wallet),
            fungible_mint,
            context.payer.pubkey(),
            context.last_blockhash,
        )
        .await
        .unwrap();

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            minter_wallet_key,
            post_swap_1_balance.checked_mul(2_u64).unwrap(),
        )
        .await;


        let mut numerator = (deployment_account_obj.limit_per_mint as u128).checked_mul(10000_u128).unwrap().checked_mul(10_u128.pow(deployment_account_obj.decimals as u32)).unwrap();

        let denominator = 10000_u128.checked_sub(DEFLATION_RATE as u128).unwrap();

        let remainder = numerator.checked_rem(denominator);
    
        if let Some(x) = remainder {
            if x > 0 {
                numerator = numerator.checked_add(denominator).unwrap().checked_sub(x).unwrap();
            }
        }

        let expected_cost_to_swap = numerator.checked_div(denominator).unwrap();


        println!("Expected cost to swap {}", expected_cost_to_swap);

        // see if we can swap back
        swap_to_non_fungible_2022(
            banks_client,
            &deployment_account_obj.ticker,
            non_fungible_mints[0],
            Some(&minter_wallet),
            fungible_mint,
            context.payer.pubkey(),
            context.last_blockhash,
        )
        .await
        .unwrap();


        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            minter_wallet_key,
            post_swap_1_balance.checked_mul(2_u64).unwrap().checked_sub(expected_cost_to_swap as u64).unwrap(),
        )
        .await;


        // check_token_account_state(
        //     banks_client,
        //     minter_fungible_token_account,
        //     fungible_mint,
        //     minter_wallet_key,
        //     post_swap_1_balance.checked_mul(2_u64).unwrap(),
        // )
        // .await;

        // deployment should be unchanged
        check_deployment_account_state(
            &mut banks_client,
            deployment,
            fungible_mint,
            context.payer.pubkey(),
            1,
            2,
            DECIMALS,
        )
        .await;
    println!("cost_to_swap {}", expected_cost_to_swap);

    let post_swap_2_balance = post_swap_1_balance.checked_mul(2_u64).unwrap().checked_sub(expected_cost_to_swap as u64).unwrap();

        // check_token_account_state(
        //     banks_client,
        //     minter_fungible_token_account,
        //     fungible_mint,
        //     minter_wallet_key,
        //     post_swap_2_balance,
        // )
        // .await;
    }
}

pub async fn initialise_token_2022(
    context: &mut ProgramTestContext,
    ticker: &str,
) -> Result<(Pubkey, Pubkey, Pubkey)> {
    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deploy_template = format!("{}-deploy", ticker);
    let mint_template = format!("{}-mint", ticker);
    let offchain_url = format!("https://dummy.com/{}.jpg", ticker);

    let creator_fee_treasury = Keypair::new().pubkey();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::InitialiseV2 {
                    input: libreplex_fair_launch::InitialiseInputV2 {
                        limit_per_mint: LIMIT_PER_MINT,
                        max_number_of_tokens: MAX_NUMBER_OF_TOKENS,
                        decimals: DECIMALS,
                        ticker: TICKER.to_owned(),
                        deployment_template: deploy_template.clone(),
                        mint_template: mint_template.clone(),
                        offchain_url: offchain_url.clone(),
                        creator_cosign_program_id: None,
                        use_inscriptions: true,
                        deployment_type: TOKEN2022_DEPLOYMENT_TYPE,
                        creator_fee_per_mint_in_lamports: CREATOR_FEE_IN_LAMPORTS,
                        creator_fee_treasury,
                        deflation_rate_per_swap: DEFLATION_RATE,
                    },
                }
                .data(),
                accounts: libreplex_fair_launch::accounts::InitialiseV2Ctx {
                    deployment,
                    deployment_config,

                    payer: context.payer.pubkey(),
                    creator: context.payer.pubkey(),
                    system_program: system_program::ID,
                }
                .to_account_metas(None),
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    Ok((deployment, deployment_config, creator_fee_treasury))
}

pub async fn deploy_2022<'info>(
    context: &mut ProgramTestContext,
    deployment: &Account<'info, Deployment>,
) -> Result<(Pubkey, Pubkey)> {
    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let fungible_mint = Keypair::new();
    // let non_fungible_mint = Keypair::new();
    // let fungible_mint_group = Keypair::new();
    // let group_mint = Keypair::new();

    let fungible_escrow_token_account =
        anchor_spl::associated_token::get_associated_token_address_with_program_id(
            &deployment.key(),
            &fungible_mint.pubkey(),
            &anchor_spl::token_2022::ID,
        );
    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::DeployToken22 {}.data(),
                accounts: libreplex_fair_launch::accounts::DeployToken2022Ctx {
                    deployment: deployment.key(),
                    deployment_config: deployment_config.key(),
                    hashlist,
                    payer: context.payer.pubkey(),
                    creator: context.payer.pubkey(),
                    fungible_mint: fungible_mint.pubkey(),
                    fungible_escrow_token_account,
                    // just need
                    token_program_2022: anchor_spl::token_2022::ID,
                    associated_token_program: AssociatedToken::id(),
                    system_program: system_program::ID,
                    sysvar_instructions: sysvar_instructions_program::ID,
                    // these will be ignored for hybrid w/ deployment type TOKEN2022
                    rent: Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap(),
                }
                .to_account_metas(None),
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer, &fungible_mint],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // check that the fungible token has been minted out into the escrow
    // and that authorities have been removed
    Ok((fungible_mint.pubkey(), fungible_escrow_token_account))
}

pub async fn mint_token_2022(
    banks_client: &mut BanksClient,
    ticker: &str,
    minter_wallet: Option<&Keypair>,
    creator_fee_treasury: Pubkey,
    fungible_mint: Pubkey,
    context_payer: &Keypair,
    recent_blockhash: Hash,
) -> (Pubkey, Pubkey, Option<BanksClientError>) {
    let non_fungible_mint = Keypair::new();

    let mut signing_keypairs = vec![&context_payer, &non_fungible_mint];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(&x);
            x.pubkey()
        }
        _ => context_payer.pubkey(),
    };

    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            non_fungible_mint.pubkey().as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_token_account = get_associated_token_address_with_program_id(
        &&minter_wallet_key,
        &non_fungible_mint.pubkey(),
        &spl_token_2022::ID,
    );

    let inscription_summary =
        Pubkey::find_program_address(&[b"inscription_summary"], &libreplex_inscriptions::ID).0;

    let inscription_v3 = Pubkey::find_program_address(
        &[b"inscription_v3", non_fungible_mint.pubkey().as_ref()],
        &libreplex_inscriptions::ID,
    )
    .0;

    let inscription_data = Pubkey::find_program_address(
        &[b"inscription_data", non_fungible_mint.pubkey().as_ref()],
        &libreplex_inscriptions::ID,
    )
    .0;

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let mut accounts = libreplex_fair_launch::accounts::MintToken2022Ctx {
        deployment,
        deployment_config,
        creator_fee_treasury,
        payer: minter_wallet_key,
        fungible_mint,
        system_program: system_program::ID,
        hashlist,
        hashlist_marker,
        signer: context_payer.pubkey(),
        minter: minter_wallet_key,
        non_fungible_mint: non_fungible_mint.pubkey(),
        non_fungible_token_account,
        token_program: spl_token_2022::ID,
        associated_token_program: associated_token::ID,
        
    }
    .to_account_metas(None);

    accounts.push(AccountMeta{
        pubkey: libreplex_inscriptions::ID,
        is_signer: false,
        is_writable: false,
    });

    accounts.push(AccountMeta{
        pubkey: inscription_summary,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta{
        pubkey: inscription_v3,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta{
        pubkey: inscription_data,
        is_signer: false,
        is_writable: true,
    });

    let err = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::MintToken22 {}.data(),
                accounts,
            }],
            Some(&minter_wallet_key),
            signing_keypairs.as_slice(),
            recent_blockhash,
        ))
        .await
        .err();

    (non_fungible_mint.pubkey(), non_fungible_token_account, err)
}

pub async fn swap_to_fungible_2022(
    banks_client: &mut BanksClient,
    ticker: &str,
    non_fungible_mint: Pubkey,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
    payer: Pubkey,
    last_blockhash: Hash,
) -> Result<Pubkey> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(&x);
            x.pubkey()
        }
        _ => payer,
    };

    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            non_fungible_mint.as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_source_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &non_fungible_mint,
        &spl_token_2022::ID,
    );

    let non_fungible_target_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &non_fungible_mint,
        &spl_token_2022::ID,
    );

    let fungible_source_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &fungible_mint,
        // fungible is still old skool at this point
        &anchor_spl::token_2022::ID,
    );

    let fungible_target_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &fungible_mint,
        &anchor_spl::token_2022::ID,
    );

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToFungible22 {}.data(),
                accounts: libreplex_fair_launch::accounts::SwapToFungible2022Ctx {
                    deployment,
                    payer: minter_wallet_key,
                    signer: minter_wallet_key,
                    fungible_target_token_account_owner: minter_wallet_key,
                    non_fungible_source_account_owner: minter_wallet_key,
                    system_program: system_program::ID,
                    hashlist_marker,
                    non_fungible_mint,
                    token_program: anchor_spl::token::ID,
                    token_program_22: spl_token_2022::ID,
                    associated_token_program: associated_token::ID,
                    sysvar_instructions: sysvar_instructions_program::ID,
                    fungible_mint,
                    fungible_source_token_account,
                    fungible_target_token_account,
                    non_fungible_source_token_account,
                    non_fungible_target_token_account,
                }
                .to_account_metas(None),
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            last_blockhash,
        ))
        .await
        .unwrap();

    Ok(fungible_target_token_account)
}

pub async fn swap_to_non_fungible_2022(
    banks_client: &mut BanksClient,
    ticker: &str,
    non_fungible_mint: Pubkey,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
    payer: Pubkey,
    last_blockhash: Hash,
) -> Result<()> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(&x);
            x.pubkey()
        }
        _ => payer,
    };

    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            non_fungible_mint.as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_source_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &non_fungible_mint,
        &spl_token_2022::ID,
    );

    let non_fungible_target_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &non_fungible_mint,
        &spl_token_2022::ID,
    );

    let fungible_source_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &fungible_mint,
        &anchor_spl::token_2022::ID,
    );

    let fungible_target_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &fungible_mint,
        &anchor_spl::token_2022::ID,
    );

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToNonfungible22 {}.data(),
                accounts: libreplex_fair_launch::accounts::SwapToNonFungible2022Ctx {
                    deployment,
                    deployment_config,
                    payer: minter_wallet_key,
                    system_program: system_program::ID,
                    hashlist_marker,
                    non_fungible_mint,
                    token_program: anchor_spl::token::ID,
                    token_program_22: spl_token_2022::ID,
                    associated_token_program: associated_token::ID,
                    sysvar_instructions: sysvar_instructions_program::ID,
                    fungible_mint,
                    fungible_source_token_account,
                    fungible_target_token_account,
                    non_fungible_source_token_account,
                    non_fungible_target_token_account,
                }
                .to_account_metas(None),
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            last_blockhash,
        ))
        .await
        .unwrap();

    Ok(())
}

pub async fn check_token_account_state(
    banks_client: &mut BanksClient,
    token_account: Pubkey,
    expected_mint: Pubkey,
    expected_owner: Pubkey,
    expected_amount: u64,
) {
    let mut token_account_data = banks_client
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();

    let token_account_info = AccountInfo::new(
        &token_account,
        false,
        false,
        &mut token_account_data.lamports,
        &mut token_account_data.data,
        &token_account_data.owner,
        token_account_data.executable,
        token_account_data.rent_epoch,
    );

    let token_account_obj = spl_token_2022::state::Account::unpack_from_slice(
        &token_account_info.try_borrow_data().unwrap(),
    )
    .unwrap();

    assert_eq!(token_account_obj.mint, expected_mint);
    assert_eq!(token_account_obj.owner, expected_owner);

    assert_eq!(token_account_obj.amount, expected_amount);
}

pub async fn check_deployment_account_state(
    banks_client: &mut BanksClient,
    deployment: Pubkey,
    expected_fungible_mint: Pubkey,
    expected_creator: Pubkey,
    expected_escrow_non_fungible_count: u64,
    expected_tokens_issued: u64,
    expected_decimals: u8,
) {
    let mut deployment_account = banks_client.get_account(deployment).await.unwrap().unwrap();

    let deployment_account_info = AccountInfo::new(
        &deployment,
        false,
        false,
        &mut deployment_account.lamports,
        &mut deployment_account.data,
        &deployment_account.owner,
        deployment_account.executable,
        deployment_account.rent_epoch,
    );

    let deployment_account_obj: Account<Deployment> =
        Account::try_from(&deployment_account_info).unwrap();

    assert_eq!(
        deployment_account_obj.fungible_mint,
        expected_fungible_mint.key()
    );

    assert_eq!(deployment_account_obj.creator, expected_creator);

    assert_eq!(
        deployment_account_obj.number_of_tokens_issued,
        expected_tokens_issued
    );

    assert_eq!(
        deployment_account_obj.escrow_non_fungible_count,
        expected_escrow_non_fungible_count
    );

    assert_eq!(deployment_account_obj.decimals, expected_decimals);
}

pub async fn check_deployment_config_account_state(
    banks_client: &mut BanksClient,
    deployment: Pubkey,
    creator_fee_treasury: Pubkey,
    creator_fee_per_mint_lamports: u64,
) {
    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let mut deployment_config_account = banks_client
        .get_account(deployment_config)
        .await
        .unwrap()
        .unwrap();

    let deployment_config_account_info = AccountInfo::new(
        &deployment_config,
        false,
        false,
        &mut deployment_config_account.lamports,
        &mut deployment_config_account.data,
        &deployment_config_account.owner,
        deployment_config_account.executable,
        deployment_config_account.rent_epoch,
    );

    let deployment_account_obj: Account<DeploymentConfig> =
        Account::try_from(&deployment_config_account_info).unwrap();

    assert_eq!(
        deployment_account_obj.creator_fee_per_mint_lamports,
        creator_fee_per_mint_lamports
    );

    assert_eq!(
        deployment_account_obj.creator_fee_treasury,
        creator_fee_treasury
    );
}

pub async fn check_mint_state(
    banks_client: &mut BanksClient,
    mint: Pubkey,
    expected_mint_authority: Option<Pubkey>,
    expected_freeze_authority: Option<Pubkey>,
    _expected_group: Option<Pubkey>, // to enable once groups are enabled
) {
    println!("Check mint state");
    let mut mint_account = banks_client.get_account(mint).await.unwrap().unwrap();

    let mint_account_info = AccountInfo::new(
        &mint,
        false,
        false,
        &mut mint_account.lamports,
        &mut mint_account.data,
        &mint_account.owner,
        mint_account.executable,
        mint_account.rent_epoch,
    );

    let input_data = &mint_account_info.try_borrow_data().unwrap();
    let mint_account_obj =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(input_data).unwrap();

    match &expected_freeze_authority {
        None => {
            assert_eq!(mint_account_obj.base.freeze_authority.is_none(), true);
        }
        Some(x) => {
            assert_eq!(mint_account_obj.base.freeze_authority.unwrap(), x.clone());
        }
    }

    match &expected_mint_authority {
        None => {
            assert_eq!(mint_account_obj.base.mint_authority.is_none(), true);
        }
        Some(x) => {
            assert_eq!(mint_account_obj.base.mint_authority.unwrap(), x.clone());
        }
    }

    // Renable once groups have rolled out
    // match &expected_group {
    //     None => {}
    //     Some(x) => {
    //         let token_group_member_extension = mint_account_obj.get_extension::<TokenGroupMember>();
    //         println!("Group is not none");
    //         let extension = token_group_member_extension.unwrap();
    //         assert_eq!(extension.mint, mint);

    //         assert_eq!(&extension.group, x);
    //     }
    // }
}
