use crate::errors::ErrorCode;

use crate::{
    Inscription, InscriptionData, InscriptionRankPage,
    InscriptionSummary, MediaType, EncodingType, InscriptionEventData, InscriptionV3,
};
use anchor_lang::prelude::*;




#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionInput {
    pub authority: Option<Pubkey>,
    // each rank page holds a maximum of 320000 inscription ids.
    // when this runs out, we move onto the next page
    pub current_rank_page: u32,
    pub signer_type: SignerType,
    pub validation_hash: Option<String>
}

impl CreateInscriptionInput {
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


const INITIAL_SIZE: usize = 8;
#[derive(Accounts)]
#[instruction(inscription_input: CreateInscriptionInput)]
pub struct CreateInscription<'info> {
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

    #[account(mut,
        // always leave 32 bytes spare at the end. new additions write to the last 32 bytes and add extra 32 bytes
        // space = InscriptionRankPage::BASE_SIZE + 32, 
        // payer = payer,
        seeds = ["inscription_rank".as_bytes(), &inscription_input.current_rank_page.to_le_bytes()],
        bump)]
    pub inscription_ranks_current_page: Box<Account<'info, InscriptionRankPage>>,

    // next page is needed in case the current inscription spills
    // over. it's INSCRIPTIONS_PER_PAGE inscriptions per page so this will happen eventually
    #[account(mut,
        // always leave 32 bytes spare at the end. new additions write to the last 32 bytes and add extra 32 bytes
        // space = InscriptionRankPage::BASE_SIZE,
        // payer = payer,
        seeds = ["inscription_rank".as_bytes(), &(inscription_input.current_rank_page +1).to_le_bytes()],
        bump)]
    pub inscription_ranks_next_page: Box<Account<'info, InscriptionRankPage>>,

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
            "inscription".as_bytes(),
            root.key().as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription: Account<'info, Inscription>,

    /// CHECK: validated in logic
    #[account(init,
        space = Inscription::BASE_SIZE + inscription_input.get_size(), // v1 and v2 have the same size
        seeds=[
            "inscription_v3".as_bytes(),
            root.key().as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription2: Account<'info, InscriptionV3>,

    pub system_program: Program<'info, System>,
}



pub fn handler(ctx: Context<CreateInscription>, input: CreateInscriptionInput) -> Result<()> {

    let inscription = &mut ctx.accounts.inscription;
    let inscription_v2 = &mut ctx.accounts.inscription2;
    let inscription_summary = &mut ctx.accounts.inscription_summary;

    let authority = match input.authority {
        Some(x) => x.to_owned(),
        None => ctx.accounts.payer.key(),
    };
    let inscription_data = &ctx.accounts.inscription_data;

    let clock = Clock::get()?;

    inscription_summary.last_inscription_create_time = clock.unix_timestamp;
    inscription_summary.last_inscription = inscription.key();
    inscription_summary.last_inscriber = ctx.accounts.payer.key();
    inscription_summary.inscription_count_total += 1;


    // inscription v1
    inscription.authority = authority;
    inscription.size = INITIAL_SIZE as u32;
    inscription.inscription_data = inscription_data.key();
    inscription.root = ctx.accounts.root.key();
    inscription.media_type = MediaType::None;
    inscription.encoding_type = EncodingType::None;
    inscription.validation_hash = input.validation_hash.clone();
    inscription.order = inscription_summary.inscription_count_total;

    // inscription v2
    inscription_v2.authority = authority;
    inscription_v2.size = INITIAL_SIZE as u32;
    inscription_v2.inscription_data = inscription_data.key();
    inscription_v2.root = ctx.accounts.root.key();
    inscription_v2.content_type = "".to_owned();
    inscription_v2.encoding = "".to_owned();
    inscription_v2.validation_hash = input.validation_hash.clone();
    inscription_v2.order = inscription_summary.inscription_count_total;
    
    
    let signer = ctx.accounts.signer.key();
    let root_key = inscription.root.key();


    // check signer - it must be either the mint itself
    // or a PDA signed by an authorised signer program

    // is the signer an authorised PDA?
    match input.signer_type {
        SignerType::Root => {
            if signer != inscription.root {
                return Err(ErrorCode::RootSignerMismatch.into());
            }
        }
        SignerType::LegacyMetadataSigner => {
            let expected_signer =
                Pubkey::find_program_address(&[root_key.as_ref()], &legacy_inscriber::id()).0;
            if expected_signer != signer {
                return Err(ErrorCode::LegacyMetadataSignerMismatch.into());
            }
        },
        SignerType::FairLaunchGhostRootSigner => {
            return Err(ErrorCode::RootSignerMismatch.into());
        }
    }

    // let page_to_update: &mut Box<Account<'_, InscriptionRankPage>>;
    // // if inscription_summary.inscription_count_total > inscription_input.current_rank_page * INSCRIPTIONS_PER_PAGE  {
    // if inscription_summary.inscription_count_total - 1
    //     <= (input.current_rank_page as u64 + 1) * INSCRIPTIONS_PER_PAGE
    // {
    //     page_to_update = inscriptions_ranks_current_page;
    // } else if inscription_summary.inscription_count_total - 1
    //     <= (input.current_rank_page as u64 + 2) * INSCRIPTIONS_PER_PAGE
    // {
    //     page_to_update = inscriptions_ranks_next_page;
    // } else {
    //     return Err(ErrorCode::BadInscriptionRankPage.into());
    // }

    // let page_rank_accountinfo = &mut page_to_update.to_account_info();

    // reallocate_rank_page(page_rank_accountinfo, payer, system_program, inscription_summary.inscription_count_total as usize)?;
    // add_inscription_to_rank_page(page_to_update, inscription)?;

    // for now, only fire events for inscription v1
    emit!(InscriptionEventCreate {
        id: inscription.key(),
        data: InscriptionEventData { 
            authority: inscription.authority, 
            root: inscription.root, 
            media_type: inscription.media_type.clone(),
            encoding_type: inscription.encoding_type.clone(),
            inscription_data: inscription.inscription_data,
            order: inscription.order,
            size: inscription.size,
            validation_hash: inscription.validation_hash.clone()
        }
    });

    Ok(())
}
