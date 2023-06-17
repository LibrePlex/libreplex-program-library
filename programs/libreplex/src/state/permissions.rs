use anchor_lang::prelude::*;
use prog_common::{errors::ErrorCode};



#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum PermissionEventType {
    Update, Delete
}

#[event]
pub struct PermissionEvent {
    pub reference: Pubkey,
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


pub fn assert_valid_permissions(permissions: &Account<DelegatePermissions>, 
    reference: Pubkey, user: Pubkey, permission_type: &PermissionType) -> Result<()> {

    // check derivation

    let path = &[b"permissions", reference.as_ref(), user.as_ref()];

    let (key, bump) = Pubkey::find_program_address(path, &crate::id());

    if permissions.key() != key {
        return Err(ErrorCode::UnexpectedPermissionsKey.into());
    }

   
    if permissions.bump != bump {
        return Err(ErrorCode::InvalidBump.into());
    } 


    if permissions.permissions.clone().into_iter().find(|x|(x.eq(permission_type))).is_none() {
        return Err(ErrorCode::InvalidPermissions.into());
    }

    Ok(())
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum PermissionType {
    Admin,
}

#[account]
pub struct DelegatePermissions {
    pub bump: u8,
    pub user: Pubkey,
    pub reference: Pubkey,
    pub permissions: Vec<PermissionType>,
    pub update_authority: Pubkey,
}

impl DelegatePermissions {

    pub const BASE_SIZE: usize = 8 + 1 + 32 + 32 + 4;

    pub fn get_size(&self) -> usize {
        return DelegatePermissions::BASE_SIZE + self.permissions.len();
    }
}