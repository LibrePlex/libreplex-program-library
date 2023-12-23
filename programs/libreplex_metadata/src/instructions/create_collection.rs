use crate::state::Collection;
use crate::{PermissionEvent, PermissionEventType, COLLECTION, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH};
use anchor_lang::prelude::Error as AnchorError;
use anchor_lang::prelude::*;
use std::result::Result;

use crate::errors::ErrorCode;

#[event]
pub struct CollectionEventCreate {
    pub authority: Pubkey,
    pub name: String,
    pub id: Pubkey,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateCollectionInput {
    pub name: String,
    pub symbol: String,
    pub description: String
}

impl CreateCollectionInput {
    pub fn get_size(&self) -> usize {
        4 + self.name.len() +
        4 + self.symbol.len() +
        4 + self.description.len()
    }
}

#[derive(Accounts)]
#[instruction(collection_input: CreateCollectionInput)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, seeds = [COLLECTION.as_ref(), seed.key().as_ref()],
        bump, payer = authority, space = Collection::BASE_SIZE + collection_input.get_size())]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateCollection>,
    collection_input: CreateCollectionInput,
) -> anchor_lang::Result<()> {
    let group = &mut ctx.accounts.collection;
    let authority = &mut ctx.accounts.authority;
    group.update_authority = authority.key();
    group.seed = ctx.accounts.seed.key();
    group.item_count = 0;
    group.update_authority = authority.key();

    update_collection_from_input(collection_input, group)?;

    emit!(CollectionEventCreate {
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key(),
    });

    msg!(
        "Collection data created with authority pubkey {}",
        ctx.accounts.authority.key()
    );

    emit!(PermissionEvent {
        reference: ctx.accounts.collection.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });

    Ok(())
}

pub fn update_collection_from_input(
    input: CreateCollectionInput,
    collection: &mut Box<Account<Collection>>,
) -> Result<(), AnchorError> {
    let CreateCollectionInput {
        name,
        symbol,
        description,
    } = input;
    let name_length = name.len();
    let symbol_length = symbol.len();
    if name_length > MAX_NAME_LENGTH || symbol_length > MAX_SYMBOL_LENGTH {
        return Err(error!(ErrorCode::InvalidStringInput));
    }

    collection.name = name.clone();
    collection.symbol = symbol;
    
    collection.description = description;

    Ok(())
}
