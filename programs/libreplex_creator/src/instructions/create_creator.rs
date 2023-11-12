use anchor_lang::prelude::*;


use crate::{AccountEvent, Creator, AccountEventType, AssetUrl, MintNumbers, errors::ErrorCode, MINT_NUMBERS_START};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateCreatorInput {
    pub max_mints: u32,
    pub seed: Pubkey,
    pub symbol: String,
    pub asset_url: AssetUrl,
    pub collection: Pubkey,
    pub description: Option<String>,
    pub attribute_mappings: Option<Pubkey>,
    pub mint_authority: Pubkey,
    pub is_ordered: bool,
    pub name: String,
}

impl CreateCreatorInput {
    pub fn get_size (&self) -> usize {
        8 + 8 + 32 + 4
    }
}

#[derive(Accounts)]
#[instruction(create_creator_input: CreateCreatorInput)]
pub struct CreateCreator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"creator", create_creator_input.seed.key().as_ref()],
              bump, payer = signer, space = Creator::BASE_SIZE + create_creator_input.get_size())]
    pub creator: Box<Account<'info, Creator>>,

    #[account(zero)]
    pub minter_numbers: Option<Box<Account<'info, MintNumbers>>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCreator>, input: CreateCreatorInput) -> Result<()> {
    let creator = &mut ctx.accounts.creator;

    creator.update_authority = ctx.accounts.signer.key();
    creator.seed = input.seed.key();
    creator.supply = input.max_mints;
    creator.symbol = input.symbol;
    creator.asset_url = input.asset_url;

    if !input.is_ordered {
        let mint_numbers = ctx.accounts.minter_numbers.as_ref().ok_or(ErrorCode::MissingMintNumbers)?;

        creator.minter_numbers = Some(mint_numbers.key());

        let mint_numbers_info: &AccountInfo = mint_numbers.as_ref().as_ref();
        let mut mint_numbers_data = mint_numbers_info.data.borrow_mut();

        let base_offset = MINT_NUMBERS_START;
        for i in 0..creator.supply {
            let bytes = i.to_le_bytes();
            let offset = base_offset + (i as usize) * 4;

            mint_numbers_data[offset..offset + 4].copy_from_slice(&bytes);
        }
    }

    creator.name = input.name;
    creator.collection = input.collection;
    creator.bump = ctx.bumps.creator;
    creator.description = input.description;
    creator.attribute_mappings = input.attribute_mappings;

    creator.creator_authority = input.mint_authority;
    creator.is_ordered = input.is_ordered;

    emit!(AccountEvent {
        reference: creator.key(),
        event_type: AccountEventType::Create,
    });

    Ok(())
}
