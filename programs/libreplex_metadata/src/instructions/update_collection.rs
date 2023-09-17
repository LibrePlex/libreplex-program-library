use crate::instructions::update_collection_from_input;
use crate::state::{Collection, CollectionInput};
use crate::{DelegatePermissions, PermissionType};
use crate::errors::ErrorCode;

use anchor_lang::prelude::*;

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
#[instruction(collection_input: CollectionInput)]
pub struct UpdateGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(seeds = ["permissions".as_ref(), authority.key().as_ref(), group.key().as_ref()], bump)]
    pub delegated_group_wide_permissions: Option<Box<Account<'info, DelegatePermissions>>>,

    #[account(mut, 
        realloc =  Collection::BASE_SIZE + collection_input.get_size(),
        realloc::payer = authority,
        realloc::zero = false)]
    pub group: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateGroup>, collection_input: CollectionInput) -> Result<()> {
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

    emit!(CollectionEventUpdate{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key()
    });


    Ok(())
}
