import { Program } from "@coral-xyz/anchor"
import { PublicKey, SystemProgram } from "@solana/web3.js"
import {LibreplexMetadata} from "@libreplex/idls/lib/types/libreplex_metadata";

import { LIBREPLEX_METADATA_PROGRAM_ID } from "./constants";

export enum UserPermission {
    Update,
    Delete,
    AddToGroup,
  }
  
  export function getGroupWiderUserPermissionsAddress(group: PublicKey, user: PublicKey, program = LIBREPLEX_METADATA_PROGRAM_ID) {
    return  PublicKey.findProgramAddressSync([Buffer.from("permissions"), user.toBuffer(), group.toBuffer()], program)[0]

  }
  
  function convertPermission(p: UserPermission) {
    if (p === UserPermission.AddToGroup) {
      return {
        addToGroup: {}
      }
    }
  
    if (p === UserPermission.Delete) {
      return {
        addToGroup: {}
      }
    }
  
     if (p === UserPermission.Update) {
      return {
        addToGroup: {}
      }
    }
    
    throw new Error("Invalid permission enum")
  }


export async function setUserPermissionsForGroup(
    {
      metadataProgram,
      group,
      user,
      permissions,
      groupUpdateAuthority,
    }: {
      metadataProgram: Program<LibreplexMetadata>,
      group: PublicKey,
      user: PublicKey,
      permissions: UserPermission[],
      groupUpdateAuthority: PublicKey,
    }
  ) {
    const permissionsAccountAddress = getGroupWiderUserPermissionsAddress(group, user)  
  
    const existingPermissionsInfo = await metadataProgram.provider.connection.getAccountInfo(permissionsAccountAddress)
  
    const anchorPermissions = permissions.map(convertPermission);
  
    if (!existingPermissionsInfo) {
      return metadataProgram.methods.delegateGroupPermissions({
        permissions: anchorPermissions,
      }).accounts({
        group,
        delegatedUser: user,
        systemProgram: SystemProgram.programId,
        updateAuthority: groupUpdateAuthority,
        userPermissions: permissionsAccountAddress
      })
    }
  
    return metadataProgram.methods.updatePermissions({
      permissions: anchorPermissions
    }).accounts({
      updateAuthority: groupUpdateAuthority,
      user: user,
      userPermissions: permissionsAccountAddress
    })
  }
  