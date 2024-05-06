use mpl_token_metadata::instructions::{
    CreateMasterEditionV3Builder, CreateMetadataAccountV3Builder,
};

use mpl_token_metadata::instructions::SignMetadataBuilder;


use anchor_spl::token::Mint as SplMint;
use anchor_spl::token::ID;

use mpl_token_metadata::types::Creator;
use mpl_token_metadata::types::DataV2;

use solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};

use anchor_lang::prelude::*;


use crate::SharedError;

pub fn create_mint_with_metadata_and_masteredition<'f>(
    payer: &AccountInfo<'f>,
    owner: &AccountInfo<'f>,
    destination: Option<&AccountInfo<'f>>,
    mint: &AccountInfo<'f>,
    metadata: &AccountInfo<'f>,
    master_edition: Option<&AccountInfo<'f>>,
    token_program: &AccountInfo<'f>,
    token_account: Option<&AccountInfo<'f>>,
    metadata_program: &AccountInfo<'f>,
    associated_token_program: &AccountInfo<'f>,
    system_program: &AccountInfo<'f>,
    verified_creator: &AccountInfo<'f>,
    rent: &AccountInfo<'f>,
    name: &str,
    symbol: &str,
    offchain_url: &str,
    royalties_basis_points: u16,
    creators: Option<Vec<Creator>>,
    verify_first_creator: bool,
    initial_mint_amount: u64,
    decimals: u8,
    max_supply: Option<u64>,
    authority_seeds: &[&[u8]],
    is_spl: bool,
) -> Result<()> {
    let payer_key = payer.key();
    let mint_key = mint.key();
    let metadata_key = metadata.key();
    
    let mint_infos = vec![payer.to_account_info(), mint.to_account_info()];

    let owner_key = &owner.key();

    
    // CREATE MINT
    invoke_signed(
        &system_instruction::create_account(
            &payer_key,
            &mint_key,
            // rent.minimum_balance(Mint::LEN),
            Rent::get()?.minimum_balance(SplMint::LEN),
            SplMint::LEN as u64,
            &ID,
        ),
        mint_infos.as_slice(),
        &[&authority_seeds],
    )?;

    // initialize mint
    invoke(
        &spl_token::instruction::initialize_mint2(
            &spl_token::id(),
            &mint_key,
            owner_key,
            Some(owner_key),
            decimals,
        )?,
        &[token_program.to_account_info(), mint.to_account_info()],
    )?;

    if initial_mint_amount > 0 {
        match destination {
            Some(x) => {
                match token_account {
                    Some(y) => {
                        let token_account_key = &y.key();
                        if !y.data_is_empty() {
                            return Err(SharedError::TokenAccountNotEmpty.into());
                        }
                        // msg!("{}",payer.key() );
                        anchor_spl::associated_token::create(CpiContext::new(
                            associated_token_program.to_account_info(),
                            anchor_spl::associated_token::Create {
                                payer: payer.to_account_info(),
                                associated_token: y.to_account_info(),
                                authority: x.to_account_info(),
                                mint: mint.to_account_info(),
                                system_program: system_program.to_account_info(),
                                token_program: token_program.to_account_info(),
                            },
                        ))?;

                        let mint_to_account_infos = vec![
                            mint.to_account_info(),
                            y.to_account_info(),
                            owner.to_account_info(),
                        ];

                        // msg!("token_account: {}", token_account_key);
                        // msg!("owner: {}", owner_key);
                        // mint to
                        invoke_signed(
                            &spl_token::instruction::mint_to(
                                &spl_token::id(),
                                &mint_key,
                                token_account_key,
                                owner_key,
                                &[],
                                initial_mint_amount,
                            )?,
                            mint_to_account_infos.as_slice(),
                            &[&authority_seeds],
                        )?;
                    }
                    None => {
                        return Err(SharedError::MissingTokenAccount.into());
                    }
                }
            }
            None => {
                return Err(SharedError::MissingDestinationAccount.into());
            }
        }
    }

    let mut create_metadata_builder = CreateMetadataAccountV3Builder::new();

    create_metadata_builder
        .mint_authority(*owner_key)
        .update_authority(*owner_key, false)
        .metadata(metadata_key)
        .payer(payer_key)
        .mint(mint_key)
        .data(DataV2 {
            name: name.to_owned(),
            symbol: symbol.to_owned(),
            uri: offchain_url.to_string(),
            seller_fee_basis_points: royalties_basis_points,
            creators: creators.to_owned(),
            collection: None,
            uses: None,
        });

    // only create master edition and verify creators if decimals is 0 (i.e. we
    // have an NFT)
    if decimals == 0 && !is_spl {
        match master_edition {
            Some(x) => {
                let master_edition_infos = vec![
                    x.to_account_info(),
                    mint.to_account_info(),
                    owner.to_account_info(),
                    payer.to_account_info(),
                    metadata.to_account_info(),
                    metadata_program.to_account_info(),
                    token_program.to_account_info(),
                    system_program.to_account_info(),
                    rent.to_account_info(),
                    verified_creator.to_account_info(),
                ];

                let mut create_master_edition_builder = CreateMasterEditionV3Builder::new();

                create_master_edition_builder
                    .metadata(metadata.key())
                    .edition(x.key())
                    .mint(mint.key())
                    .mint_authority(owner.key())
                    .update_authority(owner.key())
                    .payer(payer.key())
                    .max_supply(match max_supply {
                        Some(x) => x,
                        _ => 0,
                    });

                let ix = create_master_edition_builder.instruction();

                invoke_signed(&ix, master_edition_infos.as_slice(), &[authority_seeds])?;

                match &creators {
                    None => {}
                    _ => match verify_first_creator {
                        true => {
                            let sign_metadata_infos = vec![
                                metadata.to_account_info(),
                                verified_creator.to_account_info(),
                            ];
                            let mut sign_metadata_builder = SignMetadataBuilder::new();

                            let ix = sign_metadata_builder
                                .metadata(metadata.key())
                                .creator(verified_creator.key())
                                .instruction();

                            invoke_signed(&ix, &sign_metadata_infos, &[authority_seeds])?;
                        }
                        false => {}
                    },
                }
            }
            None => {
                return Err(SharedError::MissingMasterEditionForNft.into());
            }
        }
    }

    Ok(())
}
