use crate::errors::ErrorCode;

use crate::{
    Inscription, InscriptionEvent, InscriptionEventType, InscriptionSummary, InscriptionData, InscriptionRankPage,
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionInput {
    pub max_data_length: u32,
    pub authority: Option<Pubkey>,
    // each rank page holds a maximum of 320000 inscription ids.
    // when this runs out, we move onto the next page
    pub current_rank_page: u32,
}

impl CreateInscriptionInput {
    pub fn get_size(&self) -> u32 {
        self.max_data_length
            + 1
            + match self.authority {
                Some(_) => 32,
                None => 0,
            }
    }
}

const INSCRIPTIONS_PER_PAGE: u64 = 300000;

#[derive(Accounts)]
#[instruction(inscription_input: CreateInscriptionInput)]
pub struct CreateInscription<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub root: Signer<'info>,


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
    #[account(zero)]
    pub inscription_data: Account<'info, InscriptionData>,

    /// CHECK: validated in logic
    #[account(init,
        space = Inscription::SIZE,
        seeds=[
            "inscription".as_bytes(),
            root.key().as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,


    
    
}

pub fn handler(
    ctx: Context<CreateInscription>,
    input: CreateInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;
    let inscription_summary = &mut ctx.accounts.inscription_summary;
    msg!("Writing authority");

    let authority = match input.authority {
        Some(x) => x.to_owned(),
        None => ctx.accounts.payer.key(),
    };
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let inscription_data = &ctx.accounts.inscription_data;

    let inscriptions_ranks_current_page = &mut ctx.accounts.inscription_ranks_current_page;
    let inscriptions_ranks_next_page = &mut ctx.accounts.inscription_ranks_next_page;
    
    let clock = Clock::get()?;

    inscription_summary.last_inscription_create_time = clock.unix_timestamp;
    inscription_summary.last_inscription = inscription.key();
    inscription_summary.last_inscriber = ctx.accounts.payer.key();

    // augment the total count but not the immutable count
    inscription_summary.inscription_count_total += 1;

    inscription.authority = authority;
    inscription.size = input.max_data_length;
    inscription.inscription_data = inscription_data.key();
    inscription.root = ctx.accounts.root.key();

    inscription.rank = inscription_summary.inscription_count_total;
    println!(
        "Inscription count {}",
        inscription_summary.inscription_count_total
    );

    let page_to_update: &mut Box<Account<'_, InscriptionRankPage>>;
    // if inscription_summary.inscription_count_total > inscription_input.current_rank_page * INSCRIPTIONS_PER_PAGE  {
    if inscription_summary.inscription_count_total - 1
        <= (input.current_rank_page as u64 + 1) * INSCRIPTIONS_PER_PAGE
    {
        page_to_update = inscriptions_ranks_current_page;
       
    } else if inscription_summary.inscription_count_total - 1
        <= (input.current_rank_page as u64 + 2) * INSCRIPTIONS_PER_PAGE
    {
        page_to_update = inscriptions_ranks_next_page;
       
    } else {
        return Err(ErrorCode::BadInscriptionRankPage.into());
    }

    let page_rank_accountinfo = &mut page_to_update.to_account_info();
       
   
    reallocate_rank_page(page_rank_accountinfo, payer, system_program)?;
    add_inscription_to_rank_page(page_to_update, inscription)?;


    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}


fn reallocate_rank_page<'info>(
    inscriptions_ranks_page: &mut AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<()> {

   
    let new_size = inscriptions_ranks_page.data.borrow().len() + 32;
    println!("new size {}", new_size);
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    let lamports_diff = new_minimum_balance.saturating_sub(inscriptions_ranks_page.lamports());
    println!("lamports_diff {}", lamports_diff);
   
    invoke(
        &system_instruction::transfer(&payer.key(), 
        inscriptions_ranks_page.key, 
        lamports_diff*1000),
        &[
            payer.clone(),
            inscriptions_ranks_page.clone(),
            system_program.clone(),
        ],
    )?;
    inscriptions_ranks_page.realloc(new_size, false)?;
    Ok(())
}

fn add_inscription_to_rank_page(
    inscriptions_ranks_page: &mut Box<Account<'_, InscriptionRankPage>>,
    inscription: &mut Account<'_, Inscription>,
) -> Result<()> {

    println!("Adding inscription {}", inscription.key());
    let inscriptions_ranks_current_page_account_info = inscriptions_ranks_page.to_account_info();
    let current_data = inscriptions_ranks_current_page_account_info
        .data
        .borrow_mut();

    InscriptionRankPage::add_inscription(inscriptions_ranks_page, current_data, inscription.key())?;
    Ok(())
}