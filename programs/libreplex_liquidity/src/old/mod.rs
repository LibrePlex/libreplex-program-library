#[account]
#[derive(InitSpace)]
pub struct HybridRedeemer {
    pub redeem_start: i64,
    pub total_redeemed: u64,
    pub allocation_account: Pubkey,
    pub deployment: Pubkey,

    pub seed: Pubkey,
    pub bump: u8,

    pub padding: [u8; 100]
}

#[account]
#[derive(InitSpace)]
pub struct MyHybridRedeems {
    pub total: u32,
}

#[error_code]
pub enum HybridRedeemErrors {
    RedeemsNotStarted,
}