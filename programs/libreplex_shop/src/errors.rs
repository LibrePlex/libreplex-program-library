use anchor_lang::prelude::*;

#[error_code]
pub enum ShopError {
    #[msg("Collection has active filters")]
    CollectionHasActiveFilters,

    #[msg("Unsupported filter type")]
    UnsupportFilterType,

    #[msg("Lister not allowed")]
    ListerNotAllowed,

    #[msg("Collection not allowed")]
    CollectionNotAllowed,


    #[msg("Collection has active listings")]
    CollectionHasActiveListings,

}
