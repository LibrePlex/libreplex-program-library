use anchor_lang::prelude::*;

use crate::state::{RoyaltyShare};

// Can be separate PDA or be an optional field in NftMetadata Struct

#[repr(C)]
#[account]
#[derive()]
pub struct NftMetadataUnique {

    // The metadata account for which the additional unique data exists
    pub metadata: Pubkey,

    pub royalty_bps_override: Option<u16>,

    pub royalties_share_override: Option<Vec<RoyaltyShare>>,

    pub permitted_signers_override: Option<Vec<Pubkey>>,
}
