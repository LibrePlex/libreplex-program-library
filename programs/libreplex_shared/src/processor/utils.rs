
use std::collections::BTreeMap;

use anchor_lang::prelude::*;

use crate::SharedError;


pub fn assert_derivation(program_id: &Pubkey, account_key: &Pubkey, path: &[&[u8]]) -> Result<u8> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account_key {
        return err!(SharedError::DerivedKeyInvalid);
    }
    Ok(bump)
}

pub fn check_bump(bump_name: &String, bumps: &BTreeMap<String, u8>, bump: u8) -> Result<()> {
    let expected_bump = *bumps
        .get(bump_name)
        .ok_or(SharedError::MissingBump)?;

    if expected_bump != bump {
        return err!(SharedError::InvalidBump);
    }
    Ok(())
}

