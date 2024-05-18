
// pub mod create_inscription;
pub mod create_inscription_v2;
pub mod v3;
pub mod create_inscription_rank_page;
pub mod resize_inscription;
pub mod write_to_inscription;
pub mod make_inscription_immutable;
pub mod set_validation_hash;
pub mod claim_excess_rent;
pub mod migrate_to_v3;
pub mod ghost;


pub use migrate_to_v3::*;
pub use create_inscription_rank_page::*;
pub use make_inscription_immutable::*;
pub use resize_inscription::*;
pub use write_to_inscription::*;
// pub use create_inscription::*;
pub use create_inscription_v2::*;
pub use v3::*;
pub use claim_excess_rent::*;
pub use set_validation_hash::*;
pub use ghost::*;

