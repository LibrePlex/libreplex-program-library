use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token::AssociatedToken, token::{spl_token, TokenAccount}, token_interface::{spl_token_2022, transfer_checked, TransferChecked}};

use crate::{Liquidity, DEPLOYMENT_TYPE_SPL};
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment};

#[derive(Accounts)]
pub struct MintSplCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        constraint = liquidity.cosigner_program_id.eq(&system_program::ID) || authority.key() == liquidity.authority)]
    pub authority: Signer<'info>,

    /// CHECK: Checked by has one
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut, 
        has_one = deployment, has_one = treasury)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi, hence no checks required
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_config: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub creator_fee_treasury: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_hashlist_market: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub fungible_mint: UncheckedAccount<'info>,

    #[account(mut, 
        associated_token::authority = liquidity,
         associated_token::mint = fungible_mint)]
    pub liquidity_fungible_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub fungible_token_account_minter: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub pooled_non_fungible_token_account: UncheckedAccount<'info>,


    /// CHECK: Checked in cpi.
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    pub token_program_22: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,

    /// CHECK: Checked in cpi.
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn mint_spl_handler<'info>(ctx: Context<'_, '_, '_, 'info, MintSplCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;

    let liquidity = &mut ctx.accounts.liquidity;
    let deployment = &ctx.accounts.deployment;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let token_program = &ctx.accounts.token_program;
    let token_program_22 = &ctx.accounts.token_program_22;
    
    liquidity.total_mints += 1;

    if liquidity.deployment_type != DEPLOYMENT_TYPE_SPL {
        panic!("Wrong deployment type. Expected type=1 (SPL), received {}", liquidity.deployment_type)
    }

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];


    if liquidity.lookup_table_address == system_program::ID {
        panic!("Lookup table not initialised");
    }

    let remaining_accounts_mint_pooled = ctx.remaining_accounts[
        std::cmp::min(0, ctx.remaining_accounts.len())..std::cmp::min(4, ctx.remaining_accounts.len())].to_vec();

    libreplex_fair_launch::cpi::mint_token22(CpiContext::new_with_signer(
        fair_launch.to_account_info(),
        libreplex_fair_launch::cpi::accounts::MintToken2022Ctx {
            deployment: ctx.accounts.deployment.to_account_info(),
            deployment_config: ctx.accounts.deployment_config.to_account_info(),
            creator_fee_treasury: ctx.accounts.creator_fee_treasury.to_account_info(),
            hashlist: ctx.accounts.hashlist.to_account_info(),
            hashlist_marker: ctx.accounts.pooled_hashlist_market.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            signer: liquidity.to_account_info(),
            fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
            minter: liquidity.to_account_info(),
            non_fungible_mint: ctx.accounts.pooled_non_fungible_mint.to_account_info(),
            non_fungible_token_account: ctx
                .accounts
                .pooled_non_fungible_token_account
                .to_account_info(),
            token_program: ctx.accounts.token_program_22.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[seeds],
    ).with_remaining_accounts(remaining_accounts_mint_pooled))?;


    libreplex_fair_launch::cpi::swap_to_fungible22(
        CpiContext::new_with_signer(
            ctx.accounts.fair_launch.to_account_info(),
            libreplex_fair_launch::cpi::accounts::SwapToFungible2022Ctx {
                non_fungible_source_account_owner: liquidity.to_account_info(),
                fungible_target_token_account_owner: liquidity.to_account_info(),
                deployment: ctx.accounts.deployment.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: liquidity.to_account_info(),
                fungible_mint: fungible_mint.to_account_info(),
                hashlist_marker: ctx.accounts.pooled_hashlist_market.to_account_info(),
                fungible_source_token_account: ctx.accounts.deployment_fungible_token_account.to_account_info(),
                fungible_target_token_account: ctx.accounts.liquidity_fungible_token_account.to_account_info(),
                non_fungible_mint: ctx.accounts.pooled_non_fungible_mint.to_account_info(),
                non_fungible_source_token_account: ctx.accounts.pooled_non_fungible_token_account.to_account_info(),
                non_fungible_target_token_account: ctx.accounts.deployment_non_fungible_token_account.to_account_info(),
                token_program_22: ctx.accounts.token_program_22.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                sysvar_instructions: ctx.accounts.sysvar_instructions.to_account_info(),
            }, 
            &[seeds]
        )
    )?;


    // the amount that goes to the minter is r/(r+1) where r = lp_ratio
    // that means 1/r is left in the LP reserve

    let amount_to_transfer_to_minter = deployment.get_fungible_mint_amount().checked_mul(
        liquidity.lp_ratio as u64
    ).unwrap().checked_div(liquidity.lp_ratio as u64+1).unwrap();


    let token_program_for_fungible = match *fungible_mint.owner {
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("Fungible mint is not owned by tokenkeg or token-2022");
        }
    };

    transfer_checked(
        CpiContext::new_with_signer(
            token_program_for_fungible.clone(),
            TransferChecked {
                to: ctx.accounts.fungible_token_account_minter.to_account_info(),
                from: ctx.accounts.liquidity_fungible_token_account.to_account_info(),
                authority: liquidity.to_account_info(),
                mint: fungible_mint.to_account_info(),
            },
            &[seeds],
        ),
        amount_to_transfer_to_minter,
        deployment.decimals
    )?;
   
    Ok(())
}
