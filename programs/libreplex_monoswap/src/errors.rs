use anchor_lang::prelude::*;

#[error_code]
pub enum MonoSwapError {
    #[msg("Invalid Marker State")]
    InvalidMarkerState,
}
