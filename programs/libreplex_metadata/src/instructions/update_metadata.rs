use anchor_lang::prelude::*;

use crate::Metadata;


#[event]
pub struct EditMetadataEvent {
    pub id: Pubkey,
    pub name: String,
    pub symbol: String,
    pub url_json: String,
}



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMetadataInput {
    pub update_authority: Option<Pubkey>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub url_json: Option<String>
}


// Who can edit the Metadata?
// The update authority!
// The editor of delegated_metadata_specific_permissions

// If part of a group
// The group authority
// The editor with delegated group wide permissions

#[derive(Accounts)]
#[instruction(input: UpdateMetadataInput)]
pub struct UpdateMetadata<'info> {
    
    // as usual, keep payer and update authority
    // separate as this allows for PDA signers
    
    pub payer: Signer<'info>,

    pub update_authority: Signer<'info>,

    #[account(mut,
        constraint = metadata.update_authority == update_authority.key())]
    pub metadata: Box<Account<'info, Metadata>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMetadata>,
    // same input as create
    input: UpdateMetadataInput,
) -> Result<()> {
    

    let UpdateMetadataInput {name, 
            symbol,
            url_json,
            update_authority
        } = input;

    
    let metadata = &mut ctx.accounts.metadata;

    if let Some(x) = name {
        metadata.name = x
    }

    if let Some(x) = symbol {
        metadata.symbol = x
    }

    if let Some(x) = url_json {
        metadata.url_json = x
    }


    if let Some(x) = update_authority {
        metadata.update_authority = x
    }
    
    
    emit!(EditMetadataEvent{
        id: metadata.key(),
        name: metadata.name.clone(),
        symbol: metadata.symbol.clone(),
        url_json: metadata.url_json.clone()
    });

    Ok(())
}
