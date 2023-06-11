use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_2022::solana_zk_token_sdk::zk_token_proof_instruction::PubkeyValidityData;
use libreplex::Permissions;

use crate::{AccountEvent, Creator, AccountEventType, Phase};

use super::create_creator::CreateCreator;


#[derive(Accounts)]
#[instruction(input: Vec<u8>)]
pub struct AddAttributeMapping<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, 
        constraint = signer.key() == toybox.owner.key()
              )]
    pub toybox: Box<Account<'info, Creator>>,

    #[account(init, seeds = [b"permissions", toybox.key().as_ref(), signer.key().as_ref()],
            // all permissions start out with one permission, hence the +1
              bump, payer = signer, space = Permissions::BASE_SIZE + 1)]
    pub permissions: Box<Account<'info, Permissions>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddAttributeMapping>, input: Vec<u8>) -> Result<()> {
    let toybox = &mut ctx.accounts.toybox;

    /* OUT FOR ERRANDS - BACK SOON. All code is in  */

    emit!(AccountEvent {
        reference: toybox.key(),
        event_type: AccountEventType::Update,
    });

    Ok(())
}
