use std::collections::BTreeMap;

use anchor_lang::prelude::{Result};
pub use crate::MetadataError;

pub fn check_bump(bump_name: &String, bumps: &BTreeMap<String, u8>, bump: u8) -> Result<()> {
    let expected_bump = *bumps
        .get(bump_name)
        .ok_or(MetadataError::MissingBump)?;

    if expected_bump != bump {
        return Err(MetadataError::InvalidBump.into());
    }
    Ok(())
}