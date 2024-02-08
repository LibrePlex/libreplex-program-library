use anchor_lang::prelude::*;

anchor_gen::generate_cpi_crate!("amm.json");

declare_id!("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB");

pub fn get_first_key(key1: Pubkey, key2: Pubkey) -> Pubkey {
    if key1 > key2 {
        return key1;
    }
    key2
}
/// get second key
pub fn get_second_key(key1: Pubkey, key2: Pubkey) -> Pubkey {
    if key1 > key2 {
        return key2;
    }
    key1
}

/// get curve type
pub fn get_curve_type(curve_type: CurveType) -> u8 {
    match curve_type {
        CurveType::ConstantProduct {} => 0,
        _ => 1,
    }
}

  /// Trade fee numerator for constant product swap curve.
    // 25bps, https://docs.uniswap.org/protocol/V2/concepts/advanced-topics/fees
    pub static CONSTANT_PRODUCT_TRADE_FEE_NUMERATOR: u64 = 250;

    /// Trade fee numerator for stable swap curve.
    // 1bps, https://curve.fi/rootfaq
    pub static STABLE_SWAP_TRADE_FEE_NUMERATOR: u64 = 10;

    /// Admin trade fee numerator for constant product swap curve.
    // 5bps, https://docs.uniswap.org/protocol/V2/concepts/advanced-topics/fees
    pub static CONSTANT_PRODUCT_ADMIN_TRADE_FEE_NUMERATOR: u64 = 50;

    /// Admin trade fee numerator for stable swap curve.
    // 2bps, https://curve.fi/rootfaq
    pub static STABLE_SWAP_ADMIN_TRADE_FEE_NUMERATOR: u64 = 5;

    /// Host trade fee numerator
    // 20% of admin trade fee
    pub static HOST_TRADE_FEE_NUMERATOR: u64 = 20000;

    /// Default fee denominator
    pub static FEE_DENOMINATOR: u64 = 100000;
    /// Max fee BPS
    pub static MAX_FEE_DBPS: u64 = 600; // 6%
    /// Max basis point. 100% in pct
    pub static MAX_BASIS_POINT: u64 = 10000;

/// Convert fees numerator and denominator to BPS. Minimum 1 bps, Maximum 10_000 bps. 0.01% -> 100%
pub fn to_bps(numerator: u128, denominator: u128) -> Option<u64> {
    let bps = numerator
        .checked_mul(MAX_BASIS_POINT.into())?
        .checked_div(denominator)?;
    bps.try_into().ok()
}

/// Get default fee settings
pub fn get_default_fee(curve_type: &CurveType) -> PoolFees {
    match curve_type {
        CurveType::ConstantProduct {} => PoolFees {
            trade_fee_numerator: CONSTANT_PRODUCT_TRADE_FEE_NUMERATOR,
            trade_fee_denominator: FEE_DENOMINATOR,
            owner_trade_fee_numerator: CONSTANT_PRODUCT_ADMIN_TRADE_FEE_NUMERATOR,
            owner_trade_fee_denominator: FEE_DENOMINATOR,
        },
        CurveType::Stable { .. } => PoolFees {
            trade_fee_numerator: STABLE_SWAP_TRADE_FEE_NUMERATOR,
            trade_fee_denominator: FEE_DENOMINATOR,
            owner_trade_fee_numerator: STABLE_SWAP_ADMIN_TRADE_FEE_NUMERATOR,
            owner_trade_fee_denominator: FEE_DENOMINATOR,
        },
    }
}


/// get trade fee bps seed for pool pda
pub fn get_trade_fee_bps_bytes(curve_type: CurveType, trade_fee_bps: u64) -> Option<Vec<u8>> {
    let default_fees = get_default_fee(&curve_type);

    let default_trade_fee_bps = to_bps(
        default_fees.trade_fee_numerator.into(),
        default_fees.trade_fee_denominator.into(),
    )?;

    if default_trade_fee_bps == trade_fee_bps {
        return Some(vec![]);
    }

    Some(trade_fee_bps.to_le_bytes().to_vec())
}

pub fn derive_permissionless_pool_with_fee_tier(
    curve_type: CurveType,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    trade_fee_bps: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &get_curve_type(curve_type).to_le_bytes(),
            get_first_key(token_a_mint, token_b_mint).as_ref(),
            get_second_key(token_a_mint, token_b_mint).as_ref(),
            get_trade_fee_bps_bytes(curve_type, trade_fee_bps)
                .unwrap()
                .as_ref(),
        ],
        &crate::ID,
    )
}