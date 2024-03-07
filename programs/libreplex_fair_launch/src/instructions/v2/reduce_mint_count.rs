use anchor_lang::prelude::*;



use crate::Deployment;



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct ReduceMintCountInput {
    pub max_number_of_tokens: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
}


#[derive(Accounts)]
#[instruction(input: ReduceMintCountInput)]
pub struct ReduceMintCountCtx<'info>  {
    #[account(mut,
        constraint = deployment.creator == creator.key())]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Can be anyone.
    pub creator: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}



pub fn reduce_mint_count(ctx: Context<ReduceMintCountCtx>, input: ReduceMintCountInput) -> Result<()> {
    
    let deployment: &mut Account<'_, Deployment> = &mut ctx.accounts.deployment;
  
    if deployment.max_number_of_tokens < input.max_number_of_tokens {
        panic!("Cannot increase the number of tokens")
    }

    if deployment.number_of_tokens_issued > input.max_number_of_tokens {
        panic!("Max number of tokens must be less than number of tokens issued")
    }

    deployment.max_number_of_tokens = input.max_number_of_tokens;

    Ok(())
}