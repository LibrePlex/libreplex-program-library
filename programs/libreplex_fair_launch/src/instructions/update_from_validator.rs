use anchor_lang::prelude::*;



use crate::Deployment;


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
#[instruction(validated_token_count: u64, offchain_url: String)]
pub struct UpdateFromValidatorCtx<'info>  {
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut,
        constraint = payer.key().to_string() == *"11111111111111111111111111111111".to_owned())]
    pub payer: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,


}

pub fn update_from_validator(ctx: Context<UpdateFromValidatorCtx>, validated_token_count: u64, offchain_url: String) -> Result<()> {
    
    let deployment = &mut ctx.accounts.deployment;
   
    deployment.max_number_of_tokens = validated_token_count;
    deployment.number_of_tokens_issued = validated_token_count;
    deployment.offchain_url = offchain_url;
    
    Ok(())
}