use anchor_lang::{prelude::*};

use crate::state::{Phase, CreatorController};

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateInput {
    pub phases: Vec<Phase>,
}

#[derive(Accounts)]
#[instruction(input: UpdateInput)]
pub struct Update<'info> {
    pub update_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, realloc = CreatorController::size_for_input(&input.phases), realloc::zero = false,
                realloc::payer = payer, has_one = update_authority)]
    pub creator_controller: Account<'info, CreatorController>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Update>, input: UpdateInput) -> Result<()> {
    let controller = &mut ctx.accounts.creator_controller;

    controller.phases = input.phases;

    Ok(())
}
