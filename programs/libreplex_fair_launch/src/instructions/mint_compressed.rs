
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, set_authority, SetAuthority, spl_token::instruction::AuthorityType},
};
use bubblegum_proxy::{TreeConfig, MetadataArgs};
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;
use mpl_bubblegum::utils::get_asset_id;

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV3,
    cpi::accounts::MakeInscriptionImmutableV3,
    cpi::accounts::ResizeInscriptionV3,
    cpi::accounts::WriteToInscriptionV3,
    instructions::{SignerType, WriteToInscriptionInput},
};
use libreplex_shared::{SharedError, create_mint_metadata_and_masteredition::create_mint_with_metadata_and_masteredition, MintAccounts, sysvar_instructions_program};
use mpl_token_metadata::types::{Creator, TokenStandard};
use solana_program::{program::invoke, system_instruction};

use crate::{
    errors::FairLaunchError, Deployment, HashlistMarker, MintEvent, HashlistEvent,
};

#[derive(Accounts)]
pub struct MintLegacyCtx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    
    #[account(mut, 
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(init, 
        space = 8,
        payer = payer,
        seeds = ["hashlist_marker".as_bytes(), 
        deployment.key().as_ref(),
        get_asset_id(merkle_tree.key, tree_authority.num_minted).as_ref()],
        bump,)]
    pub hashlist_marker: Account<'info, HashlistMarker>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub inscriber: UncheckedAccount<'info>,

    #[account(mut,
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: Account<'info, Mint>,

    /// CHECK: Checked in logic, created as necessary
    #[account(
        mut,
    )]
    pub fungible_token_account_escrow: UncheckedAccount<'info>,

    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: sent via CPI to libreplex_inscriptions_program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /* BOILERPLATE PROGRAM ACCOUNTS */
    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,

    
    /// CHECK: address checked
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

    /// CHECK: checked in cpi
    account_compression_program: AccountInfo<'info>,

    /// CHECK: checked in cpi
    noop_program: AccountInfo<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(mut)]
    pub tree_authority: Account<'info, TreeConfig>,

    /// CHECK: checked by address
    #[account(address = mpl_bubblegum::id())]
    bubblegum_program: AccountInfo<'info>,
}

