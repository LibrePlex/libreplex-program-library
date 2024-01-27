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

    // v2 endpoints. Prefer these over the original ones. 
    // they allow setting of optional creator co-signer
    // and toggling inscriptions on and off. 
    // for now, creator co-sign is disabled but will be enabled
    // soon to allow for wrapper contracts
    pub fn initialise_v2(ctx: Context<InitialiseV2Ctx>, input: InitialiseInputV2) -> Result<()> {
        instructions::initialise_v2(ctx, input)
    }

    // allows for deploying of cNFTs - to be enabled once tested 
    // pub fn deploy_v2(ctx: Context<DeployLegacyV2Ctx>) -> Result<()> {
    //     instructions::deploy_v2(ctx)
    // }

    // deploy token 2022 - token 2022 uses Token 2022 including native metadata 
    // pub fn deploy_token22(ctx: Context<DeployToken2022Ctx>) -> Result<()> {
    //     instructions::deploy_token_2022(ctx)
    // }

    // deploy hybrid - it's like token 2022 but with an extra metaplex
    // metadata for the FUNGIBLE mint only
    pub fn deployhybrid(ctx: Context<DeployHybridCtx>) -> Result<()> {
        instructions::deploy_hybrid(ctx)
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
   


    pub fn mint_token22<'info>(
        ctx: Context<'_, '_, '_, 'info, MintToken2022Ctx<'info>>,
    ) -> Result<()> {
        instructions::mint_token2022(ctx)
    }


    // disabling these for now - needs proper testing
    // pub fn swap_fungible_to_compressed<'info>(
    //     ctx: Context<'_, '_, '_, 'info, SwapFungibleToCompressedCtx<'info>>,
    //     root: [u8; 32],
    //     data_hash: [u8; 32],
    //     creator_hash: [u8; 32],
    //     nonce: u64,
    //     index: u32,
    // ) -> Result<()> {
    //     instructions::v2::metaplex_cnft::swap_fungible_to_compressed(ctx, root, data_hash, creator_hash, nonce, index)
    // }

    // pub fn swap_compressed_to_fungible<'info>(
    //     ctx: Context<'_, '_, '_, 'info, SwapCompressedToFungibleCtx<'info>>,
    //     root: [u8; 32],
    //     data_hash: [u8; 32],
    //     creator_hash: [u8; 32],
    //     nonce: u64,
    //     index: u32,
    // ) -> Result<()> {
    //     instructions::v2::metaplex_cnft::swap_compressed_to_fungible(ctx, root, data_hash, creator_hash, nonce, index)
    // }

    
    pub fn swap_to_fungible22(ctx: Context<SwapToFungible2022Ctx>) -> Result<()> {
        instructions::swap_to_fungible_2022(ctx)
    }

    pub fn swap_to_nonfungible22(ctx: Context<SwapToNonFungible2022Ctx>) -> Result<()> {
        instructions::swap_to_nonfungible_2022(ctx)
    }
    

       
    // pub fn mint_compressed<'info>(
    //     ctx: Context<'_, '_, '_, 'info, MintCompressedCtx<'info>>,
    //     input: MintCompressedInput,
    // ) -> Result<()> {
    //     instructions::mint_compressed(ctx, input)
    // }

    // pub fn redeem_compressed(
    //     ctx: Context<InscribeCompressedCtx>,
    // ) -> Result<()> {
    //     instructions::redeem(ctx)
    // }


    /* v1 swap methods */




    

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
