use crate::errors::ErrorCode;

use crate::instructions::{SignerType, InscriptionEventCreate};
use crate::{
    Inscription, InscriptionData, InscriptionSummary, InscriptionV3, InscriptionV3EventData, InscriptionEventData,
};

use crate::legacy_inscriber;
#[event]
pub struct InscriptionV3EventCreate {
    pub id: Pubkey,
    pub data: InscriptionV3EventData,
}

use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionInputV3 {
    pub authority: Option<Pubkey>,
    // each rank page holds a maximum of 320000 inscription ids.
    // when this runs out, we move onto the next page
    pub signer_type: SignerType,
    pub validation_hash: Option<String>
}


impl CreateInscriptionInputV3 {
    pub fn get_size(&self) -> usize {
            1
            + match self.authority {
                Some(_) => 32,
                None => 0,
            } 
            + 2 // default media type length
            + 1 + match &self.validation_hash {
                Some(x)=> x.len() + 4,
                None => 0
            }
    }
}

/*
   This endpoint generates an inscription with v3 config only.
   It does not use page_rank accounts either as those
   are no longer populated.
   v1 and v2 endpoints will be retained for backwards
   compatibility. However anybody using v1 / v2 endpoints
   should migrate to this endpoint as the historical v1/v2
   inscription configs are no longer needed or used.

   the reason we moved to v3 inscription configs is ease
   of indexability compared with the original field ordering.

   none of this touches inscription data as that as always
   been stored in a separate account.
*/

const INITIAL_SIZE: usize = 8;
#[derive(Accounts)]
#[instruction(inscription_input: CreateInscriptionInputV3)]
pub struct CreateInscriptionV3<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // this must be either the root or else a PDA
    // that is accepted as a proxy
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Checked in logic
    pub root: UncheckedAccount<'info>,

    #[account(init_if_needed, seeds = [b"inscription_summary"],
        bump, payer = payer, space = InscriptionSummary::BASE_SIZE)]
    pub inscription_summary: Box<Account<'info, InscriptionSummary>>,

    /*
        generated as a PDA to make sure that each chunk address
        can be derived from the base address
    */
    /// CHECK: Created outside and zero
    #[account(
        init,
        payer = payer,
        // starts with base anchor discriminator, although this will be overwritten
        // by data
        space = INITIAL_SIZE, // set a base rent for now, reduce later
        seeds=[
            "inscription_data".as_bytes(),
            root.key().as_ref()
        ],
        bump)]
    pub inscription_data: Account<'info, InscriptionData>,

    /// CHECK: validated in logic
    #[account(init,
        space = Inscription::BASE_SIZE + inscription_input.get_size(),
        seeds=[
            "inscription_v3".as_bytes(),
            root.key().as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription_v3: Account<'info, InscriptionV3>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateInscriptionV3>, input: CreateInscriptionInputV3) -> Result<()> {
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_summary = &mut ctx.accounts.inscription_summary;

    let authority = match input.authority {
        Some(x) => x.to_owned(),
        None => ctx.accounts.payer.key(),
    };
    let inscription_data = &ctx.accounts.inscription_data;

    let clock = Clock::get()?;

    inscription_summary.last_inscription_create_time = clock.unix_timestamp;
    inscription_summary.last_inscription = inscription_v3.key();
    inscription_summary.last_inscriber = ctx.accounts.payer.key();
    inscription_summary.inscription_count_total += 1;

    // inscription v2
    inscription_v3.authority = authority;
    inscription_v3.size = INITIAL_SIZE as u32;
    inscription_v3.inscription_data = inscription_data.key();
    inscription_v3.root = ctx.accounts.root.key();
    inscription_v3.content_type = "".to_owned();
    inscription_v3.encoding = "".to_owned();
    inscription_v3.validation_hash = input.validation_hash.clone();
    inscription_v3.order = inscription_summary.inscription_count_total;

    let signer = ctx.accounts.signer.key();
    let root_key = inscription_v3.root.key();

    // check signer - it must be either the mint itself
    // or a PDA signed by an authorised signer program

    // is the signer an authorised PDA?
    match input.signer_type {
        SignerType::Root => {
            if signer != inscription_v3.root {
                return Err(ErrorCode::RootSignerMismatch.into());
            }
        }
        SignerType::LegacyMetadataSigner => {
            let expected_signer =
                Pubkey::find_program_address(&[root_key.as_ref()], &legacy_inscriber::ID).0;
            if expected_signer != signer {
                return Err(ErrorCode::LegacyMetadataSignerMismatch.into());
            }
        }
        SignerType::FairLaunchGhostRootSigner => {
            return Err(ErrorCode::RootSignerMismatch.into());
        }
    }

    emit!(InscriptionV3EventCreate {
        id: inscription_v3.key(),
        data: InscriptionV3EventData {
            authority: inscription_v3.authority,
            root: inscription_v3.root,
            content_type: inscription_v3.content_type.clone(),
            encoding: inscription_v3.encoding.clone(),
            inscription_data: inscription_v3.inscription_data,
            order: inscription_v3.order,
            size: inscription_v3.size,
            validation_hash: inscription_v3.validation_hash.clone()
        }
    });

    // generate backwards compatible events
    let inscription_v1_id = Pubkey::find_program_address(
        &["inscription".as_bytes(), inscription_v3.root.as_ref()]
        , &crate::id()).0;

    emit!(InscriptionEventCreate {
        id: inscription_v1_id.key(),
        data: InscriptionEventData { 
            authority: inscription_v3.authority, 
            root: inscription_v3.root, 
            media_type: crate::MediaType::Custom { media_type: inscription_v3.content_type.clone()},
            encoding_type: crate::EncodingType::None,
            inscription_data: inscription_v3.inscription_data,
            order: inscription_v3.order,
            size: inscription_v3.size,
            validation_hash: inscription_v3.validation_hash.clone()
        }
    });


    Ok(())
}
