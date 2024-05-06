use mpl_token_metadata::instructions::{
    CreateMasterEditionV3Builder, CreateMetadataAccountV3Builder,
};

use mpl_token_metadata::instructions::SignMetadataBuilder;

use mpl_token_metadata::types::Creator;
use mpl_token_metadata::types::DataV2;

use solana_program::program::{invoke_signed, invoke};

use anchor_lang::prelude::*;

use crate::SharedError;

pub fn create_metadata_and_masteredition<'f>(
    payer: &AccountInfo<'f>,
    owner: &AccountInfo<'f>,
    mint: &AccountInfo<'f>,
    metadata: &AccountInfo<'f>,
    master_edition: Option<&AccountInfo<'f>>,
    token_program: &AccountInfo<'f>,
    metadata_program: &AccountInfo<'f>,
    system_program: &AccountInfo<'f>,
    verified_creator: Option<&AccountInfo<'f>>,
    // rent: &AccountInfo<'f>,
    name: String,
    symbol: String,
    offchain_url: String,
    royalties_basis_points: u16,
    creators: Option<Vec<Creator>>,
    max_supply: Option<u64>,
    authority_seeds: Option<&[&[u8]]>,
    is_spl: bool,
) -> Result<()> {
    let payer_key = payer.key();
    let mint_key = mint.key();
    let metadata_key = metadata.key();

    let owner_key = &owner.key();

    let mut create_metadata_builder = CreateMetadataAccountV3Builder::new();

    create_metadata_builder
        .mint_authority(*owner_key)
        .update_authority(*owner_key, false)
        .metadata(metadata_key)
        .payer(payer_key)
        .mint(mint_key)
        .data(DataV2 {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: offchain_url.clone(),
            seller_fee_basis_points: royalties_basis_points,
            creators: creators.to_owned(),
            collection: None,
            uses: None,
        });

    // only create master edition and verify creators if decimals is 0 (i.e. we
    // have an NFT)
    if !is_spl {
        match master_edition {
            Some(x) => {
                let mut master_edition_infos = vec![
                    x.to_account_info(),
                    mint.to_account_info(),
                    owner.to_account_info(),
                    payer.to_account_info(),
                    metadata.to_account_info(),
                    metadata_program.to_account_info(),
                    token_program.to_account_info(),
                    system_program.to_account_info(),
                    // rent.to_account_info(),
                ];

                match &verified_creator {
                    Some(x) => {
                        master_edition_infos.push(x.to_account_info());
                    }
                    None => {}
                };

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

                match authority_seeds {
                    Some(x) => {
                        invoke_signed(&ix, master_edition_infos.as_slice(), &[x])?;
                        match &creators {
                            None => {}
                            _ => {
                                if let Some(_verified_creator) = verified_creator {
                                    let sign_metadata_infos =
                                        vec![metadata.to_account_info(), _verified_creator.to_account_info()];
                                    let mut sign_metadata_builder = SignMetadataBuilder::new();
        
                                    let ix = sign_metadata_builder
                                        .metadata(metadata.key())
                                        .creator(_verified_creator.key())
                                        .instruction();
        
                                    invoke_signed(&ix, &sign_metadata_infos, &[x])?;
                                }
                            },
                        }
                    },
                    None => {
                        invoke(&ix, master_edition_infos.as_slice())?;
                    }
                }

               
            }
            None => {
                return Err(SharedError::MissingMasterEditionForNft.into());
            }
        }
    }

    Ok(())
}
