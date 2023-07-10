use anchor_lang::prelude::*;

/// Do NOT reorder the errors in this enum. Tests are relying on error ordering.
/// Not great, but for some reason when ErrorCode is factored out into a lib,
/// test messages no longer print actual messages and print error codes instead.
///
/// The other alternative is to have a custom error type inside the common library
/// and to try to convert that -> ErrorCode -> ProgramError
/// Unfortunately I wasn't able to get that working, last leg is failing.
///
/// todo to revisit in v1
#[error_code]
pub enum ShopError {
    #[msg("Group has active filters")]
    GroupHasActiveFilters,

    #[msg("Unsupported filter type")]
    UnsupportFilterType,

    #[msg("Lister not allowed")]
    ListerNotAllowed,

    #[msg("Group not allowed")]
    GroupNotAllowed,


    #[msg("Group has active listings")]
    GroupHasActiveListings,

}
