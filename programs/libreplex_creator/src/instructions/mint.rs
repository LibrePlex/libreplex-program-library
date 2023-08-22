/*
    Creates a metadata for a pre existing mint, 
    wraps and ensures it as an NFT and adds it to the required Group.
*/

use anchor_lang::prelude::*;
use arrayref::array_ref;
use libreplex_metadata::{Group, CreateMetadataInput};

use crate::{Creator, AssetUrl, MintNumbers, errors::ErrorCode, MINT_NUMBERS_START, AttributeConfig};

use super::attributes::ATTRIBUTE_DATA_START;

use anchor_spl::{token_2022::ID as TOKEN_2022_PROGRAM_ID, token_interface::TokenAccount};
use libreplex_nft::ID as LIBREPLEX_NFT_PROGRAM_ID;

#[event]
pub struct MintEvent {
    pub group: Pubkey,
    pub number: u32,
    pub authority: Pubkey,
    pub holder: Pubkey,
    pub mint: Pubkey,
    pub total_minted: u32,
}



#[derive(Accounts)]
pub struct Mint<'info> {
    pub receiver: Signer<'info>,

    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub creator_authority: Signer<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    /// CHECK: address checked
    #[account(mut)]
    pub mint_wrapper: AccountInfo<'info>,

    pub mint_authority: Signer<'info>,

    #[account(mut, has_one = creator_authority)]
    pub creator: Box<Account<'info, Creator>>,

    /// CHECK: checked in cpi
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    #[account(mut)]
    pub group: Box<Account<'info, Group>>,

    /// CHECK: checked in cpi
    #[account(mut)]
    pub group_permissions: AccountInfo<'info>,

    #[account(mut)]
    pub minter_numbers: Option<Account<'info, MintNumbers>>,
    pub attribute_config: Option<Account<'info, AttributeConfig>>,


    pub system_program: Program<'info, System>,


    /// CHECK: address checked
    #[account(address = libreplex_metadata::id())]
    pub libreplex_metadata_program: AccountInfo<'info>,

    
      /// CHECK: address checked
    #[account(address = solana_program::sysvar::slot_hashes::id())]
    pub recent_slothashes: AccountInfo<'info>,

    /// CHECK: address checked
    #[account(address = LIBREPLEX_NFT_PROGRAM_ID)]
    pub libreplex_nft_program: AccountInfo<'info>,

     /// CHECK: address checked
     #[account(address = TOKEN_2022_PROGRAM_ID)]
    pub token_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<Mint>) -> Result<()> {
    let creator = &mut ctx.accounts.creator;
    let creator_seeds = ["creator".as_bytes(), creator.seed.as_ref(), &[creator.bump]];

    let receiver_token_account = &ctx.accounts.receiver_token_account;
    let mint = &ctx.accounts.mint;

    let receiver = &ctx.accounts.receiver;

    if &receiver_token_account.owner != receiver.key || 
        &receiver_token_account.mint != mint.key {
        return Err(ErrorCode::InvalidTokenAccout.into())
    }

    if creator.minted >= creator.supply {
        return Err(ErrorCode::SoldOut.into())
    }

    let create_ix_accounts = libreplex_metadata::cpi::accounts::CreateMetadata {
        metadata: ctx.accounts.metadata.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
        invoked_migrator_program: None,
    };

    let mint_number = match creator.is_ordered {
        true => creator.minted,
        false => {
            let mint_numbers 
                = &mut ctx.accounts.minter_numbers.as_ref().ok_or(ErrorCode::MissingMintNumbers)?;

            if mint_numbers.key() != creator.minter_numbers.ok_or(ErrorCode::MissingMintNumbers)? {
                return Err(ErrorCode::WrongMintNumbers.into());
            }

            select_mint_number(mint_numbers.as_ref(), &ctx.accounts.recent_slothashes, (creator.supply - creator.minted) as usize)
        },
    };

    let signer_seeds = [creator_seeds.as_slice()];

    let create_ctx = CpiContext::new_with_signer(ctx.accounts.libreplex_metadata_program.to_account_info(), 
        create_ix_accounts, 
        signer_seeds.as_slice());

    let asset = match &creator.asset_url {
        AssetUrl::JsonPrefix { url } => {
            libreplex_metadata::Asset::Json { url: format!("{}{}.json", url, mint_number) }
        },
        AssetUrl::ChainRenderer { output_address, program_id, description } => libreplex_metadata::Asset::ChainRenderer { render_output_address: *output_address, description: description.clone(), program_id: *program_id },
    };

    let name = format!("{}{}", creator.name, mint_number);   



    let attributes = match creator.attribute_mappings {
        Some(attribute_config) => {
            let attribute_config_account = ctx.accounts.attribute_config.as_ref().ok_or(ErrorCode::MissingAttributeConfig)?;

            if attribute_config != attribute_config_account.key() {
                return Err(ErrorCode::MissingAttributeConfig.into())
            }

            let attribute_index = ATTRIBUTE_DATA_START + 
                (attribute_config_account.max_onchain_attribute_count as usize) * mint_number as usize;

            let attribute_account_info: &AccountInfo = attribute_config_account.as_ref();
            let attribute_account_data = attribute_account_info.data.borrow();


            let attribute_slice = &attribute_account_data[attribute_index..attribute_index + (attribute_config_account.max_onchain_attribute_count as usize)];

            attribute_slice.to_vec()
        },
        None => vec![],
    };

    // I assume collection level royalties
    libreplex_metadata::cpi::create_metadata(create_ctx, CreateMetadataInput {
        name,
        symbol: creator.symbol.clone(),
        asset,
        update_authority: creator.key(),
        extension: libreplex_metadata::MetadataExtension::Nft { attributes, signers: vec![], royalties: None, license: None },
    })?;
    

    let group_add_accounts = libreplex_metadata::cpi::accounts::GroupAdd {
        payer: ctx.accounts.payer.to_account_info(),
        metadata_authority: creator.to_account_info(),
        group_authority: creator.to_account_info(),
        metadata: ctx.accounts.metadata.to_account_info(),
        delegated_metadata_specific_permissions: None,
        delegated_group_wide_permissions: Some(ctx.accounts.group_permissions.to_account_info()),
        group: ctx.accounts.group.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let group_add_ctx
    = CpiContext::new_with_signer(ctx.accounts.libreplex_metadata_program.to_account_info(), group_add_accounts, signer_seeds.as_slice());

    // Transfers update authority to the group.
    libreplex_metadata::cpi::group_add(group_add_ctx)?;

    creator.minted += 1;

    emit!(MintEvent {
        group: creator.collection,
        authority: creator.update_authority,
        holder: receiver.key(), 
        number: mint_number,
        mint: ctx.accounts.mint.key(),
        total_minted: creator.minted,
    });


    let wrap_ctx = CpiContext::new(ctx.accounts.libreplex_nft_program.to_account_info(), libreplex_nft::cpi::accounts::WrapCtx {
        payer: ctx.accounts.payer.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        wrapped_mint: ctx.accounts.mint_wrapper.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    });

    libreplex_nft::cpi::wrap(wrap_ctx)?;

    Ok(())
}


pub fn select_mint_number(
    mints_numbers_info: &AccountInfo, 
    recent_slothashes: &AccountInfo, 
    mints_left: usize) -> u32 {
        let recent_slothash_data = recent_slothashes.data.borrow();
        let chosen_index 
            = (4 * (u64::from_le_bytes( *array_ref![recent_slothash_data, 12, 8]) % (mints_left as u64)) + (MINT_NUMBERS_START as u64)) as usize;
        let end: usize = MINT_NUMBERS_START + 4 * (mints_left - 1);

        msg!("{}", chosen_index);
        let mut mint_number_data = mints_numbers_info.data.borrow_mut();
        let chosen_value = u32::from_le_bytes(*array_ref![mint_number_data, chosen_index, 4]);

        msg!("{}", chosen_value);
        let end_data = *array_ref![mint_number_data, end, 4];
        mint_number_data[chosen_index..chosen_index + 4]
            .copy_from_slice(&end_data);


        chosen_value
    }   