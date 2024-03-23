use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[account]
#[derive(InitSpace)]
pub struct NiftyMarker {
    pub namespace: Pubkey,
    // The Nifty asset associated with this marker
    pub asset: Pubkey,
    // The mint associated with this marker
    pub mint: Pubkey,
    // The state of the marker indicating which type of asset is currently escrowed
    pub state: MarkerState,
    // The amount of the fungible token escrowed
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, InitSpace)]
pub enum MarkerState {
    Uninitialized,
    AssetEscrowed,
    FungibleEscrowed,
}
