use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
};
use libreplex_shared::operations::transfer_non_pnft;

use crate::{Deployment, HashlistMarker};

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

pub fn move_fungible_out_of_escrow<'a>(
    token_program: &AccountInfo<'a>,
    fungible_source_token_account: &Account<'a, TokenAccount>,
    fungible_target_token_account: &UncheckedAccount<'a>,
    deployment: &mut Account<'a, Deployment>,
    fungible_mint: &Account<'a, Mint>,
    payer: &Signer<'a>,
    associated_token_program: &Program<'a, AssociatedToken>,
    system_program: &Program<'a, System>,
    authority_seeds: &[&[u8]; 3],
    hashlist_marker: &HashlistMarker,
) -> Result<()> {
    transfer_non_pnft(
        &token_program.to_account_info(),
        &fungible_source_token_account.to_account_info(),
        &fungible_target_token_account.to_account_info(),
        &deployment.to_account_info(),
        &fungible_mint.to_account_info(),
        &payer.to_account_info(),
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
        Some(&[authority_seeds]),
        &payer.to_account_info(),
        deployment.get_fungible_mint_amount(hashlist_marker),
    )?;
    deployment.escrow_non_fungible_count += 1;
    Ok(())
}
