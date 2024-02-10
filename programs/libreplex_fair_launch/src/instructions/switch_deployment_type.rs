use std::str::FromStr;

use anchor_lang::{prelude::*, system_program};



use crate::{check_deploy_allowed, Deployment, DeploymentConfig, HYBRID_DEPLOYMENT_TYPE};


pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}   

/*

    Initialise sets the main template parameters of the deployment:
    1) ticker
    2) deployment template
    3) mint template
    4) decimals
    5) limit per mint
    6) max number of tokens

    It does not create any inscriptions / mints as these are handled by the deploy endpoints.
    This method is metadata agnostic.

*/

#[derive(Accounts)]
#[instruction(deployment_type: u8)]
pub struct SwitchDeploymentTypeCtx<'info>  {
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        
        seeds=["deployment_config".as_bytes(), deployment.key().as_ref()],
        bump,
        // restriction to be removed when the process is more streamlined. maybe allow
        // a time window for changing the deployment type?
        constraint = deployment_config.cosigner_program_id.eq(&Pubkey::from_str("LiquGRWGrp8JKspo8zDDu6qpRmX1p6U3PX2USqiE1eg").unwrap())
    )]
    pub deployment_config: Account<'info, DeploymentConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}



pub fn switch_deployment_type(ctx: Context<SwitchDeploymentTypeCtx>, deployment_type: u8) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    if deployment_type != HYBRID_DEPLOYMENT_TYPE {
        panic!("Can only switch to hybrid at the moment")
    }
    // same criteria as for allowing deployment
    check_deploy_allowed(deployment);

    deployment.deployment_type = deployment_type;
    deployment.fungible_mint = system_program::ID;
   
    Ok(())
}
