use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

pub mod errors;
pub mod state;
pub use state::*;

pub mod state_v2;
pub use state_v2::*;

#[program]
pub mod libreplex_fair_launch {

    use super::*;

    pub fn initialise_v3(ctx: Context<InitialiseV3Ctx>, input: InitialiseInputV3) -> Result<()> {
        instructions::initialise_v3(ctx, input)
    }

    pub fn initialise_raw(ctx: Context<InitialiseRawCtx>, input: InitialiseRawInput) -> Result<()> {
        instructions::initialise_raw(ctx, input)
    }

    pub fn deploy_raw(ctx: Context<DeployRawCtx>) -> Result<()> {
        instructions::deploy_raw(ctx)
    }

    pub fn deploy_token22(ctx: Context<DeployToken2022Ctx>) -> Result<()> {
        instructions::deploy_token_2022(ctx)
    }

    pub fn toggle_freeze(ctx: Context<ToggleFreeze>) -> Result<()> {
        instructions::toggle_freeze(ctx)
    }

    // deploy hybrid - it's like token 2022 but with an extra metaplex
    // metadata for the FUNGIBLE mint only
    pub fn deployhybrid(ctx: Context<DeployHybridCtx>) -> Result<()> {
        instructions::deploy_hybrid(ctx)
    }

    pub fn deploy_hybrid_unchecked(ctx: Context<DeployHybridUncheckedCtx>) -> Result<()> {
        deploy_hybrid_unchecked_handler(ctx)
    }

    pub fn relinquish_cosigner(ctx: Context<RelinquishCosignersCtx>) -> Result<()> {
        instructions::relinquish_cosigner(ctx)
    }

    // some of the early token-2022 launches had "" as symbol instead of the ticker.
    // this is a throwback to metaplex metadata where symbol is limited to 10 characters
    // whereas there are no limits on the ticker size

    // this method works because metadata update authority is retain until token-metadata-2022
    // groups roll out. the plan is to include all generated token-2022 launches in groups
    // and for that you need the update auth too

    // incidentally the update auth can be used to update the symbol here as well from ""
    // to the ticker as token-2022 metadata has no limitations on the size of the symbol
    pub fn update_symbol22<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateSymbol2022Ctx<'info>>,
    ) -> Result<()> {
        instructions::update_symbol2022(ctx)
    }

    pub fn join<'info>(
        ctx: Context<'_, '_, '_, 'info, JoinCtx<'info>>,
        input: MintInput,
    ) -> Result<()> {
        instructions::join_handler(ctx, input)
    }

    pub fn joinraw<'info>(
        ctx: Context<'_, '_, '_, 'info, JoinRawCtx<'info>>,
        input: MintInput,
    ) -> Result<()> {
        instructions::joinraw_handler(ctx, input)
    }

    pub fn update_spl_metadata<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateSplMetadata2022Ctx<'info>>,
        new_uri: String,
    ) -> Result<()> {
        instructions::update_spl_metadata2022(ctx, new_uri)
    }
    pub fn switch_deployment_type<'info>(
        ctx: Context<'_, '_, '_, 'info, SwitchDeploymentTypeCtx<'info>>,
        deployment_type: u8,
    ) -> Result<()> {
        instructions::switch_deployment_type(ctx, deployment_type)
    }

    pub fn reduce_mint_count<'info>(
        ctx: Context<'_, '_, '_, 'info, ReduceMintCountCtx<'info>>,
        input: ReduceMintCountInput,
    ) -> Result<()> {
        instructions::reduce_mint_count(ctx, input)
    }

    pub fn burn_excess_spl<'info>(
        ctx: Context<'_, '_, '_, 'info, BurnExcessSplCtx<'info>>,
    ) -> Result<()> {
        instructions::burn_excess_spl(ctx)
    }

    pub fn mint_token22<'info>(
        ctx: Context<'_, '_, '_, 'info, MintToken2022Ctx<'info>>,
        input: MintInput,
    ) -> Result<()> {
        instructions::mint_token2022(ctx, input)
    }
    pub fn swap_to_fungible22<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToFungible2022Ctx<'info>>,
    ) -> Result<()> {
        instructions::swap_to_fungible_2022(ctx)
    }

    pub fn swap_to_nonfungible22<'a>(
        ctx: Context<'_, '_, '_, 'a, SwapToNonFungible2022Ctx<'a>>,
    ) -> Result<()> {
        instructions::swap_to_nonfungible_2022(ctx)
    }

    pub fn mint_legacy<'info>(ctx: Context<'_, '_, '_, 'info, MintLegacyCtx<'info>>) -> Result<()> {
        instructions::mint_legacy::mint_legacy(ctx)
    }

    pub fn claim_transfer_fee_auth_as_creator<'info>(
        ctx: Context<'_, '_, '_, 'info, ClaimTransferFeeAuthAsCreatorCtx<'info>>,
        sources: Vec<Pubkey>
    ) -> Result<()> {
        instructions::claim_transfer_fee_as_creator::claim_transfer_fee_as_creator_handler(
            ctx,
            sources
        )
    }
    
    /*
       Migration methods - to be deactivated once old validation migrations are complete
    */
    // pub fn deploy_migrated(ctx: Context<DeployMigratedCtx>) -> Result<()> {
    //     instructions::deploy_migrated::deploy_migrated(ctx)
    // }

    pub fn migrate_to_hashlist(ctx: Context<MigrateToHashlistCtx>) -> Result<()> {
        instructions::migrate_to_hashlist::migrate_to_hashlist(ctx)
    }

    /* v1 swap methods */
    pub fn swap_to_fungible(ctx: Context<SwapLegacyToFungibleCtx>) -> Result<()> {
        instructions::swap_metaplex_to_fungible(ctx)
    }

    pub fn swap_to_nonfungible(ctx: Context<SwapFungibleToLegacyCtx>) -> Result<()> {
        instructions::swap_to_nonfungible(ctx)
    }
}
