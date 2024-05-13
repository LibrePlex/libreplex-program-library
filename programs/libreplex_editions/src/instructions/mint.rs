
use dyn_fmt::AsStrFormatExt;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken, token_2022
};


use libreplex_shared::{create_token_2022_and_metadata, operations::mint_non_fungible_2022_logic, MintAccounts2022, SharedError, TokenMemberInput};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_metadata_interface::state::TokenMetadata;

use crate::{add_to_hashlist, errors::EditionsError, group_extension_program, EditionsDeployment, HashlistMarker};




#[derive(Accounts)]
pub struct MintCtx<'info> {

    #[account(mut,
        seeds = ["editions_deployment".as_ref(), editions_deployment.symbol.as_ref()], bump)]
    pub editions_deployment: Account<'info, EditionsDeployment>,

    
    /// CHECK: Checked in PDA. Not deserialized because it can be rather big
    #[account(mut, 
        seeds = ["hashlist".as_bytes(), 
        editions_deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = HashlistMarker::SIZE,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        editions_deployment.key().as_ref(),
        mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // when deployment.require_creator_cosign is true, this must be equal to the creator
    // of the deployment otherwise, can be any signer account
    #[account(mut,
        constraint = editions_deployment.cosigner_program_id == system_program::ID || signer.key() == editions_deployment.creator)]
    pub signer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub minter: UncheckedAccount<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    pub member: Signer<'info>,

    /// CHECK: Checked in constraint
    #[account(mut,
        constraint = editions_deployment.group == group.key())]
    pub group: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    



    /* BOILERPLATE PROGRAM ACCOUNTS */
    /// CHECK: Checked in constraint
    #[account(
        constraint = token_program.key() == token_2022::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,


    /// CHECK: address checked
    #[account(address = group_extension_program::ID)]
    pub group_extension_program: AccountInfo<'info>,


    #[account()]
    pub system_program: Program<'info, System>,

}

pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
    // let MintToken2022Ctx { 
      
    //     ..
    // } = &ctx.accounts;

    let payer = &ctx.accounts.payer; 
    let signer = &ctx.accounts.signer;
    let minter= &ctx.accounts.minter;
    let minter_token_account = &ctx.accounts.token_account;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let member = &ctx.accounts.member;
    
    let group = &ctx.accounts.group;
    let group_extension_program = &ctx.accounts.group_extension_program;
    // mutable borrows
    let editions_deployment = &mut ctx.accounts.editions_deployment;
    let hashlist = &mut ctx.accounts.hashlist;



    if !editions_deployment.cosigner_program_id.eq(&system_program::ID) && !signer.key().eq(&editions_deployment.creator.key()) {
        return Err(SharedError::InvalidCreatorCosigner.into());
    }
  
    // max_number_of_tokens == 0 means unlimited mints 
    if editions_deployment.max_number_of_tokens > 0 && editions_deployment.number_of_tokens_issued >= editions_deployment.max_number_of_tokens {
        return Err(EditionsError::MintedOut.into());
    }

     
    let update_authority =
        OptionalNonZeroPubkey::try_from(Some(editions_deployment.key())).expect("Bad update auth");

        let deployment_seeds: &[&[u8]] = &[
            "editions_deployment".as_bytes(),
            editions_deployment.symbol.as_ref(),
            &[ctx.bumps.editions_deployment],
        ];



    let name = match editions_deployment.name_is_template {
        true => editions_deployment.name.format(&[editions_deployment.number_of_tokens_issued+1]),
        false => editions_deployment.name.clone()
    };

    let url = match editions_deployment.url_is_template {
        true => editions_deployment.offchain_url.format(&[editions_deployment.number_of_tokens_issued+1]),
        false => editions_deployment.offchain_url.clone()
    };
    // msg!("Create token 2022 w/ metadata");
    create_token_2022_and_metadata(
        MintAccounts2022 {
            authority: editions_deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: minter.to_account_info(),
            nft_mint: mint.to_account_info(),
            spl_token_program: token_program.to_account_info(),
        },
        0,
        Some(TokenMetadata {
            name,
            symbol: editions_deployment.symbol.clone(),
            uri: url,
            update_authority,
            mint: mint.key(),
            additional_metadata: vec![],
        }),
        None,
        Some(TokenMemberInput {
            member: member.to_account_info(),
            group: group.to_account_info(),
        }),
        Some(deployment_seeds),
        None,
        Some(group_extension_program.key())
    )?;

     // msg!("Minting 2022");
     mint_non_fungible_2022_logic(
        &mint.to_account_info(),
        minter_token_account,
        associated_token_program,
        payer,
        &minter.to_account_info(),
        system_program,
        token_program,
        &editions_deployment.to_account_info(),
        deployment_seeds,
    )?;
    
    editions_deployment.number_of_tokens_issued += 1;
    add_to_hashlist(
        editions_deployment.number_of_tokens_issued as u32,
        hashlist,
        payer,
        system_program,
        &mint.key(),
        editions_deployment.number_of_tokens_issued,
    )?;
    
    Ok(())
}
