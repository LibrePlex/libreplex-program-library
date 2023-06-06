use anchor_lang::prelude::*;
use crate::state::{Collection, CollectionInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, PERMISSIONS_SIZE, COLLECTION, PermissionEvent, PermissionEventType, Permissions, PermissionType};
use std::result::Result;
use anchor_lang::prelude::Error as AnchorError;

use prog_common::{errors::ErrorCode};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum CollectionEventType {
    Create,
    Edit,
    Delete
}

#[event]
pub struct CollectionEvent {
    pub id: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub event_type: CollectionEventType
}

#[derive(Accounts)]
#[instruction(collection_input: CollectionInput)]
pub struct CreateCollection<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = ["permissions".as_ref(), collection.key().as_ref(), authority.key().as_ref(), &(PermissionType::Admin as u8).to_le_bytes()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,

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


    let collection = &mut ctx.accounts.collection;
    let authority = &mut ctx.accounts.authority;
    collection.creator = authority.key();
    collection.seed = ctx.accounts.seed.key();
    collection.item_count = 0;
    

    update_collection_from_input(collection_input, collection)?;


    emit!(CollectionEvent{
        creator: ctx.accounts.authority.key(),
        name: collection.name.clone(),
        id: collection.key(),
        event_type: CollectionEventType::Create
    });


    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    emit!(PermissionEvent {
        collection: ctx.accounts.collection.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });
    
    Ok(())
}

pub fn update_collection_from_input<'a>(collection_input: CollectionInput, 
    collection: &mut Box<Account<Collection>>) 
    -> Result<(), AnchorError> {
    let CollectionInput {name, symbol, 
        metadata_render_mode, 
        collection_render_mode, 
        nft_collection_data,
        // description,
    } = collection_input;
    let name_length = name.len();
    let symbol_length = symbol.len();
    if name_length > MAX_NAME_LENGTH || symbol_length > MAX_SYMBOL_LENGTH {
        return Err(error!(ErrorCode::InvalidStringInput));
    }
    if nft_collection_data.is_some() {
        let nft_collection_data_unwrapped = nft_collection_data.as_ref().unwrap();
        let royalty_bps = nft_collection_data_unwrapped.royalty_bps;

        // Ensure that basis points are between 0-10,000
        if royalty_bps > 10_000 {
            return Err(error!(ErrorCode::InvalidBpsInput));
        }

        let royalty_shares_vec: Vec<u16> = nft_collection_data_unwrapped.royalty_shares.iter().map(|x| x.share).collect();

        for rs in royalty_shares_vec {
            if rs > 10_000 {
                return Err(error!(ErrorCode::InvalidBpsInput));
            }
        }
    }
    
    collection.name = name.clone();
    collection.symbol = symbol;
    // commenting out until we 
    // figure out a way to expand the 
    // instruction input size limit
    // collection.description = "".to_owned(); //description;
    collection.collection_render_mode = collection_render_mode;
    collection.metadata_render_mode = metadata_render_mode;
    collection.nft_collection_data = nft_collection_data;
    
    match &collection.nft_collection_data {
        Some(x) => {
            msg!("Create collection with royalties: {} ", x.royalty_bps);
        }, None => {}
    }
    

    Ok(())
}