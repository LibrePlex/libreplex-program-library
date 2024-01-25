

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022, token_interface::Mint
};
use solana_program::program::invoke_signed;
use spl_token_metadata_interface::{instruction::update_field, state::Field};




use crate::{Deployment, HashlistMarker,
};

#[derive(Accounts)]
pub struct UpdateSymbol2022Ctx<'info> {
    #[account(mut,
       

        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,


    #[account(
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        non_fungible_mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub non_fungible_mint: InterfaceAccount<'info, Mint>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn update_symbol2022(ctx: Context<UpdateSymbol2022Ctx>) -> Result<()> {
    // let MintToken2022Ctx { 
      
    //     ..
    // } = &ctx.accounts;


    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    
    // mutable borrows
    let deployment = &mut ctx.accounts.deployment;
    

    let deployment_seeds: &[&[u8]] =
    &["deployment".as_bytes(), deployment.ticker.as_ref(), &[ctx.bumps.deployment]];


    let initialise_metadata_ix = update_field(
        &spl_token_2022::ID,
        &non_fungible_mint.key(),
        &deployment.key(),
        Field::Symbol,
        deployment.ticker.clone(),
    );

    let account_infos = &[
        non_fungible_mint.to_account_info(),
        deployment.to_account_info(),
    ];
    invoke_signed(&initialise_metadata_ix, account_infos, &[deployment_seeds])?;
   
//     // Creates an `UpdateField` instruction
// pub fn update_field(
//     program_id: &Pubkey,
//     metadata: &Pubkey,
//     update_authority: &Pubkey,
//     field: Field,
//     value: String,
// ) -> Instruction {
//     let data = TokenMetadataInstruction::UpdateField(UpdateField { field, value });
//     Instruction {
//         program_id: *program_id,
//         accounts: vec![
//             AccountMeta::new(*metadata, false),
//             AccountMeta::new_readonly(*update_authority, true),
//         ],
//         data: data.pack(),
//     }
// }

    Ok(())
}
