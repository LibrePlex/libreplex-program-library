use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

pub mod state;
pub mod errors;

pub use state::*;

#[program]
pub mod libreplex_fair_launch {

    use super::*;

   // v2 endpoints. Prefer these over the original ones

   pub fn deploy_legacy_v2(
    ctx: Context<DeployLegacyV2Ctx>,
    input: DeployLegacyV2Input
    ) -> Result<()> {
        instructions::deploy_legacy_v2::deploy_legacy_v2(
            ctx,
            input
        )
   }   

   pub fn deploy_legacy(
        ctx: Context<DeployLegacyCtx>,
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

    pub fn mint_legacy(
        ctx: Context<MintLegacyCtx>,
    ) -> Result<()> {
        instructions::mint_legacy::mint_legacy(
            ctx
        )
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




}
