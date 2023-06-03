use anchor_lang::prelude::*;

use crate::{CreateOverride, RoyaltyShare};

pub fn handle_create_metadata_override(
    ctx: Context<CreateOverride>,
    royalty_bps_override: Option<u16>,
    royalties: Option<Vec<RoyaltyShare>>,
    permitted_signers: Option<Vec<Pubkey>>,
) -> Result<()> {
    
    let metadata_override = &mut ctx.accounts.metadata_override;
    let metadata_nft = &mut ctx.accounts.metadata_nft;
    metadata_override.metadata_nft = metadata_nft.key();

    metadata_override.royalty_bps_override = royalty_bps_override;
    metadata_override.royalties = royalties;
    metadata_override.permitted_signers = permitted_signers;
    
    Ok(())
}
