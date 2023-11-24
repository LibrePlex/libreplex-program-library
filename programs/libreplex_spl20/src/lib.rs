use anchor_lang::prelude::*;

use state::*;
use errors::*;

declare_id!("insFmVukT9LYVygNbdpSjbxPy4FtQ6WgcuChnxDLbAm");

pub mod state;
pub mod errors;

#[program]
pub mod libreplex_spl20 {
    use super::*;

    pub fn register_token(ctx: Context<RegisterTokenCtx>, new_deployment: RegisterTokenInput) -> Result<()> {
        let deployment = &mut ctx.accounts.new_deployment_account;

        if new_deployment.ticker.len() > TICKER_LIMIT {
            return Err(Spl20Error::TickerToLong.into());
        }

        if new_deployment.root_type.len() > ROOT_TYPE_LIMIT {
            return Err(Spl20Error::RootTypeToLong.into())
        }
        
        deployment.creator = new_deployment.creator;
        deployment.limit = new_deployment.limit;
        deployment.max = new_deployment.max;
        deployment.collection = new_deployment.collection;
        deployment.ticker = new_deployment.ticker;
        deployment.root = new_deployment.root;
        deployment.root_type = new_deployment.root_type;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(new_deployment: RegisterTokenInput)]
pub struct RegisterTokenCtx<'info>  {
    #[account(init, payer = payer, space = 8 + TokenDeployment::INIT_SPACE + EXCESS, 
        seeds = ["spl20".as_ref(), new_deployment.ticker.as_ref()], bump)]
    pub new_deployment_account: Account<'info, TokenDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RegisterTokenInput {
    pub creator: Pubkey,
    pub limit: u64,
    pub max: u64,
    pub collection: Pubkey,

    pub ticker: String,
    
    pub root: Pubkey,

    pub root_type: String,
}