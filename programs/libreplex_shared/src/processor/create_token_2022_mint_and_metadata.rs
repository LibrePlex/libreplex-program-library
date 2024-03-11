use anchor_lang::prelude::*;

use solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};

use spl_token_2022::{
    extension::{transfer_fee::TransferFeeConfig, ExtensionType},
    instruction::initialize_mint2,
    state::Mint,
};

use spl_token_group_interface::{
    instruction::{initialize_group, initialize_member},
    state::{TokenGroup, TokenGroupMember},
};
use spl_token_metadata_interface::{instruction::initialize, state::TokenMetadata};
use spl_type_length_value::state::{TlvState, TlvStateBorrowed};

/// Accounts to mint an NFT.
pub struct MintAccounts2022<'info> {
    pub authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub nft_owner: AccountInfo<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub spl_token_program: AccountInfo<'info>,
}

pub struct TokenGroupInput {
    pub max_size: u32,
}

pub struct TokenMemberInput<'f> {
    pub group_mint: AccountInfo<'f>,
}

/// Creates the metadata accounts and mint a new token.
pub fn create_token_2022_and_metadata(
    accounts: MintAccounts2022,
    decimals: u8,
    token_metadata: Option<TokenMetadata>,
    // token group is optional - specifying this turns this into a group mint
    token_group: Option<TokenGroupInput>,
    token_member: Option<TokenMemberInput>,
    auth_seeds: Option<&[&[u8]]>,
    transfer_fee_bps: u16,
) -> Result<()> {
    // msg!("create_token_2022_and_metadata called");
    let MintAccounts2022 {
        payer,
        nft_mint,
        spl_token_program,
        nft_owner,
        authority,
        ..
    } = accounts;

    let rent = Rent::get()?;

    let mut extension_types = vec![];

    let mut extension_extra_space: usize = 0;

    match &token_metadata {
        Some(x) => {
            extension_types.push(ExtensionType::MetadataPointer);
            extension_extra_space += x.tlv_size_of().unwrap();
        }
        None => {}
    };

    match &token_group {
        Some(_) => {
            extension_types.push(ExtensionType::GroupPointer);
            extension_extra_space +=
                std::mem::size_of::<TokenGroup>() + TlvStateBorrowed::get_base_len();
        }
        None => {}
    };

    if transfer_fee_bps > 0 {
        extension_types.push(ExtensionType::TransferFeeConfig);
        extension_extra_space += std::mem::size_of::<TransferFeeConfig>()
    }

    match &token_member {
        Some(_) => {
            extension_types.push(ExtensionType::GroupMemberPointer);
            extension_extra_space +=
                std::mem::size_of::<TokenGroupMember>() + TlvStateBorrowed::get_base_len();
        }
        None => {}
    };

    let base_size = ExtensionType::try_calculate_account_len::<Mint>(&extension_types).unwrap();

    let rent_lamports = rent.minimum_balance(base_size + extension_extra_space);

    let create_account_ix = system_instruction::create_account(
        &payer.key(),
        &nft_mint.key(),
        rent_lamports,
        (base_size).try_into().unwrap(),
        &spl_token_2022::ID,
    );

    msg!("Invoke create account {},{}", payer.key(), nft_mint.key());

    invoke(
        &create_account_ix,
        &[
            nft_mint.to_account_info(),
            payer.to_account_info(),
            spl_token_program.to_account_info(),
        ],
    )?;

    match &token_metadata {
        Some(_) => {
            let initialize_extension =
                spl_token_2022::extension::metadata_pointer::instruction::initialize(
                    &spl_token_2022::ID,
                    &nft_mint.key(),
                    Some(authority.key()),
                    // we are using the native metadata implementation,
                    // hence setting metadata address = mint address
                    Some(nft_mint.key()),
                )
                .unwrap();

            msg!("Invoke initialise metadata pointer extension");

            match auth_seeds {
                Some(x) => {
                    invoke_signed(
                        &initialize_extension,
                        &[authority.to_account_info(), nft_mint.to_account_info()],
                        &[x],
                    )?;
                }
                None => {
                    invoke(
                        &initialize_extension,
                        &[authority.to_account_info(), nft_mint.to_account_info()],
                    )?;
                }
            }
        }
        _ => {}
    }

    if transfer_fee_bps > 0 {
        let initialise_transfer_fee_extension =
            spl_token_2022::extension::transfer_fee::instruction::initialize_transfer_fee_config(
                &spl_token_2022::ID,
                &nft_mint.key(),
                Some(&authority.key()),
                Some(&authority.key()),
                transfer_fee_bps,
                std::u64::MAX,
            )?;
        match &auth_seeds {
            Some(y) => {
                invoke_signed(
                    &initialise_transfer_fee_extension,
                    &[
                        nft_mint.to_account_info(),
                        authority.to_account_info(),
                        nft_mint.to_account_info(),
                    ],
                    &[y],
                )?;
            }
            None => {
                invoke(
                    &initialise_transfer_fee_extension,
                    &[
                        nft_mint.to_account_info(),
                        authority.to_account_info(),
                        nft_mint.to_account_info(),
                    ],
                )?;
            }
        }
    }

    match &token_group {
        Some(x) => {
            let initialize_extension =
                spl_token_2022::extension::group_pointer::instruction::initialize(
                    &spl_token_2022::ID,
                    &nft_mint.key(),
                    Some(authority.key()),
                    Some(nft_mint.key()),
                )
                .unwrap();
            match &auth_seeds {
                Some(y) => {
                    invoke_signed(
                        &initialize_extension,
                        &[
                            nft_mint.to_account_info(),
                            authority.to_account_info(),
                            nft_mint.to_account_info(),
                        ],
                        &[y],
                    )?;
                }
                None => {
                    invoke(
                        &initialize_extension,
                        &[
                            nft_mint.to_account_info(),
                            authority.to_account_info(),
                            nft_mint.to_account_info(),
                        ],
                    )?;
                }
            }
        }
        None => {}
    }



    match &token_member {
        Some(x) => {
            let initialize_extension =
                spl_token_2022::extension::group_member_pointer::instruction::initialize(
                    &spl_token_2022::ID,
                    &nft_mint.key(),
                    Some(authority.key()),
                    Some(nft_mint.key()),
                )
                .unwrap();
            match &auth_seeds {
                Some(y) => {
                    invoke_signed(
                        &initialize_extension,
                        &[
                            nft_mint.to_account_info(),
                            authority.to_account_info(),
                            nft_mint.to_account_info(),
                        ],
                        &[y],
                    )?;
                }
                None => {
                    invoke(
                        &initialize_extension,
                        &[
                            nft_mint.to_account_info(),
                            authority.to_account_info(),
                            nft_mint.to_account_info(),
                        ],
                    )?;
                }
            }
        }
        None => {}
    }
    msg!("Invoke initialise mint");

    let initialize_ix = initialize_mint2(
        &spl_token_2022::ID,
        &nft_mint.key(),
        &authority.key(),
        Some(&authority.key()),
        decimals,
    )
    .unwrap();

    // msg!("Invoke initialise mint2");
    invoke(&initialize_ix, &[nft_mint.to_account_info()])?;

    // to be enabled when groups have been audited and rolled out

    // match &token_group {
    //     Some(x) => {
    //         match &auth_seeds {
    //             Some(y) => {
    //                 msg!("Initialise group");
    //                 invoke_signed(
    //                     &initialize_group(
    //                         &spl_token_2022::ID,
    //                         &nft_mint.key(),
    //                         &nft_mint.key(),
    //                         &authority.key(),
    //                         // Pubkey::new_unique().into(),
    //                         Some(authority.key()),
    //                         x.max_size,
    //                     ),
    //                     &[nft_mint.to_account_info(), authority.to_account_info()],
    //                     &[y],
    //                 )?;
    //                 msg!("Group initialised");
    //             }
    //             None => {
    //                 invoke(
    //                     &initialize_group(
    //                         &spl_token_2022::ID,
    //                         &nft_mint.key(),
    //                         &nft_mint.key(),
    //                         &authority.key(),
    //                         // Pubkey::new_unique().into(),
    //                         Some(authority.key()),
    //                         x.max_size,
    //                     ),
    //                     &[nft_mint.to_account_info(), authority.to_account_info()],
    //                 )?;
    //             }
    //         }
    //     }
    //     None => {}
    // }

    msg!("Initialise metadata if needed");
    if let Some(x) = token_metadata {
        let initialise_metadata_ix = initialize(
            &spl_token_2022::ID,
            &nft_mint.key(),
            &authority.key(),
            &nft_mint.key(),
            &authority.key(),
            x.name.clone(),
            x.symbol.clone(),
            x.uri.clone(),
        );

        let account_infos = &[
            nft_mint.to_account_info(),
            authority.to_account_info(),
            nft_mint.to_account_info(),
            nft_owner.to_account_info(),
        ];
        match auth_seeds {
            Some(x) => {
                invoke_signed(&initialise_metadata_ix, account_infos, &[x])?;
            }
            None => {
                invoke(&initialise_metadata_ix, account_infos)?;
            }
        }
    }

    // to be enabled when groups have been audited and rolled out

    // match &token_member {
    //     Some(x) => {
    //         let group_mint = x.group_mint.clone();
    //         match &auth_seeds {
    //             Some(y) => {
    //                 invoke_signed(
    //                     &initialize_member(
    //                         &spl_token_2022::ID,
    //                         &nft_mint.key(),
    //                         &nft_mint.key(),
    //                         &authority.key(),
    //                         &x.group_mint.key(),
    //                         &authority.key(),
    //                     ),
    //                     &[nft_mint, group_mint, authority.to_account_info()],
    //                     &[y],
    //                 )?;
    //             }
    //             None => {
    //                 invoke(
    //                     &initialize_member(
    //                         &spl_token_2022::ID,
    //                         &nft_mint.key(),
    //                         &nft_mint.key(),
    //                         &authority.key(),
    //                         &x.group_mint.key(),
    //                         &authority.key(),
    //                     ),
    //                     &[nft_mint.to_account_info(), x.group_mint.to_account_info(), authority.to_account_info()]
    //                 )?;
    //             }
    //         }
    //     }
    //     None => {}
    // }

    msg!("Finished");
    Ok(())
}
