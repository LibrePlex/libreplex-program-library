

use anchor_lang::prelude::*;


use libreplex_fair_launch::DeploymentV2;
use nifty_asset::{extensions::{ExtensionBuilder, MetadataBuilder}, instructions::UpdateCpiBuilder, types::{ExtensionInput, ExtensionType}};




use crate::NiftyHybrid;

#[derive(Accounts)]
pub struct UpdateMetadataCtx<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, 
        constraint = creator.key() == deployment.creator)]
    pub creator: Signer<'info>,

    #[account(mut,
        constraint = nifty_hybrid.deployment == deployment.key())]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,
   

    #[account(mut)]
    pub deployment: Box<Account<'info, DeploymentV2>>,

    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    #[account(mut, 
        constraint = nifty_hybrid.group_mint == group_mint.key())]
    pub group_mint: UncheckedAccount<'info>,

    #[account(
        constraint = nifty_program.key().eq(&nifty_asset::ID)
    )]
    pub nifty_program: UncheckedAccount<'info>,

}

pub fn update_metadata_handler<'info>(ctx: Context<'_, '_, '_, 'info, UpdateMetadataCtx<'info>>) -> Result<()> {
    let nifty_hybrid = &mut ctx.accounts.nifty_hybrid;
    let nifty_program = &ctx.accounts.nifty_program;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let deployment = &ctx.accounts.deployment;
    let payer = &ctx.accounts.payer;
    let seeds = &[
        b"nifty_hybrid",
        nifty_hybrid.seed.as_ref(),
        &[nifty_hybrid.bump],
    ];


    let mut metadata_builder = MetadataBuilder::default();
    metadata_builder.set(
        Some(&deployment.ticker.to_string()), 
        None, 
        Some(&deployment.offchain_url.to_string()));

    let data = metadata_builder.data();


    UpdateCpiBuilder::new(nifty_program)
    .asset(&non_fungible_mint.to_account_info())
    .payer(Some(&payer.to_account_info()))
    .authority(&nifty_hybrid.to_account_info())
    .extension(ExtensionInput {
        extension_type: ExtensionType::Metadata,
        length: data.len() as u32,
        data: Some(data),
    })
    .invoke_signed(&[seeds])?;

    Ok(())
}
