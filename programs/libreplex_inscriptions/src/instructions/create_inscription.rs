
use crate::{
    Inscription, InscriptionEvent, InscriptionEventType, InscriptionSummary,
};
use anchor_lang::prelude::*;



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

#[derive(Accounts)]
#[instruction(inscription_input: CreateInscriptionInput)]
pub struct CreateInscription<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub root: Signer<'info>,

    #[account(init_if_needed, seeds = [b"inscription_summary"],
        bump, payer = payer, space = InscriptionSummary::BASE_SIZE)]
    pub inscription_summary: Box<Account<'info, InscriptionSummary>>,

    /// CHECK: validated in logic
    #[account(zero)]
    pub inscription: Account<'info, Inscription>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateInscription>,
    inscription_input: CreateInscriptionInput,
) -> Result<()> {
    let inscription = &mut ctx.accounts.inscription;
    let inscription_summary = &mut ctx.accounts.inscription_summary;
    msg!("Writing authority");

    let authority = match inscription_input.authority {
        Some(x) => x.to_owned(),
        None => ctx.accounts.payer.key(),
    };

    let clock = Clock::get()?;

    inscription_summary.last_inscription_create_time = clock.unix_timestamp;
    inscription_summary.last_inscription = inscription.key();
    inscription_summary.last_inscriber = ctx.accounts.payer.key();

    // augment the total count but not the immutable count
    inscription_summary.inscription_count_total += 1;

    inscription.authority = authority;
    inscription.size = inscription_input.max_data_length;
    inscription.root = ctx.accounts.root.key();

    emit!(InscriptionEvent {
        id: inscription.key(),
        event_type: InscriptionEventType::Create
    });

    Ok(())
}