pub fn mint_legacy(ctx: Context<MintLegacyCtx>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;

    // to be discussed w/ everybody and feedback. Not strictly in line with BRC 20 thinking
    // but seems pointless to issue tokens if they can never be valid
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens || deployment.minted_out {
        return Err(FairLaunchError::MintedOut.into());
    }

    if deployment.migrated_from_legacy {
        return Err(FairLaunchError::LegacyMigrationsAreMintedOut.into());
    }
    

    let hashlist = &mut ctx.accounts.hashlist;

    let inscription_summary = &ctx.accounts.inscription_summary;

    let payer = &ctx.accounts.payer;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let inscriptions_program = &ctx.accounts.inscriptions_program;

    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let fungible_token_account_escrow = &ctx.accounts.fungible_token_account_escrow;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let metadata_program = &ctx.accounts.metadata_program;
    let inscriber = &ctx.accounts.inscriber;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let sysvar_instructions_program = &ctx.accounts.sysvar_instructions;
    

    deployment.number_of_tokens_issued += 1;
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens {
        deployment.minted_out = true;
    }

    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];



    let mint_compressed_accounts = bubblegum_proxy::cpi::accounts::MintV1 {
        compression_program: ctx.accounts.account_compression_program,
        tree_authority: ctx.accounts.tree_authority.to_account_info(),
        leaf_owner: payer.to_account_info(),
        leaf_delegate: payer.to_account_info(),
        merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        payer: payer.to_account_info(),
        tree_delegate: deployment.to_account_info(),
        log_wrapper: ctx.accounts.noop_program,
        system_program: system_program.to_account_info(),
    };

    let metadata_args = MetadataArgs {
        name: deployment.ticker.clone(),
        symbol: "".to_string(),
        uri: deployment.offchain_url.clone(),
        seller_fee_basis_points: 0,
        primary_sale_happened: true,
        is_mutable: false,
        edition_nonce: None,
        token_standard: Some(bubblegum_proxy::TokenStandard::NonFungible),
        collection: None,
        uses: None,
        token_program_version: bubblegum_proxy::TokenProgramVersion::Original,
        creators: vec![bubblegum_proxy::Creator { 
            address: deployment.key(), 
            verified: true, 
            share: 100
        }],
    };

    bubblegum_proxy::cpi::mint_v1(
        CpiContext::new_with_signer(ctx.accounts.bubblegum_program.to_account_info(), 
        mint_compressed_accounts, &[deployment_seeds]), 
        metadata_args)?;

    

    
    /*
        Step 2: this the solana way where we meet brc 20 type thinking:

        As we create a 'mint' op inscription, we also mint a corresponding amount of
        spl tokens into an escrow account held by a PDA.

        This ensures that any SPL-20 account is always convertible into a corresponding
        amount of traditional SPL token AND vice versa.
    */


    let expected_token_account = anchor_spl::associated_token::get_associated_token_address(
        &deployment.key(), &fungible_mint.key());

    if expected_token_account != fungible_token_account_escrow.key() {
        return Err(SharedError::InvalidTokenAccount.into());
    }   

    if fungible_token_account_escrow.to_account_info().data_is_empty() {

        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: fungible_token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
                mint: fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    // mint fungible only
    // minting 
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
        deployment.get_fungible_mint_amount()   )?;

    // if we're at max tokens, remove freeze auth and mint auth

    if deployment.number_of_tokens_issued == deployment.max_number_of_tokens {
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

    if deployment.number_of_tokens_issued <= 262144 {
        add_to_hashlist(deployment.number_of_tokens_issued as u32, hashlist, 
            payer, 
            system_program, 
            &non_fungible_mint.key(), 
            &deployment.key(),
            inscription_summary.inscription_count_total
        )?;
    }

    // sets the max number of hashlist items to a nice round number
    // this is to prevent insanely large hashlists from blowing up the 
    // solana account size
    // for very large hashlists, they can also be queried by gPA to
    // first creator id or indexing hashlist_marker accounts.

    // this does NOT stop minting.

    emit!(MintEvent{
        mint: non_fungible_mint.key(),
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });

    Ok(())
}

pub fn add_to_hashlist<'a>(
    new_number_of_mints: u32, 
    hashlist: &mut UncheckedAccount<'a>, 
    payer: &Signer<'a>, 
    system_program: &Program<'a, System>, 
    mint: &Pubkey, 
    deployment: &Pubkey,
    order_number: u64) -> Result<()> {
    let new_size = 8 + 32 + 4 + (new_number_of_mints) * (32 + 8);
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size as usize);
    let lamports_diff = new_minimum_balance.saturating_sub(hashlist.lamports());
    if lamports_diff > 0 {
        invoke(
            &system_instruction::transfer(&payer.key(), hashlist.key, lamports_diff),
            &[
                payer.to_account_info(),
                hashlist.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;
    }
    hashlist.realloc(new_size as usize, false)?;
    let hashlist_account_info = hashlist.to_account_info();
   
    let mut hashlist_data = hashlist_account_info.data.borrow_mut();

    hashlist_data[40..44].copy_from_slice(&new_number_of_mints.to_le_bytes());
    let mint_start_pos:usize = (44+(new_number_of_mints-1)*40) as usize;
    hashlist_data[
        mint_start_pos..(mint_start_pos+32)
        ].copy_from_slice(mint.key().as_ref());
    hashlist_data[
        mint_start_pos + 32..mint_start_pos + 40
        ].copy_from_slice(&order_number.to_le_bytes());
  
    emit!(HashlistEvent {
        mint: mint.key(),
        deployment: deployment.key()
    });
    

    Ok(())

}
