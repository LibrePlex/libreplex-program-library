
use std::collections::BTreeMap;

use crate::{errors::ErrorCode};

use anchor_lang::prelude::*;


pub fn assert_pda_derivation(program_id: &Pubkey, account_key: &Pubkey, path: &[&[u8]]) -> Result<u8> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account_key {
        return err!(ErrorCode::DerivedKeyInvalid);
    }
    Ok(bump)
}