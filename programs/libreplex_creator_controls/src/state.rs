use anchor_lang::prelude::*;

use crate::controls::{MAX_CONTROL_TYPE_SIZE, ControlType};

#[account]
pub struct CreatorController {
    pub seed: Pubkey,
    pub bump: u8,
    pub update_authority: Pubkey,
    pub creator: Pubkey,
    pub phases: Vec<Phase>,
}

impl CreatorController {
    pub const MAX_LABEL_SIZE: usize = 25;

    pub fn size_for_input(phases: &Vec<Phase>) -> usize {
        return 8 + 32 + 1 + 32 + 32 + 4 + phases.len() * (8 + 9 + CreatorController::MAX_LABEL_SIZE + MAX_CONTROL_TYPE_SIZE)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Phase {
    pub start: i64,
    pub end: Option<i64>,
    pub label: String,
    pub controls: Vec<ControlType>,
}

pub struct ArgCtx {
    pub args: Vec<Vec<u8>>,
    pub current: u32,
}


pub struct RemainingAccountsCtx<'b, 'info> {
    pub accounts: &'b [AccountInfo<'info>],
    pub current: u32,
}

pub struct Accounts<'b, 'info> {
    pub creator: AccountInfo<'info>,

    pub receiver: AccountInfo<'info>,
    pub receiver_token_account: AccountInfo<'info>,

    pub payer: AccountInfo<'info>,

    pub mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    
    pub mint_wrapper: AccountInfo<'info>,

    pub metadata: AccountInfo<'info>,
    pub group: AccountInfo<'info>,
    pub group_permissions: AccountInfo<'info>,

    pub attribute_config: Option<AccountInfo<'info>>,
    pub minter_numbers: Option<AccountInfo<'info>>,

    pub system_program:  AccountInfo<'info>,
    pub libreplex_nft_program: AccountInfo<'info>,
    pub libreplex_metadata_program: AccountInfo<'info>,
    pub recent_slothashes: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,

    pub remaining_accounts: RemainingAccountsCtx<'b, 'info>,
}