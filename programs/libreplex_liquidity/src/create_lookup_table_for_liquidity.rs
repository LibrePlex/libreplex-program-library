use std::str::FromStr;

use anchor_lang::{prelude::*, system_program};
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::{associated_token, token, token_2022};
use libreplex_fair_launch::deploy_hybrid::sysvar_instructions_program;
use libreplex_fair_launch::Deployment;
use solana_address_lookup_table_program::instruction::{create_lookup_table, extend_lookup_table};
use solana_program::{address_lookup_table, msg};
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use libreplex_shared::wrapped_sol;
use vault_proxy::VAULT_BASE;

use crate::Liquidity;

#[derive(Accounts)]
#[instruction()]
pub struct CreateLookupTableForLiquidityCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, 
        // only allowed when the lookup table has not yet been created
        has_one = deployment,
        constraint = liquidity.lookup_table_address.eq(&system_program::ID))]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account()]
    pub deployment: Box<Account<'info, Deployment>>,

    /// CHECK: Created in logic. Type not checked at call time
    #[account(mut)]
    pub lookup_table: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked by addy
    #[account(seeds = [b"vault", wrapped_sol::ID.as_ref(), Pubkey::from_str(VAULT_BASE).unwrap().as_ref()], bump, seeds::program = vault_proxy::ID)]
    pub wrapped_sol_vault: UncheckedAccount<'info>,
}

pub fn create_lookup_table_for_liquidity(ctx: Context<CreateLookupTableForLiquidityCtx>, recent_slot: u64) -> Result<()> {
    // owned by payer - doesn't matter because we're freezing it afterwards
    let payer = &ctx.accounts.payer;
    let liquidity = &mut ctx.accounts.liquidity;
    let lookup_table = &ctx.accounts.lookup_table;
    let lookup_table_ix = create_lookup_table(liquidity.key(), payer.key(), recent_slot);
    let system_program = &ctx.accounts.system_program;
    let deployment = &ctx.accounts.deployment;
    let wrapped_sol_vault = &ctx.accounts.wrapped_sol_vault;

    msg!(
        "{} <=> {}",
        lookup_table_ix.1.key(),
        &ctx.accounts.lookup_table.key()
    );
    let account_infos = vec![
        lookup_table.to_account_info(),
        liquidity.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    ];
   
    
    invoke(&lookup_table_ix.0, account_infos.as_slice())?;

    msg!("Invoking extend");

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];
    let account_infos_extend = vec![
        lookup_table.to_account_info(),
        liquidity.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    ];

    let legacy_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            deployment.fungible_mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;
   
    let fungible_escrow = anchor_spl::associated_token::get_associated_token_address(&deployment.key(), &deployment.fungible_mint);
    let liquidity_escrow = anchor_spl::associated_token::get_associated_token_address(&liquidity.key(), &deployment.fungible_mint);
    let liquidity_wrapped_sol_escrow = anchor_spl::associated_token::get_associated_token_address(&liquidity.key(), &wrapped_sol::ID);

    let pool = amm_proxy::derive_permissionless_pool_with_fee_tier(amm_proxy::CurveType::ConstantProduct, wrapped_sol::ID, 
        deployment.fungible_mint, liquidity.pool_fee_basis_points).0;

    let lp_mint  = Pubkey::find_program_address(&[b"lp_mint", pool.as_ref()], &amm_proxy::ID).0;

    let lp_mint_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            lp_mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let vault_base = Pubkey::from_str(VAULT_BASE).unwrap();

  
    let fungible_vault
         = Pubkey::find_program_address(
            &[b"vault", deployment.fungible_mint.key().as_ref(), vault_base.as_ref()], 
            &vault_proxy::ID).0;
    let fungible_vault_token_account = Pubkey::find_program_address(&[b"token_vault", fungible_vault.key().as_ref()], &vault_proxy::ID).0;
    let fungible_vault_lp_mint = Pubkey::find_program_address(&[b"lp_mint", fungible_vault.as_ref()], &vault_proxy::ID).0;
    let fungible_vault_lp_token_account = Pubkey::find_program_address(&[fungible_vault.as_ref(), pool.as_ref()], &amm_proxy::ID).0;

   let wrapped_sol_vault_token_account = Pubkey::find_program_address(&[b"token_vault", wrapped_sol_vault.key().as_ref()], &vault_proxy::ID).0;
   //let wrapped_sol_lp_mint = wrapped_sol_vault.lp_mint;
   let wrapped_sol_vault_lp_token_account = Pubkey::find_program_address(&[wrapped_sol_vault.key().as_ref(), pool.as_ref()], &amm_proxy::ID).0;

    let system_program_fungible_account = get_associated_token_address(system_program.key, &deployment.fungible_mint);

    let wrapped_sol_fee = Pubkey::find_program_address(&[b"fee", wrapped_sol::ID.as_ref(), pool.as_ref()], &amm_proxy::ID).0;
    let fungible_mint_fee = Pubkey::find_program_address(&[b"fee", deployment.fungible_mint.as_ref(), pool.as_ref()], &amm_proxy::ID).0;

    {
        let extend_lookup_table = extend_lookup_table(
            lookup_table_ix.1,
            liquidity.key(),
            Some(payer.key()),
            vec![
               associated_token::ID,
               system_program::ID,
               wrapped_sol::ID,
               address_lookup_table::program::ID,
               crate::ID,
               token::ID,
               deployment.fungible_mint,
               liquidity.treasury,
               token_2022::ID,
               deployment.fungible_mint,
               fungible_escrow,
               liquidity.key(),
               liquidity_escrow,
               deployment.key(),
               legacy_metadata,
               hashlist,
               deployment_config,
               solana_program::sysvar::rent::ID,
               libreplex_fair_launch::ID,
               sysvar_instructions_program::ID, 
               libreplex_inscriptions::ID,
               mpl_token_metadata::ID,
               pool,
               lp_mint,
               lp_mint_metadata,
               liquidity_wrapped_sol_escrow,
               amm_proxy::ID,
               vault_proxy::ID,
               // METEORA FEE_OWNER - this is a hard coded account in Meteora
               Pubkey::from_str("6WaLrrRfReGKBYUSkmx2K6AuT21ida4j8at2SUiZdXu8").unwrap(),
               fungible_vault,
               fungible_vault_token_account,
               fungible_vault_lp_mint,
               fungible_vault_lp_token_account,
            ],
        );
        invoke_signed(&extend_lookup_table, &account_infos_extend, &[seeds])?;
    }

    {
        let extend_lookup_table = extend_lookup_table(
            lookup_table_ix.1,
            liquidity.key(),
            Some(payer.key()),
            vec![
               wrapped_sol_vault.key(),
               wrapped_sol_vault_token_account,
    
               //wrapped_sol_lp_mint,
               wrapped_sol_vault_lp_token_account,
    
               system_program_fungible_account,
    
               wrapped_sol_fee,
               fungible_mint_fee,
            ],
        );
        invoke_signed(&extend_lookup_table, &account_infos_extend, &[seeds])?;
    }
    
    // update this - mint is not possible because lookup table is defined
    liquidity.lookup_table_address = lookup_table_ix.1;
    Ok(())
}
