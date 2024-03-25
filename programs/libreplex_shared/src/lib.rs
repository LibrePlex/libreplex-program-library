use anchor_lang::prelude::*;

pub mod errors;
pub mod processor;

pub mod operations;


pub use errors::*;
pub use processor::*;

pub mod empty_account_placeholder {
    use anchor_lang::declare_id;
    declare_id!("11111111111111111111111111111111");
}

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}

pub mod wrapped_sol {
    use anchor_lang::declare_id;

    declare_id!("So11111111111111111111111111111111111111112");
}


declare_id!("insFmVukT9LYVygNbdpSjbxPy4FtQ6WgcuChnxDLbAm");

#[program]
pub mod libreplex_shared {}
