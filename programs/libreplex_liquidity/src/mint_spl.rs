use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token_interface::{TokenAccount, Mint},
    associated_token::AssociatedToken, token::spl_token, token_interface::{spl_token_2022}};
use libreplex_shared::operations::transfer_generic_spl;

use crate::{events, Liquidity, DEPLOYMENT_TYPE_SPL};
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment, HashlistMarker, MintInput};

#[derive(Accounts)]
pub struct MintSplCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,


    // this prevents direct mints via liquidity program, for ex with
    // pipelines
    #[account(
        constraint = liquidity.cosigner_program_id.eq(&system_program::ID) || authority.key() == liquidity.authority)]
    pub authority: Signer<'info>,

    #[account(mut, 
        has_one = deployment)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut,
        token::authority = deployment,
        token::mint = fungible_mint)]
    pub deployment_fungible_token_account: InterfaceAccount<'info, TokenAccount>,

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
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    #[account(mut, 
        associated_token::authority = liquidity,
         associated_token::mint = fungible_mint)]
    pub liquidity_fungible_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Checked in cpi.
    #[account(mut,
        constraint = fungible_token_account_receiver.key().eq(
            &anchor_spl::associated_token::get_associated_token_address_with_program_id(
                &receiver.key(),
                &fungible_mint.key(),
                fungible_mint.to_account_info().owner,
            )
        ))]
    pub fungible_token_account_receiver: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub liquidity_non_fungible_token_account: UncheckedAccount<'info>,


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
            minter: liquidity.to_account_info(),
            non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
            non_fungible_token_account: ctx
                .accounts
                .liquidity_non_fungible_token_account
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

    

    libreplex_fair_launch::cpi::swap_to_fungible22(
        CpiContext::new_with_signer(
            ctx.accounts.fair_launch.to_account_info(),
            libreplex_fair_launch::cpi::accounts::SwapToFungible2022Ctx {
                
                // these are the same
                non_fungible_source_token_account: ctx.accounts.liquidity_non_fungible_token_account.to_account_info(),
                non_fungible_source_account_owner: liquidity.to_account_info(),
                non_fungible_target_token_account: ctx.accounts.deployment_non_fungible_token_account.to_account_info(),
                

                fungible_target_token_account_owner: liquidity.to_account_info(),
                
                fungible_source_token_account: ctx.accounts.deployment_fungible_token_account.to_account_info(),
                fungible_target_token_account: ctx.accounts.liquidity_fungible_token_account.to_account_info(),
                
                
                
                deployment: ctx.accounts.deployment.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: liquidity.to_account_info(),
                fungible_mint: fungible_mint.to_account_info(),
                hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
                non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
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

    let mut marker_data: &[u8] = &ctx.accounts.hashlist_marker.try_borrow_data()?;
    let marker = HashlistMarker::try_deserialize(&mut marker_data)?;

    let amount_to_transfer_to_minter = liquidity.amount_to_transfer_to_minter(deployment, &marker);
    
    // deployment.get_fungible_mint_amount(&marker).checked_mul(
    //     liquidity.lp_ratio as u64 - 1
    // ).unwrap().checked_div(liquidity.lp_ratio as u64).unwrap();


    let token_program_for_fungible = match *fungible_mint.to_account_info().owner{
        spl_token::ID => token_program.to_account_info(),
        spl_token_2022::ID => token_program_22.to_account_info(),
        _ => {
            panic!("Fungible mint is not owned by tokenkeg or token-2022");
        }
    };


    
    transfer_generic_spl(
        &token_program_for_fungible,
        &ctx.accounts.liquidity_fungible_token_account.to_account_info(),
        &ctx.accounts.fungible_token_account_receiver.to_account_info(),
        &liquidity.to_account_info(),
        &fungible_mint.to_account_info(),
        &ctx.accounts.receiver.to_account_info(),
        &ctx.accounts.associated_token_program.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        Some(&[seeds]),
        &ctx.accounts.payer.to_account_info(),
        deployment.decimals,
        amount_to_transfer_to_minter,
        ctx.remaining_accounts,
    )?;

    let balance_after = AsRef::<AccountInfo>::as_ref(liquidity.as_ref()).lamports();

    let mint_funds_received = balance_after.saturating_sub(balance_before);
    msg!("mint funds received {}", mint_funds_received);


    if mint_funds_received > 0 && !liquidity.treasury.eq(&system_program::ID) {

        let treasury_account_info = ctx.remaining_accounts.iter().find(|&r| r.key().eq(&liquidity.treasury));

        if let Some(x) = treasury_account_info {
            msg!("creator basis points {}", liquidity.creator_basis_points);
            let fee_to_creator =  mint_funds_received.checked_mul(liquidity.creator_basis_points).unwrap().checked_div(10000).unwrap();

            msg!("Fee to creator {} ({})", fee_to_creator, x.key());
        
            liquidity.sub_lamports(fee_to_creator)?;
            x.add_lamports(fee_to_creator)?;
        } else {
            panic!("Liquidity specifies a treasury but it was not found in remaining accounts");
        }
    }

    // transfer_checked(
    //     CpiContext::new_with_signer(
    //         token_program_for_fungible.clone(),
    //         TransferChecked {
    //             to: ctx.accounts.fungible_token_account_minter.to_account_info(),
    //             from: ctx.accounts.liquidity_fungible_token_account.to_account_info(),
    //             authority: liquidity.to_account_info(),
    //             mint: fungible_mint.to_account_info(),
    //         },
    //         &[seeds],
    //     ),
    //     amount_to_transfer_to_minter,
    //     deployment.decimals
    // )?;
    msg!("Transferred");

    emit!(events::Mint{ liquidity: liquidity.key(), total_mints: liquidity.total_mints });
   
    Ok(())
}
