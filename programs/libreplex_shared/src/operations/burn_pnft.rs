
use mpl_token_metadata::instructions::BurnV1Builder;

use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};



pub fn burn_pnft<'info>(
    token_program: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    owner_wallet: &AccountInfo<'info>,
    edition: &AccountInfo<'info>,
    token_record: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    metadata: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    sysvar_instructions: &AccountInfo<'info>,
    authority_seeds: Option<&[&[&[u8]]]>,
    collection_metadata: &AccountInfo<'info>,
    payer: &AccountInfo<'info>
) -> Result<()> {
    // simple. move the token from source token account to the target token account

    let mut burn_builder = BurnV1Builder::new();

    burn_builder
        .authority(owner_wallet.key()) //10
        .collection_metadata(Some(collection_metadata.key()))
        .metadata(metadata.key()) //6
        .mint(mint.key()) //5
        .token(token_account.key()) // 1
        .edition(Some(edition.key())) //7
        .token_record(Some(token_record.key())) //8
        .system_program(system_program.key()) //12
        .sysvar_instructions(sysvar_instructions.key()) //13
        .spl_token_program(token_program.key()); //14

    let transfer_infos = [
        payer.to_account_info(),
        collection_metadata.to_account_info(),
        metadata.to_account_info(), // fix ! token record
        edition.to_account_info(),
        mint.to_account_info(),
        token_account.to_account_info(),
        token_record.to_account_info(),
        system_program.to_account_info(),
        sysvar_instructions.to_account_info(),
        token_program.to_account_info(),
    ];

    let ix = burn_builder.amount(1)
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
