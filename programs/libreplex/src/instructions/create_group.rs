use anchor_lang::prelude::*;
use crate::state::{Group, GroupInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, PERMISSIONS_SIZE, COLLECTION, PermissionEvent, PermissionEventType, Permissions, PermissionType, PERMISSIONS};
use std::result::Result;
use anchor_lang::prelude::Error as AnchorError;

use prog_common::{errors::ErrorCode};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum GroupEventType {
    Create,
    Edit,
    Delete
}

#[event]
pub struct GroupEvent {
    pub id: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub event_type: GroupEventType
}

#[derive(Accounts)]
#[instruction(collection_input: GroupInput)]
pub struct CreateGroup<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = [PERMISSIONS.as_ref(), collection.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,

    #[account(init, seeds = [COLLECTION.as_ref(), seed.key().as_ref()],
      bump, payer = authority, space = Group::BASE_SIZE + collection_input.get_size())]
    pub collection: Box<Account<'info, Group>>,


    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGroup>,
               collection_input: GroupInput,
) -> anchor_lang::Result<()> {


    let collection = &mut ctx.accounts.collection;
    let authority = &mut ctx.accounts.authority;
    collection.creator = authority.key();
    collection.seed = ctx.accounts.seed.key();
    collection.item_count = 0;
    

    update_collection_from_input(collection_input, collection)?;

    
    


    emit!(GroupEvent{
        creator: ctx.accounts.authority.key(),
        name: collection.name.clone(),
        id: collection.key(),
        event_type: GroupEventType::Create
    });


    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    let permissions = &mut ctx.accounts.permissions;
    
    permissions.permissions = vec![PermissionType::Admin];
    
    emit!(PermissionEvent {
        reference: ctx.accounts.collection.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });
    
    Ok(())
}

pub fn update_collection_from_input<'a>(collection_input: GroupInput, 
    collection: &mut Box<Account<Group>>) 
    -> Result<(), AnchorError> {
    let GroupInput {name, symbol, 
        metadata_render_mode, 
        collection_render_mode, 
        nft_collection_data
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
    collection.collection_render_mode = collection_render_mode;
    collection.metadata_render_mode = metadata_render_mode;
    collection.nft_collection_data = nft_collection_data;
    
    
    Ok(())
}