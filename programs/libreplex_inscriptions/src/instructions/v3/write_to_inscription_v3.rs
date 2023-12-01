use crate::{errors::ErrorCode, instructions::WriteToInscriptionInput, InscriptionV3};
use anchor_lang::{prelude::*, system_program};



#[event]
pub struct InscriptionWriteEvent {
    pub id: Pubkey,
}

#[derive(Accounts)]
#[instruction(inscription_input: WriteToInscriptionInput)]
pub struct WriteToInscriptionV3<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // required for the realloc if inscription size changes due
    // to media_type / encoding_type
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Authority checked in logic
    #[account(mut,
        constraint = inscription_v3.authority == authority.key(),
        constraint = inscription_v3.inscription_data == inscription_data.key())]
    pub inscription_v3: Account<'info, InscriptionV3>,

    /// CHECK: we never want anchor to handle this. It's just a data blob
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WriteToInscriptionV3>,
    inscription_input: WriteToInscriptionInput,
) -> Result<()> {
    let inscription_v3 = &mut ctx.accounts.inscription_v3;

    let payer = &mut ctx.accounts.payer;

    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;

    let inscription_data = &ctx.accounts.inscription_data;

    let inscription_data_account_info = inscription_data.to_account_info();
    // check that the authority matches

    if inscription_v3.authority != authority.key() {
        return Err(ErrorCode::BadAuthority.into());
    }

    if inscription_v3.inscription_data != inscription_data.key() {
        return Err(ErrorCode::IncorrectInscriptionDataAccount.into());
    }

    let new_length = InscriptionV3::get_new_size(inscription_v3, &inscription_input);
    msg!("new length {}", new_length);
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_length);
    let lamports_diff = new_minimum_balance.saturating_sub(inscription_v3.to_account_info().lamports());

    if lamports_diff > 0 {
        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: payer.to_account_info(),
                    to: inscription_v3.to_account_info(),
                },
            ),
            lamports_diff,
        )?;
    }

    inscription_v3.to_account_info().realloc(new_length, true)?;

    if let Some(x) = inscription_input.media_type {
        inscription_v3.content_type = x.clone();
    }

    if let Some(x) = inscription_input.encoding_type {
        inscription_v3.encoding = x.clone();
    }
    if !inscription_input.data.is_empty() {
        inscription_v3.write_data(
            inscription_data_account_info.data.borrow_mut(),
            &inscription_input.data,
            inscription_input.start_pos,
        )?;
    }

    emit!(InscriptionWriteEvent {
        id: inscription_v3.key(),
    });

    Ok(())
}
