use anchor_lang::prelude::*;
use crate::state::{Group, GroupInput};
use crate::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, GROUP, PermissionEvent, PermissionEventType, GroupEventType, GroupEvent};
use std::result::Result;
use anchor_lang::prelude::Error as AnchorError;

use prog_common::{errors::ErrorCode};



#[derive(Accounts)]
#[instruction(group_input: GroupInput)]
pub struct CreateGroup<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, seeds = [GROUP.as_ref(), seed.key().as_ref()],
        bump, payer = authority, space = Group::BASE_SIZE + group_input.get_size())]
    pub group: Box<Account<'info, Group>>,

    /// CHECK: The seed address used for initialization of the collection PDA
    pub seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGroup>,
               group_input: GroupInput,
) -> anchor_lang::Result<()> {


    let group = &mut ctx.accounts.group;
    let authority = &mut ctx.accounts.authority;
    group.creator = authority.key();
    group.update_authority = authority.key();
    group.seed = ctx.accounts.seed.key();
    group.item_count = 0;
    group.update_authority = authority.key();


    update_collection_from_input(group_input, group)?;


    emit!(GroupEvent{
        authority: ctx.accounts.authority.key(),
        name: group.name.clone(),
        id: group.key(),
        event_type: GroupEventType::Create
    });


    msg!("Collection data created with authority pubkey {}", ctx.accounts.authority.key());

    emit!(PermissionEvent {
        reference: ctx.accounts.group.key(),
        user: ctx.accounts.authority.key(),
        event_type: PermissionEventType::Update,
    });
    
    Ok(())
}

pub fn update_collection_from_input<'a>(group_input: GroupInput, 
    group: &mut Box<Account<Group>>) 
    -> Result<(), AnchorError> {
    let GroupInput {name, symbol, 
        metadata_render_mode, 
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

        let royalty_shares_vec: Vec<u16> = royalties_data.shares.iter().map(|x| x.share).collect();

        for rs in royalty_shares_vec {
            if rs != 10_000 {
                return Err(error!(ErrorCode::InvalidBpsInput));
            }
        }
    }
    
    group.name = name.clone();
    group.symbol = symbol;
    // commenting out until we 
    // figure out a way to expand the 
    // instruction input size limit
    group.description = description;
    // collection.collection_render_mode = collection_render_mode;
    group.metadata_render_mode = metadata_render_mode;
    group.royalties = royalties;
    group.attribute_types = attribute_types;
    group.permitted_signers = permitted_signers;
    group.url = url;
    
    
    Ok(())
}