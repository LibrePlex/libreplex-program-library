use anchor_lang::prelude::*;

use prog_common::{errors::ErrorCode};

use crate::Permissions;

/* 
    Intended for cleaning up one's own permissions Ø
    after the collection has been deleted.
 */
#[derive(Accounts)]
pub struct DeletePermissions<'info> {
    pub signer: Signer<'info>,

    #[account(mut,
        close = receiver,
        seeds = [
            "permissions".as_ref(), 
            collection.key().as_ref(), 
            signer.key().as_ref()], 
        bump)]
    pub permissions: Box<Account<'info, Permissions>>,

    /*  
        this account must be empty before permissions can be deleted 
        via this method. That's because no further validation can be 
        performed if the collection does not exist.
        TODO: Enable deleting appropriate permissions as admin for 
        a non-deleted collection.
    */
    /// CHECK: Checked for empty in logic
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,

    /// CHECK: Receiver address for the rent-exempt lamports
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeletePermissions>) -> Result<()> {
    let permissions = &ctx.accounts.permissions;
    let collection = &ctx.accounts.collection;

    if !collection.data_is_empty() {
        return Err(ErrorCode::CollectionExists.into())
    }


    // assert_valid_user_permissions(
    //     permissions,
    //     &ctx.accounts.collection.key(),
    //     ctx.accounts.signer.key,
    // )?;

    // if !permissions.is_admin {
    //     return Err(ErrorCode::MissingPermissionAdmin.into());
    // }


    msg!(
        "Collection permissions with pubkey {} now deleted",
        permissions.key()
    );
    Ok(())
}