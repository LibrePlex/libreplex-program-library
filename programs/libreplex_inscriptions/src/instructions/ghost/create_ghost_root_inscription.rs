use crate::errors::ErrorCode;

use crate::instructions::create_inscription_v2::legacy_inscriber;
use crate::instructions::{SignerType, InscriptionV3EventCreate};
use crate::{
    Inscription, InscriptionData, InscriptionSummary, InscriptionV3, InscriptionV3EventData,
};

use anchor_lang::prelude::*;

mod fair_launch_inscriber {
    use super::*;
    declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateGhostRootInscriptionInput{
    pub authority: Option<Pubkey>,
    // each rank page holds a maximum of 320000 inscription ids.
    // when this runs out, we move onto the next page
    pub signer_type: SignerType,
    pub validation_hash: Option<String>,
    pub root: Pubkey,
}


impl CreateGhostRootInscriptionInput {
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
   This endpoint is identical to v3 create.
   However it only accepts LegacyMetadataSigners
   and the root is passed in as an argument.

   This is for roots that cannot exist as accounts on chain.
*/

const INITIAL_SIZE: usize = 8;
#[derive(Accounts)]
#[instruction(inscription_input: CreateGhostRootInscriptionInput)]
pub struct CreateGhostRootInscription<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // this must be either the root or else a PDA
    // that is accepted as a proxy
    #[account(mut)]
    pub signer: Signer<'info>,

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
            inscription_input.root.as_ref()
        ],
        bump)]
    pub inscription_data: Account<'info, InscriptionData>,

    /// CHECK: validated in logic
    #[account(init,
        space = Inscription::BASE_SIZE + inscription_input.get_size(),
        seeds=[
            "inscription_v3".as_bytes(),
            inscription_input.root.as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription_v3: Account<'info, InscriptionV3>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGhostRootInscription>, input: CreateGhostRootInscriptionInput) -> Result<()> {
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
    inscription_v3.root = input.root;
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
            return Err(ErrorCode::LegacyMetadataSignerMismatch.into());
        }
        SignerType::LegacyMetadataSigner => {
            let expected_signer =
                Pubkey::find_program_address(&[root_key.as_ref()], 
                &legacy_inscriber::id()).0;
            if expected_signer != signer {
                return Err(ErrorCode::LegacyMetadataSignerMismatch.into());
            }
        }
        SignerType::FairLaunchGhostRootSigner => {
            let expected_signer =
                Pubkey::find_program_address(&[root_key.as_ref()], 
                &fair_launch_inscriber::id()).0;

            if expected_signer != signer {
                return Err(ErrorCode::LegacyMetadataSignerMismatch.into());
            }
        },
    }

    // for now, only fire events for inscription v1
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

    Ok(())
}
