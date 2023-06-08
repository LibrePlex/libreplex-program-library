use crate::state::{Group, Metadata};
use crate::{
    assert_valid_permissions, MetadataExtended, PermissionType, Permissions, RoyaltyShare, Royalties,
};
use anchor_lang::prelude::*;

use anchor_spl::token::Mint;
use prog_common::{errors::ErrorCode, TryAdd};

#[event]
struct ExtendMetadataEvent {
    id: Pubkey,
    group: Pubkey,
    mint: Pubkey,
}


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ExtendMetadataInput {
    pub attributes: Vec<u8>,  // base: 4
    pub royalties: Option<Royalties>,
    pub invoked_permission: PermissionType,
}

impl ExtendMetadataInput {
    pub const BASE_SIZE: usize = 4 + 1 + 1;

    pub fn get_size(&self) -> usize {
        ExtendMetadataInput::BASE_SIZE
            + self.attributes.len()
            + match &self.royalties {
                Some(x) => x.get_size(),
                None => 0,
            }
    }
}

pub fn validate_extend_metadata_input(
    metadata_input: &ExtendMetadataInput,
    group: &Group,
) -> Result<()> {
  
    match &metadata_input.royalties {

        Some(royalties)=>{
            let total_shares: u16 = royalties.shares.iter().map(|x|x.share).sum();
            if total_shares != 10000 {
                return Err(ErrorCode::RoyaltiesBadSum.into())
            }
        },
        None =>{}
    }
    


    /*
        ensure that the initial render mode of the metadata matches the
        currently active render mode of the collection.

        NB: It is possible to change the active render mode of the collection.
        If that happens, it is the responsibility of the update auth holder
        to add the appropriate render mode data to each metadata.

    */

    // render_mode_data.is_compatible_with(&collection.collection_render_mode);

    // Ensure that the lengths of strings do not exceed the maximum allowed length

    Ok(())
}

#[derive(Accounts)]
#[instruction(extend_metadata_input: ExtendMetadataInput)]
pub struct ExtendMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /*
       To extend metadata and to add it to a group, you must have Admin access to both
       group and the metadata
    */
    #[account(
        seeds = ["permissions".as_ref(), group.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub group_permissions: Box<Account<'info, Permissions>>,

    #[account(
        seeds = ["permissions".as_ref(), metadata.key().as_ref(), signer.key().as_ref()], 
        bump)]
    pub metadata_permissions: Box<Account<'info, Permissions>>,

    #[account(mut)]
    pub group: Box<Account<'info, Group>>,

    #[account(seeds = [b"metadata".as_ref(), mint.key().as_ref()],
              bump)]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(init, seeds = [b"metadata_extended".as_ref(), metadata.key().as_ref()],
              bump, payer = signer, space = MetadataExtended::BASE_SIZE + extend_metadata_input.get_size())]
    pub metadata_extended: Box<Account<'info, MetadataExtended>>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ExtendMetadata>,
    extend_metadata_input: ExtendMetadataInput,
) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let metadata_extended = &mut ctx.accounts.metadata_extended;
    let group = &mut ctx.accounts.group;
    let group_permissions = &ctx.accounts.group_permissions;
    let metadata_permissions = &ctx.accounts.metadata_permissions;
    let authority = &ctx.accounts.signer;

    validate_extend_metadata_input(&extend_metadata_input, &group)?;

    let ExtendMetadataInput {
        attributes,
        royalties,
        invoked_permission,
    } = extend_metadata_input;

    if invoked_permission != PermissionType::Admin {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    // ensure we have permissions on the group
    assert_valid_permissions(
        group_permissions,
        group.key(),
        authority.key(),
        &invoked_permission,
    )?;

    // ensure we have permissions on the metadata
    assert_valid_permissions(
        metadata_permissions,
        metadata.key(),
        authority.key(),
        &invoked_permission,
    )?;

    // ensure that the mint is in fact a mint
    Account::<Mint>::try_from(&ctx.accounts.mint.to_account_info())?;

    // Update the metadata state account
    // metadata_extended.render_mode_data = vec![extend_metadata_input.render_mode_data];
    metadata_extended.group = group.key();
    metadata_extended.attributes = attributes;
    metadata_extended.royalties = royalties;
    metadata_extended.signers = vec![]; // signers always start out empty

    // Increment collection data counter
    group.item_count.try_add_assign(1)?;

    msg!(
        "metadata created for mint with pubkey {}",
        ctx.accounts.mint.key()
    );

    emit!(ExtendMetadataEvent {
        group: group.key(),
        id: metadata_extended.key(),
        mint: ctx.accounts.mint.key(),
    });

    Ok(())
}
