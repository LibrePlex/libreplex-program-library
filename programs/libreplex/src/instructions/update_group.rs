use crate::instructions::{update_collection_from_input};
use crate::state::{Group, GroupInput};
use crate::{
    GROUP, Permissions, PermissionType, assert_valid_permissions, GroupEvent, GroupEventType
};
use anchor_lang::prelude::*;

#[event]
struct EditCollectionEvent {
    id: Pubkey,
    creator: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(collection_input: GroupInput)]
pub struct UpdateGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = ["permissions".as_ref(), group.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,

    #[account(mut, 
        realloc =  Group::BASE_SIZE + collection_input.get_size(),
        realloc::payer = authority,
        realloc::zero = false,
        seeds = [GROUP.as_ref(), seed.key().as_ref()],
      bump)]
    pub group: Box<Account<'info, Group>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateGroup>, collection_input: GroupInput) -> Result<()> {


    let group =&mut ctx.accounts.group;
    let authority = &mut ctx.accounts.authority;
    let user_permissions = &ctx.accounts.permissions;

    assert_valid_permissions(&user_permissions, group.key(),  authority.key(), &PermissionType::Admin)?;

    
    update_collection_from_input(collection_input, group)?;


    emit!(GroupEvent{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key(),
        event_type: GroupEventType::Update
    });


    Ok(())
}
