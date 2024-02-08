use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token, token, token_2022};
use libreplex_fair_launch::deploy_hybrid::sysvar_instructions_program;
use libreplex_fair_launch::Deployment;
use solana_address_lookup_table_program::instruction::{create_lookup_table, extend_lookup_table};
use solana_program::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use libreplex_shared::wrapped_sol;

use crate::Liquidity;

#[derive(Accounts)]
#[instruction()]
pub struct CreateLookupTableForLiquidityCtx<'info> {

    /// CHECK: Can be anyone
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Can be anyone
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, 
        // only allowed when the lookup table has not yet been created
        constraint = liquidity.lookup_table_address.eq(&system_program::ID))]
    pub liquidity: Box<Account<'info, Liquidity>>,

    #[account(constraint = deployment.key() == liquidity.deployment
    )]
    pub deployment: Box<Account<'info, Deployment>>,

    /// CHECK: Created in logic. Type not checked at call time
    #[account(mut)]
    pub lookup_table: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_lookup_table_for_liquidity(ctx: Context<CreateLookupTableForLiquidityCtx>, recent_slot: u64) -> Result<()> {
    // owned by payer - doesn't matter because we're freezing it afterwards
    let payer = &ctx.accounts.payer;
    let liquidity = &mut ctx.accounts.liquidity;
    let lookup_table = &ctx.accounts.lookup_table;
    let lookup_table_ix = create_lookup_table(liquidity.key(), payer.key(), recent_slot);
    let system_program = &ctx.accounts.system_program;
    let deployment = &ctx.accounts.deployment;
    if lookup_table_ix.1.key() != ctx.accounts.lookup_table.key() {
        panic!("Bad lookup table address");
    }

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
   
    let extend_lookup_table = extend_lookup_table(
        lookup_table_ix.1,
        payer.key(),
        Some(payer.key()),
        vec![
           associated_token::ID,
           system_program::ID,
           wrapped_sol::ID,
           crate::ID,
           token::ID,
           token_2022::ID,
           deployment.fungible_mint,
           deployment.key(),
           legacy_metadata,
           hashlist,
           deployment_config,
           libreplex_fair_launch::ID,
           sysvar_instructions_program::ID, 
           libreplex_inscriptions::ID
        ],
    );
    invoke_signed(&extend_lookup_table, &account_infos_extend, &[seeds])?;
    
    // update this - mint is not possible because lookup table is defined
    liquidity.lookup_table_address = lookup_table_ix.1;
    Ok(())
}
