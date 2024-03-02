use anchor_lang::{prelude::*, system_program};


use crate::{events::LiquidityCreate, Liquidity};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub seed: Pubkey,

    pub deployment: Pubkey,
    pub bootstrap_start_time: Option<i64>,
    pub bootstrap_requires_sold_out: bool,
    pub creator_basis_points: u64,

    pub lp_ratio: u16,

    pub pool_fee_basis_points: u64,
    pub cosigner_program_id: Option<Pubkey>,
    pub deployment_type: u8
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct Initialise<'info> {

    /// CHECK: Can be anyone
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Can be anyone
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + Liquidity::INIT_SPACE,
         seeds = [b"liquidity", input.seed.as_ref()], bump)]
    pub liquidity: Box<Account<'info, Liquidity>>,

    pub system_program: Program<'info, System>,
}

pub fn init_handler(ctx: Context<Initialise>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        creator_basis_points,
        lp_ratio,
        pool_fee_basis_points,
        cosigner_program_id,
        deployment_type
    } = input;

    ctx.accounts.liquidity.set_inner(Liquidity {
        pool_bootstrapped: false,
        lp_ratio,
        treasury: ctx.accounts.treasury.key(),
        total_mints: 0,
        pool_fee_basis_points,
        seed,
        bump: ctx.bumps.liquidity,
        bootstrap_start_time,
        bootstrap_requires_sold_out,
        deployment,
        creator_basis_points,
        authority: ctx.accounts.authority.key(),
        lookup_table_address: system_program::ID,
        cosigner_program_id: match cosigner_program_id {
            Some(x)=>x,
            _=>system_program::ID
        },
        deployment_type,
        required_double_mints: None,
        padding: [0; 62],
    });

    emit_create(&ctx.accounts.liquidity);

    Ok(())
}

// Avoid blowing up the stack.
fn emit_create(liquidity: &Account<Liquidity>) {
    let liquidity_ref: &Liquidity = liquidity.as_ref();
    emit!(LiquidityCreate { liquidity: liquidity_ref.clone(), id: liquidity.key()});
}
