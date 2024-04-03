use anchor_lang::prelude::*;
use anchor_spl::{token::{mint_to, Mint, MintTo, Token, TokenAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token::AssociatedToken};
use libreplex_inscriptions::InscriptionV3;
use libreplex_shared::SharedError;



use crate::{add_to_hashlist, Deployment, HashlistMarker, MigrationCounter, MigrationMarker};

#[event]
pub struct HashlistEvent {
    pub mint: Pubkey,
    pub deployment: Pubkey
}

pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}   

/*

    Initialise sets the main template parameters of the deployment:
    1) ticker
    2) deployment template
    3) mint template
    4) decimals
    5) limit per mint
    6) max number of tokens

    It does not create any inscriptions / mints as these are handled by the deploy endpoints.
    This method is metadata agnostic.

*/

#[derive(Accounts)]
pub struct MigrateToHashlistCtx<'info>  {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Box<Account<'info, Deployment>>,


    // will prevent a single mint from being migrated multiple times
    #[account(
        init,
        space=8,
        payer=payer,
        seeds=["migration_marker".as_ref(), mint.key().as_ref()], bump)]
    pub migration_marker: Box<Account<'info, MigrationMarker>>,

    #[account(
        init_if_needed,
        space=8+32+8,
        payer=payer,
        seeds=["migration_counter".as_ref(), deployment.key().as_ref()], bump)]
    pub migration_counter: Box<Account<'info, MigrationCounter>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // needed because some of the old hashlists are still stored off chain. 
    // this API signer will be removed once the migration is complete.
    #[account(mut,
        constraint = signer.key().to_string() == *"4aAifU9ck88koMhSK6fnUSQHMzpyuLzGa6q7nfvqA6vx".to_owned())]
    pub signer: Signer<'info>,

    /// CHECK: checked via PDA
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = 8,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        mint.key().as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account()]
    pub inscription_v3: Box<Account<'info, InscriptionV3>>,


    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: Box<Account<'info, Mint>>,


     /// CHECK: Id checked in constraint
    #[account(mut,
        token::authority = deployment.key(),
        token::mint = fungible_mint.key()
    )]
    pub fungible_token_account_escrow: Box<Account<'info, TokenAccount>>,


    #[account()]
    pub system_program: Program<'info, System>,

     /* BOILERPLATE PROGRAM ACCOUNTS */
     #[account()]
     pub token_program: Program<'info, Token>,

     #[account()]
     pub associated_token_program: Program<'info, AssociatedToken>,


}

pub fn migrate_to_hashlist(ctx: Context<MigrateToHashlistCtx>) -> Result<()> {

    let deployment = &mut ctx.accounts.deployment;
    let hashlist = &mut ctx.accounts.hashlist;
    let mint: &mut Account<'_, Mint> = &mut ctx.accounts.mint;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let token_program = &ctx.accounts.token_program;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let fungible_token_account_escrow = &ctx.accounts.fungible_token_account_escrow;
    // let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let migration_counter = &mut ctx.accounts.migration_counter;

    migration_counter.migration_count += 1;
    migration_counter.deployment = deployment.key();
    


    let expected_hashlist_key = Pubkey::find_program_address(&["hashlist".as_bytes(), 
    deployment.key().as_ref()], &crate::id());

    if hashlist.key != &expected_hashlist_key.0 {
        panic!("Invalid hashlist key")
    }


    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];

    if !deployment.migrated_from_legacy {
        panic!("Cannot migrate to this deployment")
    }


    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &deployment.key(), &fungible_mint.key());

    if expected_token_account != fungible_token_account_escrow.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }   


    let current_mint_amount = fungible_mint.supply;

    let final_mint_amount = deployment.get_max_fungible_mint_amount_per_deployment(
        &None
    );


    // if we're not minted out on the fungible, max out the mint 
    // and remove freeze + mint authorities
    if current_mint_amount < final_mint_amount {
        msg!("current_mint_amount {}",current_mint_amount);
        msg!("final_mint_amount {}",final_mint_amount);

        // it is possible that the current mint amount is smaller than the final mint amount
        // this can happen when a part of the total supply has been burned after scribing out
        // so we put this check here. In any case there's no point trying to mint anything 
        // if there is no mint authority.
        if fungible_mint.mint_authority.is_some()  {
            mint_to(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    MintTo {
                        mint: fungible_mint.to_account_info(),
                        // always mint spl tokens to the program escrow
                        to: fungible_token_account_escrow.to_account_info(),
                        authority: deployment.to_account_info(),
                    },
                    &[deployment_seeds],
                ),

                final_mint_amount - current_mint_amount  
            )?;
        }

        if fungible_mint.freeze_authority.is_some() {
            // ok we are at max mint
            set_authority(CpiContext::new_with_signer(
                token_program.to_account_info(),
                SetAuthority {
                    current_authority: deployment.to_account_info(),
                    account_or_mint: fungible_mint.to_account_info(),
                },
                &[deployment_seeds]
            ),
            AuthorityType::FreezeAccount,
            None
            )?;
        }

        if fungible_mint.mint_authority.is_some() {
            // ok we are at max mint
            set_authority(CpiContext::new_with_signer(
                token_program.to_account_info(),
                SetAuthority {
                    current_authority: deployment.to_account_info(),
                    account_or_mint: fungible_mint.to_account_info(),
                },
                &[deployment_seeds]
            ),
            AuthorityType::MintTokens,
            None
            )?;
        }

        
        
    } 
    // we cannot use the number of tokens issued because for migrations
    // number of tokens issues is usually equal to the max supply. So we
    // need to work off the migration counter.
   
    add_to_hashlist(
        migration_counter.migration_count as u32, // already pre-incremented
        hashlist, 
        payer, 
        system_program, 
        &mint.key(), 
        &deployment.key(),
        inscription_v3.order
    )?;

    


    Ok(())
}