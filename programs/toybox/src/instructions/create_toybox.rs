use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_2022::solana_zk_token_sdk::zk_token_proof_instruction::PubkeyValidityData;
use libreplex::Permissions;

use crate::{AccountEvent, Toybox, AccountEventType, Phase};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateToyboxInput {
    max_mints: u64,
    seed: Pubkey,
    phases: Vec<Phase>,
}

impl CreateToyboxInput {
    pub fn get_size (&self) -> usize {
        return 8 + 8 + 32 + 4 + &self.phases.len()
    }
}

#[derive(Accounts)]
#[instruction(toybox_input: CreateToyboxInput)]
pub struct CreateToybox<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"toybox", toybox_input.seed.key().as_ref()],
              bump, payer = signer, space = Toybox::BASE_SIZE + toybox_input.get_size())]
    pub toybox: Box<Account<'info, Toybox>>,

    #[account(init, seeds = [b"permissions", toybox.key().as_ref(), signer.key().as_ref()],
            // all permissions start out with one permission, hence the +1
              bump, payer = signer, space = Permissions::BASE_SIZE + 1)]
    pub permissions: Box<Account<'info, Permissions>>,

    /*
        Signer constraint to be relaxed later
        to allow for migration signatures etc.

        Currently this signer does not need to be a mint,
        but you can tag metadata onto anything.
    */
    pub collection: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateToybox>, input: CreateToyboxInput) -> Result<()> {
    let toybox = &mut ctx.accounts.toybox;

    toybox.attributes = vec![];
    toybox.max_mints = input.max_mints;
    toybox.minted = 0;
    toybox.phases = input.phases;

    /* OUT FOR ERRANDS - BACK SOON. All code is in  */

    emit!(AccountEvent {
        reference: toybox.key(),
        event_type: AccountEventType::Create,
    });

    Ok(())
}
