

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022, token_interface::Mint
};
use solana_program::program::invoke_signed;
use spl_token_metadata_interface::{instruction::update_field, state::Field};




use crate::Deployment;

#[derive(Accounts)]
pub struct UpdateSplMetadata2022Ctx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,


    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut,
        constraint = deployment.creator == signer.key())]
    pub signer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut, constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn update_spl_metadata2022(ctx: Context<UpdateSplMetadata2022Ctx>, new_uri: String) -> Result<()> {
    // let MintToken2022Ctx { 
      
    //     ..
    // } = &ctx.accounts;


    let fungible_mint = &ctx.accounts.fungible_mint;
    
    // mutable borrows
    let deployment = &mut ctx.accounts.deployment;
    

    let deployment_seeds: &[&[u8]] =
    &["deployment".as_bytes(), deployment.ticker.as_ref(), &[ctx.bumps.deployment]];


    let update_metadata_ix = update_field(
        &spl_token_2022::ID,
        &fungible_mint.key(),
        &deployment.key(),
        Field::Symbol,
        deployment.ticker.clone(),
    );


    let account_infos = &[
        fungible_mint.to_account_info(),
        deployment.to_account_info(),
    ];
    invoke_signed(&update_metadata_ix, account_infos, &[deployment_seeds])?;

    let update_metadata_ix: solana_program::instruction::Instruction = update_field(
        &spl_token_2022::ID,
        &fungible_mint.key(),
        &deployment.key(),
        Field::Uri,
        new_uri
    );

    let account_infos = &[
        fungible_mint.to_account_info(),
        deployment.to_account_info(),
    ];
    invoke_signed(&update_metadata_ix, account_infos, &[deployment_seeds])?;
   
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
