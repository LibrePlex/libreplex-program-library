pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022};

use crate::{accounts, HybridRedeemErrors, HybridRedeemer};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct HybridRedeemInput {
    pub allocation_index: u32,
}

#[derive(Accounts)]
#[instruction(input: HybridRedeemInput)]
pub struct HybridRedeem<'info> {
    #[account(mut, has_one = deployment, has_one = allocation_account)]
    pub redeemer: Box<Account<'info, HybridRedeemer>>,

    /// CHECK: Has no data, validated by address
    #[account(init, payer = payer, space = 0, 
        seeds = [redeemer.key().as_ref(), 
            input.allocation_index.to_le_bytes().as_ref()], bump)]
    pub allocation_marker: UncheckedAccount<'info>,

    /// CHECK: Checked by address
    pub allocation_account: UncheckedAccount<'info>,

    /// CHECK: Checked by address
    #[account(mut)]
    pub deployment: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub deployment_config: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)] 
    pub creator_fee_treasury: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub fungible_mint: UncheckedAccount<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub minter: UncheckedAccount<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,
    

     /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: sent via CPI to libreplex_inscriptions_program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,



    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Checked in cpi
    pub inscriptions_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub fair_launch_program: Program<'info, libreplex_fair_launch::program::LibreplexFairLaunch>,
}

pub fn hybrid_redeemer_handler(ctx: Context<HybridRedeem>, input: HybridRedeemInput) -> Result<()> {
    let allocation_data = ctx.accounts.allocation_account.try_borrow_data()?;
    let minter = &ctx.accounts.minter;
    let redeemer = &mut ctx.accounts.redeemer;

    let index = input.allocation_index as usize;
    let expected_winner = Pubkey::try_from(&allocation_data[index..index + 32]).unwrap();

    if &expected_winner != minter.key {
        panic!("Invalid winner");
    }


    let clock = Clock::get()?;

    if clock.unix_timestamp < redeemer.redeem_start {
        return Err(HybridRedeemErrors::RedeemsNotStarted.into())
    }


    let payer = ctx.accounts.payer.to_account_info(); 
    let minter= ctx.accounts.minter.to_account_info();
    let non_fungible_mint = ctx.accounts.non_fungible_mint.to_account_info();
    let non_fungible_token_account = ctx.accounts.non_fungible_token_account.to_account_info();
    let inscription_summary = ctx.accounts.inscription_summary.to_account_info(); 
    let inscription_v3= ctx.accounts.inscription_v3.to_account_info();
    let inscription_data = ctx.accounts.inscription_data.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let associated_token_program = ctx.accounts.associated_token_program.to_account_info();
    let inscriptions_program = ctx.accounts.inscriptions_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();
    let fungible_mint = ctx.accounts.fungible_mint.to_account_info();

    // mutable borrows
    let deployment = ctx.accounts.deployment.to_account_info();
    let deployment_config = ctx.accounts.deployment_config.to_account_info();
    let creator_fee_treasury = ctx.accounts.creator_fee_treasury.to_account_info();
    let hashlist = ctx.accounts.hashlist.to_account_info();
    let fair_launch_program = ctx.accounts.fair_launch_program.to_account_info();
    let hashlist_marker = ctx.accounts.hashlist_marker.to_account_info();


    let redeemer_bump: &[u8] = &[redeemer.bump];
    let redeemer_seeds: &[&[u8]] = &[redeemer.seed.as_ref(), redeemer_bump];

    let accs = libreplex_fair_launch::cpi::accounts::MintToken2022Ctx {
        signer: redeemer.to_account_info(),
        deployment,
        deployment_config,
        creator_fee_treasury,
        hashlist,
        hashlist_marker,
        payer,
        fungible_mint,
        minter,
        non_fungible_mint,
        non_fungible_token_account,
        inscription_summary,
        inscription_v3,
        inscription_data,
        token_program,
        associated_token_program,
        inscriptions_program,
        system_program,
    };

    libreplex_fair_launch::cpi::mint_token22(CpiContext::new_with_signer(fair_launch_program.to_account_info(), 
    accs,   
        &[redeemer_seeds]
    ))?;

    Ok(())
}