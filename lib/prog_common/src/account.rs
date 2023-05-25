use std::io::Write;

use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, try_math::*};

pub fn close_account(pda_to_close: &mut AccountInfo, sol_destination_account: &mut AccountInfo) -> Result<()> {

    // Transfer tokens from the account to be closed to the sol destination account.
    let sol_destination_account_lamports_initial = sol_destination_account.lamports();

    **sol_destination_account.lamports.borrow_mut() = sol_destination_account_lamports_initial.try_add(pda_to_close.lamports())?;
    **pda_to_close.lamports.borrow_mut() = 0;

    // Mark the account discriminator as closed.
    let mut data = pda_to_close.try_borrow_mut_data()?;
    let dst: &mut [u8] = &mut data;
    let mut cursor = std::io::Cursor::new(dst);
    cursor
        .write_all(&CLOSED_ACCOUNT_DISCRIMINATOR)
        .map_err(|_| error!(ErrorCode::AnchorSerializationIssue))?;
    Ok(())
}
