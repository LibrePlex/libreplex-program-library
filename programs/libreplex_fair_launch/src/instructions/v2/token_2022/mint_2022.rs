

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022, token_interface::Mint
};

use crate::{
    Deployment, HashlistMarker, 
    mint_token2022_logic, DeploymentConfig,
};

#[derive(Accounts)]
pub struct MintToken2022Ctx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        seeds = ["deployment_config".as_ref(), deployment.key().as_ref()], bump)]
    pub deployment_config: Account<'info, DeploymentConfig>,

    /// CHECK: checked in constraint
    #[account(mut,
        constraint = deployment_config.creator_fee_treasury == creator_fee_treasury.key())] 
    pub creator_fee_treasury: UncheckedAccount<'info>,

    #[account(mut, 
        
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = 8 + HashlistMarker::INIT_SPACE,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub fungible_mint: InterfaceAccount<'info, Mint>,


    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub minter: UncheckedAccount<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub non_fungible_token_account: UncheckedAccount<'info>,
    
    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account()]
    pub system_program: Program<'info, System>,

}

#[derive(AnchorSerialize)]
pub struct MintInput {
    pub multiplier_numerator: u16,
    pub multiplier_denominator: u16,
}

impl AnchorDeserialize for MintInput {
    fn deserialize_reader<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        let maybe_multiplier_numerator: std::io::Result<u16> = AnchorDeserialize::deserialize_reader(reader);
        let maybe_multiplier_denominator: std::io::Result<u16> = AnchorDeserialize::deserialize_reader(reader);

        if let Ok(a) = maybe_multiplier_numerator {
            if let Ok(b) = maybe_multiplier_denominator {
                return Ok(Self {
                    multiplier_numerator: a,
                    multiplier_denominator: b,
                })
            }
        }
        
       
        Ok(Self {
            multiplier_numerator: 1, 
            multiplier_denominator: 1,
        })
    }
}

pub fn mint_token2022<'info>(
    ctx: Context<'_, '_, '_, 'info, MintToken2022Ctx<'info>>, 
    input: MintInput
) -> Result<()> {
    let payer = &ctx.accounts.payer; 
    let signer = &ctx.accounts.signer;
    let minter= &ctx.accounts.minter;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_token_account = &ctx.accounts.non_fungible_token_account;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let fungible_mint = &ctx.accounts.fungible_mint;

    // mutable borrows
    let deployment = &mut ctx.accounts.deployment;
    let deployment_config = &mut ctx.accounts.deployment_config;
    let creator_fee_treasury = &mut ctx.accounts.creator_fee_treasury;
    let hashlist = &mut ctx.accounts.hashlist;

    mint_token2022_logic(
        deployment, 
        deployment_config,
        creator_fee_treasury,
        &fungible_mint.to_account_info(),
        non_fungible_mint, 
        system_program, 
        payer, 
        associated_token_program, 
        token_program, 
        minter, 
        non_fungible_token_account, 
        hashlist,
        &mut ctx.accounts.hashlist_marker,
        ctx.bumps.deployment,
        ctx.remaining_accounts, signer, true, input)?;

    Ok(())
}
