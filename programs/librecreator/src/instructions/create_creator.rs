use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_2022::solana_zk_token_sdk::zk_token_proof_instruction::PubkeyValidityData;
use libreplex::{Permissions, Group};

use crate::{AccountEvent, Creator, AccountEventType, Phase};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateCreatorInput {
    pub max_mints: u64,
    pub seed: Pubkey,
    pub phases: Vec<Phase>,
}

impl CreateCreatorInput {
    pub fn get_size (&self) -> usize {
        return 8 + 8 + 32 + 4 + &self.phases.len()
    }
}

#[derive(Accounts)]
#[instruction(create_creator_input: CreateCreatorInput)]
pub struct CreateCreator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"creator", create_creator_input.seed.key().as_ref()],
              bump, payer = signer, space = Creator::BASE_SIZE + create_creator_input.get_size())]
    pub creator: Box<Account<'info, Creator>>,

    #[account(init, seeds = [b"permissions", creator.key().as_ref(), signer.key().as_ref()],
            // all permissions start out with one permission, hence the +1
              bump, payer = signer, space = Permissions::BASE_SIZE + 1)]
    pub permissions: Box<Account<'info, Permissions>>,

    /*
        Signer constraint to be relaxed later
        to allow for migration signatures etc.

        Currently this signer does not need to be a mint,
        but you can tag metadata onto anything.
    */
    pub group: Box<Account<'info, Group>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCreator>, input: CreateCreatorInput) -> Result<()> {
    let creator = &mut ctx.accounts.creator;

    creator.max_mints = input.max_mints;
    creator.minted = 0;
    creator.phases = input.phases;
    creator.owner = ctx.accounts.signer.key();
    creator.seed = input.seed.key();

    /* OUT FOR ERRANDS - BACK SOON. All code is in  */

    emit!(AccountEvent {
        reference: creator.key(),
        event_type: AccountEventType::Create,
    });

    Ok(())
}
