use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

pub mod errors;
pub mod state;

pub use state::*;

#[program]
pub mod libreplex_fair_launch {
    
    use super::*;

    // v2 endpoints. Prefer these over the original ones. they allow setting of optional creator co-signer
    // and toggling inscriptions on and off.
    pub fn deploy_v2(ctx: Context<DeployLegacyV2Ctx>) -> Result<()> {
        instructions::deploy_v2(ctx)
    }

    pub fn initialise_v2(ctx: Context<InitialiseV2Ctx>, input: InitialiseInputV2) -> Result<()> {
        instructions::initialise_v2(ctx, input)
    }

    pub fn mint_metaplex<'info>(
        ctx: Context<'_, '_, '_, 'info, MintMetaplexStandardCtx<'info>>,
    ) -> Result<()> {
        instructions::mint_metaplex(ctx)
    }


    pub fn mint_compressed<'info>(
        ctx: Context<'_, '_, '_, 'info, MintCompressedCtx<'info>>,
        input: MintCompressedInput,
    ) -> Result<()> {
        instructions::mint_compressed(ctx, input)
    }

    pub fn redeem_compressed(
        ctx: Context<InscribeCompressedCtx>,
    ) -> Result<()> {
        instructions::redeem(ctx)
    }



    

    pub fn deploy_legacy<'f>(ctx: Context<'_, '_, '_, 'f, DeployLegacyCtx<'f>>) -> Result<()> {
        instructions::deploy_legacy::deploy(ctx)
    }

    pub fn initialise(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
        instructions::initialise::initialise(ctx, input)
    }

    pub fn mint_legacy<'info>(ctx: Context<'_, '_, '_, 'info, MintLegacyCtx<'info>>) -> Result<()> {
        instructions::mint_legacy::mint_legacy(ctx)
    }


    /*
       Migration methods - to be deactivated once old validation migrations are complete
    */
    pub fn deploy_migrated(ctx: Context<DeployMigratedCtx>) -> Result<()> {
        instructions::deploy_migrated::deploy_migrated(ctx)
    }

    pub fn migrate_to_hashlist(ctx: Context<MigrateToHashlistCtx>) -> Result<()> {
        instructions::migrate_to_hashlist::migrate_to_hashlist(ctx)
    }

    pub fn swap_to_fungible(ctx: Context<SwapLegacyToFungibleCtx>) -> Result<()> {
        instructions::swap_metaplex_to_fungible(ctx)
    }

    pub fn swap_to_nonfungible(ctx: Context<SwapFungibleToLegacyCtx>) -> Result<()> {
        instructions::swap_to_nonfungible(ctx)
    }

    pub fn swap_fungible_to_compressed<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapFungibleToCompressedCtx<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        instructions::v2::metaplex_cnft::swap_fungible_to_compressed(ctx, root, data_hash, creator_hash, nonce, index)
    }

    pub fn swap_compressed_to_fungible<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapCompressedToFungibleCtx<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        instructions::v2::metaplex_cnft::swap_compressed_to_fungible(ctx, root, data_hash, creator_hash, nonce, index)
    }
}
