
use anchor_lang::prelude::*;
use bubblegum_proxy::{TreeConfig, MetadataArgs};
// use libreplex_shared::sysvar_instructions_program;
use bubblegum::utils::get_asset_id;


use crate::{
    errors::FairLaunchError, Deployment, COMPRESSED_DEPLOYMENT_TYPE, Redeemable,
};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum TreeDelegateType {
    Global,
    Deployment,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MintCompressedInput {
    pub tree_delegate_type: TreeDelegateType
}


#[derive(Accounts)]
#[instruction(input: MintCompressedInput)]
pub struct MintCompressedCtx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + Redeemable::INIT_SPACE)]
    pub redeem: Account<'info, Redeemable>,

    /// CHECK: Can be anything
    pub nft_receiver: UncheckedAccount<'info>,

    /// CHECK: checked in cpi
    account_compression_program: AccountInfo<'info>,

    /// CHECK: checked in cpi
    noop_program: AccountInfo<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(mut)]
    pub tree_authority: Account<'info, TreeConfig>,

    /// CHECK: Checked by address has no data
    #[account(seeds = [b"global_tree_delegate"], bump)]
    pub global_tree_delegate: Option<UncheckedAccount<'info>>,

    /// CHECK: checked by address
    #[account(address = bubblegum::id())]
    bubblegum_program: AccountInfo<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn mint_compressed<'info>(ctx: Context<'_, '_, '_, 'info, 
    MintCompressedCtx<'info>>, input: MintCompressedInput) -> Result<()> {
    let deployment = &ctx.accounts.deployment;

    if deployment.deployment_type != COMPRESSED_DEPLOYMENT_TYPE {
        return Err(FairLaunchError::IncorrectMintType.into())
    }


    let tree_authority = &ctx.accounts.tree_authority;
    let merkle_tree = &ctx.accounts.merkle_tree;
   
    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    let global_tree_delegate_seeds: &[&[u8]] 
    = &[b"global_tree_delegate", &[ctx.bumps.global_tree_delegate]];


    let (tree_delegate_seeds, tree_delegate_info) = if let TreeDelegateType::Global  = input.tree_delegate_type {
        (global_tree_delegate_seeds, ctx.accounts.global_tree_delegate
            .as_ref().ok_or(FairLaunchError::MissingGlobalTreeDelegate)?.to_account_info())
    } else {
        (deployment_seeds, deployment.to_account_info())
    };

    let nft_receiver = &ctx.accounts.nft_receiver;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;

    let asset_id = get_asset_id(merkle_tree.key, tree_authority.num_minted);

    let mint_compressed_accounts = bubblegum_proxy::cpi::accounts::MintV1 {
        compression_program: ctx.accounts.account_compression_program.to_account_info(),
        tree_authority: ctx.accounts.tree_authority.to_account_info(),
        leaf_owner: nft_receiver.to_account_info(),
        leaf_delegate: nft_receiver.to_account_info(),
        merkle_tree: merkle_tree.to_account_info(),
        payer: payer.to_account_info(),
        tree_delegate: tree_delegate_info,
        log_wrapper: ctx.accounts.noop_program.to_account_info(),
        system_program: system_program.to_account_info(),
    };

    let metadata_args = MetadataArgs {
        name: deployment.ticker.clone(),
        symbol: "".to_string(),
        uri: deployment.offchain_url.clone(),
        seller_fee_basis_points: 0,
        primary_sale_happened: true,
        is_mutable: false,
        edition_nonce: None,
        token_standard: Some(bubblegum_proxy::TokenStandard::NonFungible),
        collection: None,
        uses: None,
        token_program_version: bubblegum_proxy::TokenProgramVersion::Original,
        creators: vec![bubblegum_proxy::Creator { 
            address: deployment.key(), 
            verified: true, 
            share: 100
        }],
    };

    {
        let mut deployment_as_signer = deployment.to_account_info();
        deployment_as_signer.is_signer = true;
        bubblegum_proxy::cpi::mint_v1(
            CpiContext::new_with_signer(ctx.accounts.bubblegum_program.to_account_info(), 
            mint_compressed_accounts, &[tree_delegate_seeds, deployment_seeds])
                .with_remaining_accounts(vec![deployment_as_signer]), 
            metadata_args)?;

    }


    let redeemable = &mut ctx.accounts.redeem;
    redeemable.deployment = deployment.key();
    redeemable.asset = asset_id;


    Ok(())
}




