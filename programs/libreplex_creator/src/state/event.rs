use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum AccountEventType {
    Create,
    Update,
    Delete
}

#[event]
pub struct AccountEvent {
    pub reference: Pubkey,
    pub event_type: AccountEventType
}