use anchor_lang::prelude::*;
use crate::state::{Collection, CollectionInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, COLLECTION, PermissionEvent, PermissionEventType};
use std::result::Result;
use anchor_lang::prelude::Error as AnchorError;

use crate::errors::ErrorCode;

#[event]
pub struct CollectionEventCreate {
    pub authority: Pubkey,
    pub name: String,
    pub id: Pubkey,    
}


#[derive(Accounts)]
#[instruction(collection_input: CollectionInput)]
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

pub fn handler(ctx: Context<CreateCollection>,
               collection_input: CollectionInput,
) -> anchor_lang::Result<()> {


    let group = &mut ctx.accounts.collection;
    let authority = &mut ctx.accounts.authority;
    group.creator = authority.key();
    group.update_authority = authority.key();
    group.seed = ctx.accounts.seed.key();
    group.item_count = 0;
    group.update_authority = authority.key();
   
    update_collection_from_input(collection_input, group)?;

    emit!(CollectionEventCreate{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key(),
    });


    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    emit!(PermissionEvent {
        reference: ctx.accounts.collection.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });
    
    Ok(())
}

pub fn update_collection_from_input(group_input: CollectionInput, 
    group: &mut Box<Account<Collection>>) 
    -> Result<(), AnchorError> {
    let CollectionInput {name, symbol, 
        // collection_render_mode, 
        royalties,
        permitted_signers,
        attribute_types,
        url,
        description,
    } = group_input;
    let name_length = name.len();
    let symbol_length = symbol.len();
    if name_length > MAX_NAME_LENGTH || symbol_length > MAX_SYMBOL_LENGTH {
        return Err(error!(ErrorCode::InvalidStringInput));
    }
    if royalties.is_some() {
        let royalties_data = royalties.as_ref().unwrap();
        let royalty_bps = royalties_data.bps;

        // Ensure that basis points are between 0-10,000
        if royalty_bps > 10_000 {
            return Err(error!(ErrorCode::InvalidBpsInput));
        }
    }
    
    group.name = name.clone();
    group.symbol = symbol;
    // commenting out until we 
    // figure out a way to expand the 
    // instruction input size limit
    group.description = description;
    // collection.collection_render_mode = collection_render_mode;
    group.royalties = royalties;
    group.attribute_types = attribute_types;
    group.permitted_signers = permitted_signers;
    group.url = url;
    
    
    Ok(())
}