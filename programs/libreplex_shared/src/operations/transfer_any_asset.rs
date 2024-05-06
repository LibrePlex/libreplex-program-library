use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};

use anchor_lang::prelude::*;

use crate::SharedError;

use super::{transfer_non_pnft, transfer_pnft};

pub fn transfer_any_asset<'info>(
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
    payer: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let mut is_pnft = false;

    if !metadata.to_account_info().data_is_empty() {
        // we may have a pNFT

        let metadata_obj = Metadata::try_from(&metadata.to_account_info())?;
        if let Some(x) = metadata_obj.token_standard {
            if x == TokenStandard::ProgrammableNonFungible {
                is_pnft = true;
            }
        }
    }

    if is_pnft {
        if amount > 1 {
            return Err(SharedError::CannotTransferMultiplePnfts.into());
        }
        msg!("transfer_pnft");
        transfer_pnft(
            token_program,
            source_token_account,
            target_token_account,
            source_wallet,
            edition,
            source_token_record,
            target_token_record,
            mint,
            metadata,
            target_wallet,
            associated_token_program,
            system_program,
            sysvar_instructions,
            auth_rules_program,
            auth_rules,
            authority_seeds,
            payer,
        )?;
    } else {
        transfer_non_pnft(
            token_program,
            source_token_account,
            target_token_account,
            source_wallet,
            mint,
            target_wallet,
            associated_token_program,
            system_program,
            authority_seeds,
            payer,
            amount,
        )?;
    }

    Ok(())
}
