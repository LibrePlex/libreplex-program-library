use crate::instructions::update_collection_from_input;
use crate::state::Collection;

use anchor_lang::prelude::*;

use super::CreateCollectionInput;

#[event]
struct EditCollectionEvent {
    id: Pubkey,
    creator: Pubkey,
    name: String,
}

#[event]
pub struct CollectionEventUpdate {
    pub authority: Pubkey,
    pub name: String,
    pub id: Pubkey,    
}



#[derive(Accounts)]
#[instruction(collection_input: CreateCollectionInput)]
pub struct UpdateCollectionCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        constraint = collection.update_authority == authority.key())]
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateCollectionCtx>, collection_input: CreateCollectionInput) -> Result<()> {
    let group =&mut ctx.accounts.collection;
    
    update_collection_from_input(collection_input, group)?;

    emit!(CollectionEventUpdate{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key()
    });


    Ok(())
}
