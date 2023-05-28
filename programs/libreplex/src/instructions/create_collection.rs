use anchor_lang::prelude::*;
use crate::state::{Collection, CollectionInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URL_LENGTH, PERMISSIONS_SIZE, CollectionPermissions, COLLECTION, PermissionEvent, PermissionEventType};


use prog_common::{errors::ErrorCode};

#[event]
struct CreateCollectionEvent {
    id: Pubkey,
    creator: Pubkey,
    name: String,
}

#[derive(Accounts)]
#[instruction(collection_input: CollectionInput)]
pub struct CreateCollection<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, 
        payer = authority, 
        space = PERMISSIONS_SIZE, 
        seeds = ["permissions".as_ref(), collection.key().as_ref(), authority.key().as_ref()], 
        bump)]
    pub user_permissions: Box<Account<'info, CollectionPermissions>>,

    #[account(init, seeds = [COLLECTION.as_ref(), seed.key().as_ref()],
      bump, payer = authority, space = 8 + 72 + collection_input.get_size())]
    pub collection: Box<Account<'info, Collection>>,


    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCollection>,
               collection_input: CollectionInput,
) -> Result<()> {

    let CollectionInput {name, symbol, collection_url, nft_collection_data} = collection_input;

    // Ensure that the lengths of strings do not exceed the maximum allowed length
    let name_length = name.len();
    let symbol_length = symbol.len();
    let url_length = collection_url.len();

    if name_length > MAX_NAME_LENGTH || symbol_length > MAX_SYMBOL_LENGTH || url_length > MAX_URL_LENGTH {
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

    // Update the collection data state account
    let collection = &mut ctx.accounts.collection;
    collection.seed = ctx.accounts.seed.key();
    collection.name = name.clone();
    collection.symbol = symbol;
    collection.url = collection_url;
    collection.item_count = 0;
    collection.nft_collection_data = nft_collection_data;

    emit!(CreateCollectionEvent{
        creator: ctx.accounts.authority.key(),
        name,
        id: collection.key(),
    });


    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    let user_permissions = &mut ctx.accounts.user_permissions;
    user_permissions.collection = collection.key();
    user_permissions.user = ctx.accounts.authority.key();
    user_permissions.can_create_metadata = true;
    user_permissions.can_edit_metadata = true;
    user_permissions.can_delete_metadata =  true;
    user_permissions.is_admin = true;
    user_permissions.can_delete_collection = true;

    
    emit!(PermissionEvent {
        collection: ctx.accounts.collection.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });
    
    Ok(())
}
