use anchor_lang::prelude::*;

pub const CREATOR_SIZE: usize = 32 + 1;

#[account]
pub struct RoyaltyShare {
    pub address: Pubkey,
    pub share: u8,
}

impl RoyaltyShare {
    pub const SIZE: usize = 8 + 32 + 1;
}

#[account]
pub struct Verification {}

impl Verification {
    pub const SIZE: usize = 8;
}

#[account]
pub struct Attribute {
    pub trait_type: String,
    pub attribute: String,
}

#[account]
pub struct Metadata {
    pub collection: Pubkey,
    pub mint: Pubkey,
    pub is_mutable: bool,
    pub bump: u8,
    pub offchain_url: String,
    pub name: String,
    pub token_type: u8, // 0 - spl token, 1 - NFT, 2 - anything else? TODO: Replace with enum if appropriate

    /*
       IMO it's better to have this here as an option instead of MetadataNFT pointing back to this.
       Reason is that way we can have a single metadata object and all the basic update methods (and
       delete etc) can be written once.

       If we have a separate Metadata NFT account, we need to sets of instructions for those same
       operations.

       Hence adding this as an optional extra. More dynamic would be a PDA but that would come with
       extra rent.

       To make this more generic we could consider a PDA depending on the metadata type. However
       that would incur extra rent compared to keeping things in line.  As things stand, an empty
       data account like this is a single byte.

       The only way to future-proof this for some unforeseen metadata types would be to have a PDA
       as when the contract goes immutable, we will be unable to reallocate.

       For context, the base solana account creation rent is 0.00089088. For a 10k collection, every
       new PDA costs ~9 SOL without considering the associated data. For the same rent cost we can
       squeeze in an additional 128 bytes of data if we keep it here in the same account.

       Trade-offs, trade-offs....
    */
    pub nft_data: Option<MetadataNft>,
}

#[account]
pub struct MetadataNft {
    pub attributes: Vec<Attribute>,
    pub signers: Vec<Pubkey>,
}

/*
   Typically the variables in the following PDA would be defined at collection level.

   However, this PDA exists because we want to allow customising royalties and permitted
   signers for individual NFTs.

   This PDA can be empty but the address needs to be passed into royalty-collecting instructions and
   signing instructions. If it is empty, royalty shares and permitted signers
   of the collection will be used.

   We make this a PDA as the expectation is that it will be used relatively infrequently
   compared to the standard use case, which is that royalties and permitted signers are the
   same across the collection. Hence we care less about the additional rent.
*/
#[account]
pub struct MetadataNftOverride {
    pub metadata_nft: Pubkey,
    pub royalty_bps_override: Option<u16>,
    pub royalties: Option<Vec<RoyaltyShare>>,
    pub permitted_signers: Option<Vec<Pubkey>>,
}

impl Metadata {
    // base size only, the size is set at init / reallocated at update based on
    // the lengths of the strings
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1 + 1;
}

impl MetadataNft {
    // base size only, the size is set at init / reallocated at update based on
    // the lengths of the strings
    pub const BASE_SIZE: usize = Metadata::BASE_SIZE + 1 + 2 + 1 + 4 + 4 + 1 + 1 + 1;
}

impl MetadataNftOverride {
    // base size only, the size is set at init / reallocated at update based on
    // the lengths of the strings
    pub const BASE_SIZE: usize = 8 + 32 + 1 + 1 + 1;
}

/* 
    Rationale for the existence of collection object -v- collection mint:
    1) creators are typically set on a per collection basis
    2) collection is not actually mint in that it should not be movable
        between wallets, have mastereditions / creators / attributes etc.
        hence leaving name + symbol + image url here as strings only
    3) dynamically resizes according to the length of the fields
    4) place creators at the beginning so they can be fetched via gPA
        (if really needed :D otherwise use helios.xyz or alchemy etc)
*/
#[account]
pub struct Collection {
    // collection authority
    pub authority: Pubkey,
    // collection name
    pub name: String,
    // collection symbol
    pub symbol: String,
    // TODO: See if this works
    pub item_count: u32, // ref counter
    pub url: String,
    pub bump: u8,

    /* 
        Same reasoning as above. All collections share functionality and we would like to write the logic once for these things. 
        All NFT-specific stuff falls under the following struct
    */
    pub nft_data: Option<CollectionNftData>,
}

impl Collection {
    pub const BASE_SIZE: usize = 8 + 32 + 36 + 4 + 4 + 2 + 1; // the creator + url portions are dynamically sized
}

#[account]
pub struct CollectionNftData {
    // NFT concepts
    pub royalty_bps: u16,
    pub royalty_shares: Vec<RoyaltyShare>,
    pub permitted_signers: Vec<Pubkey>,
}
