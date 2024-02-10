use anchor_lang::prelude::*;

declare_id!("Pipe6YuqZmoHeKTpwETFaZEiALNREGfZqCjMbk9P4UG");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::*;

pub use constants::*;
pub use state::*;
pub use instructions::*;

#[program]
pub mod libreplex_pipelines {
    

}
