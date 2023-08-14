use anchor_lang::prelude::*;

#[account]
pub struct WrappedMint {}

impl WrappedMint {
    pub const LEN: usize = 50;
}