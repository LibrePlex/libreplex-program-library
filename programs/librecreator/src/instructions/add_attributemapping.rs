use anchor_lang::prelude::*;
use libreplex::Permissions;

use crate::{AccountEvent, Creator, AccountEventType};



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
