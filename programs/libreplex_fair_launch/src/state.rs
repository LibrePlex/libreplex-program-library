use anchor_lang::{prelude::*, Discriminator};
use solana_program::pubkey::Pubkey;

pub const TICKER_LIMIT: usize = 200;
pub const TEMPLATE_LIMIT: usize = 1200;
pub const OFFCHAIN_URL_LIMIT: usize = 1200;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum DeploymentStatus {
    Initialised,
    Deployed,
    MintedOut,
}

#[account]
#[derive(InitSpace)]
pub struct Deployment {
    // creator has two purposes: 
    // 1) to deploy: deployment must be performed by the same wallet that initialises
    // the launch
    // 2) (optionally) to allow for a third party
    // service to add their own logic on top of fair 
    // launch by co-signing
    
    pub creator: Pubkey,

    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,

    pub number_of_tokens_issued: u64,
    pub decimals: u8,


    pub use_inscriptions: bool,
    pub deployment_type: u8,
    // to allow modular custom logic around this contract
    pub require_creator_cosign: bool,

    // indicates whether this deployment was migrated from legacy validator
    // true - from legacy
    // false - created directly via libreplex fair launch
    pub migrated_from_legacy: bool,

    // this is used to sanity check that
    // whenever swaps occur, to the maount
    // of fungible and non-fungible in the
    // escrow always remains equal to the total
    // supply.
    pub escrow_non_fungible_count: u64,

    #[max_len(TICKER_LIMIT)]
    pub ticker: String,

    #[max_len(TEMPLATE_LIMIT)]
    pub deployment_template: String,

    #[max_len(TEMPLATE_LIMIT)]
    pub mint_template: String,

    pub fungible_mint: Pubkey, // starts as 111111111111...

    #[max_len(OFFCHAIN_URL_LIMIT)]
    pub offchain_url: String, // pub padding: Vec<u8, EXCESS>
}

#[account]
#[derive(InitSpace)]
pub struct DeploymentConfig {
    pub deployment: Pubkey,
    // defined by creator. this is NOT a libreplex fee as libreplex charges no fees.
    pub creator_fee_treasury: Pubkey,
    pub creator_fee_per_mint_lamports: u64,
    pub deflation_rate_per_swap: u16, // in basis points
    // makes it easier to identify the program / endpoints that need to be called
    pub cosigner_program_id: Pubkey,
    pub multiplier_upper_limit: Option<u16>,
}

impl DeploymentConfig {
    /// leave a bit of extra space at the end in case more config is needed
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 500;
}


#[event]
pub struct NewDeploymentEvent {
    pub ticker: String,
    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
    pub creator: Pubkey,
}

#[event]
pub struct NewDeploymentV2 {
    pub ticker: String,
    pub limit_per_mint: u64,
    pub max_number_of_tokens: u64,
    pub creator: Pubkey,
    pub off_chain_url: String,
    pub require_co_sign: bool,
    pub deployment_template: String,
    pub mint_template: String,
    pub uses_inscriptions: bool,
    pub decimals: u8,
    pub deployment_type: u8,
    pub config: Option<DeploymentConfig>,
}

#[event]
pub struct DeploymentActive {
    pub ticker: String,
    pub fungible_mint: Pubkey,
}

#[event]
pub struct MintEvent {
    pub mint: Pubkey,
    pub ticker: String,
    pub tokens_minted: u64,
    pub max_number_of_tokens: u64,
}

impl Deployment {
    pub fn get_fungible_mint_amount(&self) -> u64 {
        self.limit_per_mint
            .checked_mul(10_u64.checked_pow(self.decimals as u32).unwrap())
            .unwrap()
    }

    pub fn get_max_fungible_mint_amount(&self) -> u64 {
        self.max_number_of_tokens
            .checked_mul(self.limit_per_mint)
            .unwrap()
            .checked_mul(10_u64.checked_pow(self.decimals as u32).unwrap())
            .unwrap()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintAndOrder {
    pub mint: Pubkey,
    pub order: u64,
}

// this is a genuine hashlist for the launch
#[account]
pub struct Hashlist {
    pub deployment: Pubkey,
    pub issues: Vec<MintAndOrder>,
}

// Each mint can only be migrated once
#[account]
pub struct MigrationMarker {}

#[derive(Clone, InitSpace)]
pub struct HashlistMarker {
    pub multiplier_numerator: u16,
    pub multiplier_denominator: u16,
}

impl Discriminator for HashlistMarker {
    const DISCRIMINATOR: [u8; 8] = [55, 46, 160, 53, 239, 41, 223, 50];
}

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for HashlistMarker {

}

impl anchor_lang::AccountSerialize for HashlistMarker {
   fn try_serialize<W: std::io::prelude::Write>(&self, writer: &mut W) -> Result<()> {
    if writer.write_all(&HashlistMarker::discriminator()).is_err() {
        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
    }
    if AnchorSerialize::serialize(&self.multiplier_numerator, writer).is_err() {
        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
    }

    if AnchorSerialize::serialize(&self.multiplier_denominator, writer).is_err() {
        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
    }
    Ok(())
   }
}

impl anchor_lang::Owner for HashlistMarker {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl anchor_lang::AccountDeserialize for HashlistMarker {
 fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
    if buf.len() < HashlistMarker::discriminator().len() {
        return Err(
            anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
        );
    }

    let given_disc = &buf[..8];
    if &HashlistMarker::discriminator() != given_disc {
        return Err(
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .name(),
                    error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .into(),
                    error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/libreplex_fair_launch/src/state.rs",
                            line: 157u32,
                        }),
                    ),
                    compared_values: None,
                })
                .with_account_name("HashlistMarker"),
        );
    }

    Self::try_deserialize_unchecked(buf)
 }

 fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
    let mut data: &[u8] = &buf[8..];

    if data.len() == 0 {
        return Ok(Self {
            multiplier_denominator: 1,
            multiplier_numerator: 1,
        })
    }

    Ok(Self {
        multiplier_numerator: AnchorDeserialize::deserialize_reader(&mut data)?,
        multiplier_denominator: AnchorDeserialize::deserialize_reader(&mut data)?,
    })
 }
}

#[account]
pub struct MigrationCounter {
    pub deployment: Pubkey,
    pub migration_count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Redeemable {
    pub asset: Pubkey,
    pub deployment: Pubkey,
}
