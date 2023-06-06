use crate::instructions::{update_collection_from_input, CollectionEvent, CollectionEventType};
use crate::state::{Collection, CollectionInput};
use crate::{COLLECTION, Permissions, assert_valid_permissions};
use anchor_lang::prelude::*;

use prog_common::errors::ErrorCode;

#[event]
struct EditCollectionEvent {
    id: Pubkey,
    creator: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(collection_input: CollectionInput)]
pub struct EditCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = ["permissions".as_ref(), collection.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub user_permissions: Box<Account<'info, Permissions>>,

    #[account(mut, 
        realloc =  Collection::BASE_SIZE + collection_input.get_size(),
        realloc::payer = authority,
        realloc::zero = false,
        seeds = [COLLECTION.as_ref(), seed.key().as_ref()],
      bump)]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EditCollection>, collection_input: CollectionInput) -> Result<()> {


    let collection = &mut ctx.accounts.collection;
    let authority = &mut ctx.accounts.authority;
    let user_permissions = &ctx.accounts.user_permissions;

    assert_valid_permissions(&user_permissions, collection.key(),  authority.key(), crate::PermissionType::Admin)?;

    
    update_collection_from_input(collection_input, collection)?;


    emit!(CollectionEvent{
        creator: ctx.accounts.authority.key(),
        name: collection.name.clone(),
        id: collection.key(),
        event_type: CollectionEventType::Edit
    });


    Ok(())
}
