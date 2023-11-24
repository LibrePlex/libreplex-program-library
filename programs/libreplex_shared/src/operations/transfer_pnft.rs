
use mpl_token_metadata::instructions::TransferV1Builder;
// use mpl_token_metadata::instructions:, InstructionBuilder};

use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};

use crate::SharedError;

pub fn transfer_pnft<'info>(
    token_program: &AccountInfo<'info>,
    source_token_account: &AccountInfo<'info>,
    target_token_account: &AccountInfo<'info>,
    source_wallet: &AccountInfo<'info>,
    edition: &AccountInfo<'info>,
    source_token_record: &AccountInfo<'info>,
    target_token_record: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    metadata: &AccountInfo<'info>,
    target_wallet: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    sysvar_instructions: &AccountInfo<'info>,
    auth_rules_program: &AccountInfo<'info>,
    auth_rules: &AccountInfo<'info>,
    authority_seeds: Option<&[&[&[u8]]]>,
    payer: &AccountInfo<'info>
) -> Result<()> {
    // simple. move the token from source token account to the target token account

    // msg!("{}", token_program.key());
    // msg!("{}", source_token_account.key());
    // msg!("{}", target_token_account.key());
    // msg!("{}", source_wallet.key());
    // msg!("{}", edition.key());
    // msg!("{}", source_token_record.key());
    // msg!("{}", target_token_record.key());
    // msg!("{}", mint.key());
    // msg!("{}", metadata.key());
    // msg!("{}", target_wallet.key());
    // msg!("{}", associated_token_program.key());
    // msg!("{}", system_program.key());
    // msg!("{}", sysvar_instructions.key());
    // msg!("{}", auth_rules_program.key());
    // msg!("{}", auth_rules.key());
    // msg!("{}", authority_seeds.key());
    // msg!("{}", payer.key());
    

    let expected_token_account =
        anchor_spl::associated_token::get_associated_token_address(&target_wallet.key(), &mint.key());

    if expected_token_account != target_token_account.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }

    if target_token_account.data_is_empty() {
        msg!("Create token account");
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: target_token_account.to_account_info(),
                authority: target_wallet.to_account_info(),
                mint: mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    let mut transfer_builder = TransferV1Builder::new();

    msg!("Transfer");
    transfer_builder
        .token(source_token_account.key()) // 1
        .token_owner(source_wallet.key()) // 2
        .destination_token(target_token_account.key()) //3
        .destination_owner(target_wallet.key()) //4
        .mint(mint.key()) //5
        .metadata(metadata.key()) //6
        .edition(Some(edition.key())) //7
        .token_record(Some(source_token_record.key())) //8
        .destination_token_record(Some(target_token_record.key())) //9
        .authority(source_wallet.key()) //10
        .payer(payer.key()) //11
        .system_program(system_program.key()) //12
        .sysvar_instructions(sysvar_instructions.key()) //13
        .spl_token_program(token_program.key()) //14
        .spl_ata_program(associated_token_program.key()) //15
        .authorization_rules_program(Some(auth_rules_program.key())) //16
        .authorization_rules(Some(auth_rules.key())); //17
        

    let transfer_infos = [
        source_token_account.to_account_info(),
        source_wallet.to_account_info(),
        target_token_account.to_account_info(),
        target_wallet.to_account_info(),
        mint.to_account_info(),
        metadata.to_account_info(), // fix ! token record
        edition.to_account_info(),
        source_token_record.to_account_info(),
        target_token_record.to_account_info(),
        source_wallet.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
        sysvar_instructions.to_account_info(),
        token_program.to_account_info(),
        associated_token_program.to_account_info(),
        auth_rules_program.to_account_info(),
        auth_rules.to_account_info(),
    ];

    let ix = transfer_builder.amount(1)

        // .build(TransferArgs::V1 {
        //     amount: 1,
        //     authorization_data: None,
        // })
        
        // .map_err(|_| SharedError::InstructionBuildError)?
        .instruction();

    match authority_seeds {
        Some(x) => {
            msg!("invoke_signer");
            invoke_signed(&ix, &transfer_infos, x)?;
        }, None => {
            msg!("invoke");
            invoke(&ix, &transfer_infos)?;
        }
    }
    

    Ok(())
}
