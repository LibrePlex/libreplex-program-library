
use anchor_lang::prelude::*;

use crate::{AttributeConfig};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AttributeUpdate {
    index: u32,
    data: Vec<u8>,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct EditAttributeConfigInput {
    update: Vec<AttributeUpdate>,
    add: Vec<Vec<u8>>,
}

pub const ATTRIBUTE_DATA_START: usize = 8 + 32 + 4 + 4;

#[derive(Accounts)]
pub struct InitializeAttributeConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(zero)]
    pub attribute_config: Account<'info, AttributeConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditAttributeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub attribute_config: Account<'info, AttributeConfig>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<InitializeAttributeConfig>, max_onchain_attribute_count: u32) -> Result<()> {
    let attribute_config = &mut ctx.accounts.attribute_config;

    attribute_config.max_onchain_attribute_count = max_onchain_attribute_count;
    attribute_config.authority = ctx.accounts.signer.key();
    attribute_config.current = 0;

    Ok(())
}

pub fn edit(ctx: Context<EditAttributeConfig>, input: EditAttributeConfigInput) -> Result<()> {
    let attribute_config = &mut ctx.accounts.attribute_config;
    let attribute_config_info: AccountInfo = attribute_config.to_account_info();

    let mut attribute_config_data = attribute_config_info.data.borrow_mut();

    input.update.iter().for_each(|update| {
        let index = update.index;
        let new_data = &update.data;
        let edit_index = ATTRIBUTE_DATA_START + (index as usize) * (attribute_config.max_onchain_attribute_count as usize);

        let new_data_slice = new_data.as_slice();

        attribute_config_data[edit_index..edit_index + (attribute_config.max_onchain_attribute_count as usize)].copy_from_slice(new_data_slice);
    });


    input.add.iter().for_each(|new_data| {
        let edit_index = ATTRIBUTE_DATA_START + (attribute_config.current as usize) * (attribute_config.max_onchain_attribute_count as usize);

        let new_data_slice = new_data.as_slice();

        attribute_config_data[edit_index..edit_index + (attribute_config.max_onchain_attribute_count as usize)].copy_from_slice(new_data_slice);


        attribute_config.current += 1;
    });



    Ok(())
}
