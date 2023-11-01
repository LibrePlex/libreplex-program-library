
pub mod create_inscription;
pub mod create_inscription_rank_page;
pub mod delete_inscription;
pub mod resize_inscription;
pub mod write_to_inscription;
pub mod make_inscription_immutable;

pub use create_inscription_rank_page::*;
pub use make_inscription_immutable::*;
pub use delete_inscription::*;
pub use resize_inscription::*;
pub use write_to_inscription::*;
pub use create_inscription::*;
