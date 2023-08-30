use crate::instructions::update_collection_from_input;
use crate::state::{Group, GroupInput};
use crate::{DelegatePermissions, PermissionType, GroupEvent, GroupEventType};
use crate::errors::ErrorCode;

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

    #[account(seeds = ["permissions".as_ref(), authority.key().as_ref(), group.key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(mut, 
        realloc =  Group::BASE_SIZE + collection_input.get_size(),
        realloc::payer = authority,
        realloc::zero = false)]
    pub group: Box<Account<'info, Group>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateGroup>, collection_input: GroupInput) -> Result<()> {
    let group =&mut ctx.accounts.group;
    let authority = &mut ctx.accounts.authority;

    let user_permissions = &ctx.accounts.delegated_group_wide_permissions;

    let mut can_edit = &group.update_authority == authority.key;

    if let Some(delegated_group_wide_permissions_account) = user_permissions {
        let permissions = &delegated_group_wide_permissions_account.permissions;

        can_edit = can_edit || permissions.contains(&PermissionType::Update);
    }

    if !can_edit {
        return Err(ErrorCode::InvalidPermissions.into())
    }

    update_collection_from_input(collection_input, group)?;

    emit!(GroupEvent{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key(),
        event_type: GroupEventType::Update
    });


    Ok(())
}
