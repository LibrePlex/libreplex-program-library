
/* 
    initialises a new launch. does not create any 
    on-chain accounts, mints, token accounts etc 
*/
pub mod initialise_logic;
pub use initialise_logic::*;


// this can be done before redeploy
pub mod switch_deployment_type;
pub use switch_deployment_type::*;

/* 
    mints from a legacy metadata-based deployment.
    This generates a non-fungible into the minter's wallet
    and a corresponding amount of fungible into the
    bridge escrow
 */
pub mod mint_legacy;
pub use mint_legacy::*;

pub mod mint_legacy_logic;
pub use mint_legacy_logic::*;

pub mod migrate_to_hashlist;
pub use migrate_to_hashlist::*;

pub mod add_to_hashlist;
pub use add_to_hashlist::*;

pub use swap_to_fungible::*;
pub mod swap_to_fungible;

pub use swap_to_non_fungible::*;
pub mod swap_to_non_fungible;

pub mod v2;
pub use v2::*;

pub mod raw;
pub use raw::*;

pub mod toggle_freeze;
pub use toggle_freeze::*;


pub const COMPRESSED_DEPLOYMENT_TYPE: u8 = 2;
pub const STANDARD_DEPLOYMENT_TYPE: u8 = 0;
pub const TOKEN2022_DEPLOYMENT_TYPE: u8 = 3;
pub const HYBRID_DEPLOYMENT_TYPE: u8 = 4; // creates token-2022 NFTs and metaple standard SPLs