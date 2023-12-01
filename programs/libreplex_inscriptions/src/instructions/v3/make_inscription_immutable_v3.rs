use crate::{InscriptionV3, InscriptionV3EventData};
use crate::InscriptionSummary;
use anchor_lang::prelude::*;

use anchor_lang::system_program;


#[event]
pub struct InscriptionV3EventUpdate {
    pub id: Pubkey,
    pub data: InscriptionV3EventData,
}

// limited by the max size of a PDA account, i.e. 10K (each inscription pubkey is 32 bytes)

#[derive(Accounts)]
pub struct MakeInscriptionImmutableV3<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub authority: Signer<'info>,

    #[account(init_if_needed, seeds = [b"inscription_summary"],
        bump, payer = payer, space = InscriptionSummary::BASE_SIZE)]
    pub inscription_summary: Box<Account<'info, InscriptionSummary>>,

    /// CHECK: validated in logic
    #[account(mut,
        constraint = inscription_v3.authority == authority.key())]
    pub inscription_v3: Account<'info, InscriptionV3>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MakeInscriptionImmutableV3>) -> Result<()> {
    let inscription_v3 = &mut ctx.accounts.inscription_v3;
    let inscription_summary = &mut ctx.accounts.inscription_summary;
    // let inscription_account_info = inscription.to_account_info();

    // we set the auth to the system program. This prevents any further changes
    inscription_v3.authority = system_program::ID;

    inscription_summary.inscription_count_immutables += 1;
    let clock = Clock::get()?;

    inscription_summary.last_inscription_create_time = clock.unix_timestamp;
    inscription_summary.last_inscription = inscription_v3.key();
    inscription_summary.last_inscriber = ctx.accounts.payer.key();

    emit!(InscriptionV3EventUpdate {
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
        },
    });

    Ok(())
}
