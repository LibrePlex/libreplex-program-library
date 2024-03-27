use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token::AssociatedToken, token::TokenAccount};

use crate::{events, Liquidity, DEPLOYMENT_TYPE_NFT};
use libreplex_fair_launch::{program::LibreplexFairLaunch, MintInput};

#[derive(Accounts)]
pub struct MintCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked by has one
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut, has_one = deployment, has_one = treasury)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment_non_fungible_token_account: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: UncheckedAccount<'info>,

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

    #[account(init_if_needed, 
        payer = payer,
        associated_token::authority = liquidity,
         associated_token::mint = fungible_mint)]
    pub liquidity_fungible_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,

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

pub fn mint_handler<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;

    let liquidity = &mut ctx.accounts.liquidity;
    
    if liquidity.deployment_type != DEPLOYMENT_TYPE_NFT {
        panic!("Wrong deployment type. Expected type=0 (NFT), received {}", liquidity.deployment_type)
    }

    liquidity.total_mints += 1;

    if liquidity.cosigner_program_id.ne(&system_program::ID) {
        panic!("No co signer")
    }

    let seeds = &[
        b"liquidity",
        liquidity.seed.as_ref(),
        &[liquidity.bump],
    ];

    let mut refund_due_to_payer = 0;

    if liquidity.lookup_table_address == system_program::ID {
        panic!("Lookup table not initialised");
    }

    let should_double_mint = if let Some(required_double_mints) = liquidity.required_double_mints {
        liquidity.total_mints < required_double_mints as u64
    } else {
        liquidity.total_mints % liquidity.lp_ratio  as u64 == 0
    };

    if should_double_mint {
        let balance_before = AsRef::<AccountInfo>::as_ref(liquidity.as_ref()).lamports();

        let remaining_accounts_mint_pooled = ctx.remaining_accounts[
            std::cmp::min(4, ctx.remaining_accounts.len())..std::cmp::min(8, ctx.remaining_accounts.len())].to_vec();



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
        ).with_remaining_accounts(remaining_accounts_mint_pooled), MintInput {
            multiplier_denominator: 1,
            multiplier_numerator: 1,
        })?;

        let balance_after = AsRef::<AccountInfo>::as_ref(liquidity.as_ref()).lamports();

        refund_due_to_payer = balance_after.saturating_sub(balance_before);

    
        libreplex_fair_launch::cpi::swap_to_fungible22(
            CpiContext::new_with_signer(
                ctx.accounts.fair_launch.to_account_info(),
                libreplex_fair_launch::cpi::accounts::SwapToFungible2022Ctx {
                    non_fungible_source_account_owner: liquidity.to_account_info(),
                    fungible_target_token_account_owner: liquidity.to_account_info(),
                    deployment: ctx.accounts.deployment.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    signer: liquidity.to_account_info(),
                    fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
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
            ).with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        )?;
    }

    let remaining_accounts_mint 
        = ctx.remaining_accounts[0..std::cmp::min(4, ctx.remaining_accounts.len())].to_vec();


    let balance_before = AsRef::<AccountInfo>::as_ref(liquidity.as_ref()).lamports();
    libreplex_fair_launch::cpi::mint_token22(CpiContext::new_with_signer(
        fair_launch.to_account_info(),
        libreplex_fair_launch::cpi::accounts::MintToken2022Ctx {
            deployment: ctx.accounts.deployment.to_account_info(),
            deployment_config: ctx.accounts.deployment_config.to_account_info(),
            creator_fee_treasury: ctx.accounts.creator_fee_treasury.to_account_info(),
            hashlist: ctx.accounts.hashlist.to_account_info(),
            hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            signer: liquidity.to_account_info(),
            fungible_mint: ctx.accounts.fungible_mint.to_account_info(),
            minter: ctx.accounts.receiver.to_account_info(),
            non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
            non_fungible_token_account: ctx.accounts.non_fungible_token_account.to_account_info(),
            token_program: ctx.accounts.token_program_22.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[seeds],
    ).with_remaining_accounts(remaining_accounts_mint), MintInput {
        multiplier_denominator: 1,
        multiplier_numerator: 1,
    })?;
    let balance_after = AsRef::<AccountInfo>::as_ref(liquidity.as_ref()).lamports();

    let mint_funds_received = balance_after.saturating_sub(balance_before);


    if refund_due_to_payer > 0 {
        liquidity.sub_lamports(refund_due_to_payer)?;
        ctx.accounts.payer.add_lamports(refund_due_to_payer)?;
        // msg!("Refunding {}", refund_due_to_payer);
    };

    if mint_funds_received > 0 {
        let fee_to_creator =  mint_funds_received.checked_mul(liquidity.creator_basis_points).unwrap().checked_div(10000).unwrap();

        liquidity.sub_lamports(fee_to_creator)?;
        ctx.accounts.treasury.add_lamports(fee_to_creator)?;
    }

    emit!(events::Mint{ liquidity: liquidity.key(), total_mints: liquidity.total_mints });
   

    Ok(())
}
