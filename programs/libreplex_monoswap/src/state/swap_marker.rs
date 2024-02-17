use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

// two of these are made when a mint is introduced
// to the system. allowing for two-way swaps
// PDA is always ["allowed_swap_marker", <swapper_program>, <incoming mint>]
// they can never be deleted so the swap links are permanent
#[account]
pub struct SwapMarker {
    // namespace can be anything that the caller can sign
    pub namespace: Pubkey,
    // allows slicing and dicing by incoming mint
    pub mint_incoming: Pubkey,
     // allows slicing and dicing by outgoing mint
    pub mint_outgoing: Pubkey,
    pub mint_incoming_amount: u64,
    pub mint_outgoing_amount: u64,
    // an unused marker can be closed.
    // after it has been used, it can not be
    // closed to avoid a situation where a
    // holder gets trapped into a crappy token
    // and cannot go back
    pub used: bool
}

impl SwapMarker {
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1;
}

