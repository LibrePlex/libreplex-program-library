use anchor_lang::prelude::*;
use prog_common::{errors::ErrorCode};


pub const PERMISSIONS_SIZE: usize = 32 + 32 + 1 + 1 
// Padding
+ 30;


pub fn assert_valid_user_permissions(permissions: &CollectionPermissions, 
    collection: &Pubkey, user: &Pubkey) -> Result<()> {
    let valid = &permissions.collection == collection && &permissions.user == user;

    if !valid {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    Ok(())
}

#[account]
#[derive(Debug)]
pub struct MetadataPermissions {
    pub metadata: Pubkey,
    pub user: Pubkey,
    pub can_modify: bool,

    // Maybe we will add more
}


#[account]
#[derive(Debug)]
pub struct CollectionPermissions {
    pub collection: Pubkey,
    pub user: Pubkey,

    pub can_edit_permissions: bool,
    pub can_add_metadatas: bool,
    pub can_edit_metadatas: bool,
    pub can_delete_metadatas: bool,
    pub can_delete_collection: bool,
}

