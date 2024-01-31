
/* 
    initialises a new launch. does not create any 
    on-chain accounts, mints, token accounts etc 
*/
pub mod initialise;
pub use initialise::*;


/* 
    deploys - i.e created mints, token accounts,
    and deploy inscription
 */

pub mod deploy_legacy;
pub use deploy_legacy::*;



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





/* 
    Migration methods from old validators to Libre Fair Launch
*/
// pub mod migrate_from_validator;
// pub use migrate_from_validator::*;
// no more legacy validators will be deployed to fair launch
// pub mod deploy_migrated;
// pub use deploy_migrated::*;

pub mod migrate_to_hashlist;
pub use migrate_to_hashlist::*;

pub mod add_to_hashlist;
pub use add_to_hashlist::*;

pub mod v2;
pub use v2::*;


pub const COMPRESSED_DEPLOYMENT_TYPE: u8 = 2;
pub const STANDARD_DEPLOYMENT_TYPE: u8 = 0;
pub const TOKEN2022_DEPLOYMENT_TYPE: u8 = 3;
pub const HYBRID_DEPLOYMENT_TYPE: u8 = 4; // creates token-2022 NFTs and metaple standard SPLs