use anchor_lang::prelude::*;

use crate::Creator;


#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateInput {
    pub mint_authority: Pubkey,
}


#[derive(Accounts)]
#[instruction(input: UpdateInput)]
pub struct UpdateCreator<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(mut, has_one = update_authority)]
    pub creator: Box<Account<'info, Creator>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateCreator>, input: UpdateInput) -> Result<()> {
    let creator = &mut ctx.accounts.creator;

    creator.mint_authority = input.mint_authority;

    Ok(())
}