use anchor_lang::prelude::*;
use instructions::*;

declare_id!("L1BRc7ZYjj7t9k7E5xbdnKy3KhaY6sTcJx4gAsqxUbh");

pub mod instructions;
pub mod state;
pub mod constants;

pub use constants::*;

#[program]
pub mod libreplex {

    use super::*;


}
