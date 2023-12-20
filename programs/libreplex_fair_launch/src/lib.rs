use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

pub mod state;
pub mod errors;

pub use state::*;

#[program]
pub mod libreplex_fair_launch {

    use crate::instruction::SwapFungibleToCompressed;

    use super::*;

   // v2 endpoints. Prefer these over the original ones. they allow setting of optional creator co-signer 
   // and toggling inscriptions on and off.
   pub fn deploy_legacy_v2(
    ctx: Context<DeployLegacyV2Ctx>
    ) -> Result<()> {
        instructions::deploy_legacy_v2::deploy_legacy_v2(
            ctx
        )
   }   

   pub fn mint_legacy_v2<'info>(ctx: Context<'_, '_, '_, 'info, MintLegacyV2Ctx<'info>>) -> Result<()> {
    instructions::mint_legacy_v2::mint_legacy_v2(
        ctx
    )
    }  

    pub fn initialise_v2(
        ctx: Context<InitialiseV2Ctx>,
        input: InitialiseInputV2
    ) -> Result<()> {
        instructions::initialise_v2::initialise_v2(
            ctx,
            input
        )
    }   

   pub fn deploy_legacy<'f>(
        ctx: Context<'_,'_,'_,'f,DeployLegacyCtx<'f>>,
    ) -> Result<()> {
        instructions::deploy_legacy::deploy(
            ctx
        )
    }   
    

    pub fn initialise(
        ctx: Context<InitialiseCtx>,
        input: InitialiseInput
    ) -> Result<()> {
        instructions::initialise::initialise(
            ctx,
            input
        )
    }   

    pub fn mint_legacy<'info>(ctx: Context<'_, '_, '_, 'info, MintLegacyCtx<'info>>) -> Result<()> {
        instructions::mint_legacy::mint_legacy(
            ctx
        )
    }   


    pub fn mint_compressed(ctx: Context<MintCompressedCtx>, input: MintCompressedInput) -> Result<()> {
        instructions::mint_c_legacy(ctx, input)
    }

    /* 
        Migration methods - to be deactivated once old validation migrations are complete
     */
    pub fn deploy_migrated(
        ctx: Context<DeployMigratedCtx>,
    ) -> Result<()> {
        instructions::deploy_migrated::deploy_migrated(
            ctx
        )
    }   
    
    // pub fn migrate_from_validator(
    //     ctx: Context<MigrateFromValidatorCtx>,
    //     input: InitialiseInput,
    //     validated_token_count: u64
    // ) -> Result<()> {
    //     instructions::migrate_from_validator::migrate_from_validator(
    //         ctx,
    //         input,
    //         validated_token_count
    //     )
    // }   

    pub fn migrate_to_hashlist(
        ctx: Context<MigrateToHashlistCtx>
    ) -> Result<()> {
        instructions::migrate_to_hashlist::migrate_to_hashlist(
            ctx
        )
    }   

    pub fn swap_to_fungible(
        ctx: Context<SwapToFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_fungible::swap_to_fungible(
            ctx
        )
    }   


    pub fn swap_to_nonfungible(
        ctx: Context<SwapToNonFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_nonfungible::swap_to_nonfungible(
            ctx
        )
    }   

    pub fn swap_fungible_to_compressed<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, SwapFungibleToCompressedCtx<'info>>, 
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32
    ) -> Result<()> {
        instructions::swap_fungible_to_compressed(ctx, root, data_hash, creator_hash, nonce, index)
    }

    pub fn swap_compressed_to_fungible<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, SwapCompressedToFungibleCtx<'info>>, 
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32
    ) -> Result<()> {
            instructions::swap_compressed_to_fungible(ctx, root, data_hash, creator_hash, nonce, index)
    }




}
