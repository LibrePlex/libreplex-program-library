use anchor_lang::prelude::*;

use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;

use crate::{add_to_hashlist, Deployment, MintEvent};

pub fn update_deployment_and_hashlist<'a>(
    deployment: &mut Account<'a, Deployment>,
    hashlist: &mut UncheckedAccount<'a>,
    payer: &Signer<'a>,
    system_program: &Program<'a, System>,
    root_key: Pubkey,
    inscription_summary: &UncheckedAccount<'a>,
) -> Result<()> {

  
    if deployment.number_of_tokens_issued <= 262144 {
        msg!("updating hashlist");
        let mut order_number = 0;
        if deployment.use_inscriptions {
            msg!("using inscriptions");
          
            if inscription_summary.to_account_info().data_is_empty() {
                let mut data: &[u8] = &inscription_summary.try_borrow_data()?[..];
                let inscription_summary_obj = InscriptionSummary::deserialize(&mut data)?;
                order_number = inscription_summary_obj.inscription_count_total;
            }

        } 

        add_to_hashlist(
            deployment.number_of_tokens_issued as u32,
            hashlist,
            payer,
            system_program,
            &root_key,
            &deployment.key(),
            order_number,
        )?;
    };

    emit!(MintEvent {
        mint: root_key,
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });
    Ok(())
}
