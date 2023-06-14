use crate::state::Metadata;
use crate::{
    CreateMetadataInput, MetadataEvent, MetadataEventType, Ordinal, OrdinalEvent, OrdinalEventType,
    PermissionType, Permissions,
};
use anchor_lang::prelude::*;

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AppendToOrdinalInput {
    pub append_data: Vec<u8>,
}

impl AppendToOrdinalInput {
    pub fn get_size(&self) -> usize {
        return 4 + self.append_data.len();
    }
}

#[derive(Accounts)]
#[instruction(ordinal_input: AppendToOrdinalInput)]
pub struct AppendToOrdinal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(zero)]
    #[account(mut)]
    pub ordinal: Account<'info, Ordinal>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AppendToOrdinal>,
    append_to_ordinal_input: AppendToOrdinalInput,
) -> Result<()> {
    let ordinal = &mut ctx.accounts.ordinal;


    let mut ordinal_account_info = ordinal.to_account_info();

    ordinal.append_data(ordinal_account_info.data.borrow_mut(), 
        &append_to_ordinal_input.append_data)?;

    emit!(OrdinalEvent {
        id: ordinal.key(),
        event_type: OrdinalEventType::Create
    });

    Ok(())
}
