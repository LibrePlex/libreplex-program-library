use anchor_lang::prelude::*;
use prog_common::{errors::ErrorCode};



#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum PermissionEventType {
    Update, Delete
}

#[event]
pub struct PermissionEvent {
    pub collection: Pubkey,
    pub user: Pubkey,
    pub event_type: PermissionEventType,
}

#[event]
pub struct MetadataPermissionEvent {
    pub metadata: Pubkey,
    pub user: Pubkey,
}

pub const PERMISSIONS_SIZE: usize = 32 + 32 + 1 + 1 
// Padding
+ 30;


pub fn assert_valid_collection_permissions(permissions: &CollectionPermissions, 
    collection: &Pubkey, user: &Pubkey) -> Result<()> {
    let valid = &permissions.collection == collection && &permissions.user == user;

    if !valid {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    Ok(())
}

pub fn assert_valid_metadata_permissions(permissions: &MetadataPermissions, 
    metadata: &Pubkey, user: &Pubkey) -> Result<()> {
    let valid = &permissions.metadata == metadata && &permissions.user == user;

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

    // admin permission includes
    // the ability to change permissions
    pub is_admin: bool,
    
    pub can_create_metadata: bool,
    pub can_edit_metadata: bool,
    pub can_delete_metadata: bool,
    
    pub can_edit_collection: bool,
    pub can_delete_collection: bool,
}
