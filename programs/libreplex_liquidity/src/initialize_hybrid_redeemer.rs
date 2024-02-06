use anchor_lang::prelude::*;

use crate::HybridRedeemer;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitHybridReedemerInput {
    allocation_account: Pubkey,
    redeem_start: i64,
    seed: Pubkey,
    deployment: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    input: InitHybridReedemerInput
)]
pub struct InitialiseHybridRedeemer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: IT DONT MATTER WHO IT IS.
    pub authority: UncheckedAccount<'info>,

    #[account(init, payer = payer, space = 8 + HybridRedeemer::INIT_SPACE, seeds = [input.seed.as_ref()], bump)]
    pub redeemer: Box<Account<'info, HybridRedeemer>>,

    pub system_program: Program<'info, System>,
}


pub fn init_hybrid_redeemer_handler(ctx: Context<InitialiseHybridRedeemer>, input: InitHybridReedemerInput) -> Result<()> {
    let redeemer = &mut ctx.accounts.redeemer;

    let InitHybridReedemerInput { allocation_account, deployment, redeem_start, seed } = input;

    let bump = ctx.bumps.redeemer;

    redeemer.set_inner(HybridRedeemer {
        redeem_start,
        total_redeemed: 0,
        allocation_account,
        deployment,
        seed,
        bump,
        padding: [0; 100]
    });



    Ok(())
}