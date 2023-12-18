
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
    cpi::accounts::MakeInscriptionImmutableV3,
    cpi::accounts::ResizeInscriptionV3,
    cpi::accounts::WriteToInscriptionV3,
    instructions::{SignerType, WriteToInscriptionInput},
};
use libreplex_shared::{SharedError, sysvar_instructions_program};

use crate::{
    errors::FairLaunchError, Deployment, HashlistMarker, MintEvent, add_to_hashlist,
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

    /// CHECK: Checked by address
    #[account(
        seeds = 
        [get_asset_id(merkle_tree.key, tree_authority.num_minted).as_ref()], 
        bump)]
    pub ghost_root_signer: UncheckedAccount<'info>,

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

pub fn mint_c_legacy(ctx: Context<MintLegacyCtx>) -> Result<()> {
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


    let tree_authority = &ctx.accounts.tree_authority;
    let merkle_tree = &ctx.accounts.merkle_tree;
    let asset_id = get_asset_id(merkle_tree.key, tree_authority.num_minted);
    
    let ghost_root_signer = &ctx.accounts.ghost_root_signer;
    let ghost_root_seeds: &[&[u8]] = &[asset_id.as_ref(), &[ctx.bumps.ghost_root_signer]];

    let mint_compressed_accounts = bubblegum_proxy::cpi::accounts::MintV1 {
        compression_program: ctx.accounts.account_compression_program.to_account_info(),
        tree_authority: ctx.accounts.tree_authority.to_account_info(),
        leaf_owner: payer.to_account_info(),
        leaf_delegate: payer.to_account_info(),
        merkle_tree: merkle_tree.to_account_info(),
        payer: payer.to_account_info(),
        tree_delegate: deployment.to_account_info(),
        log_wrapper: ctx.accounts.noop_program.to_account_info(),
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


    libreplex_inscriptions::cpi::create_ghost_root_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(), 
            libreplex_inscriptions::cpi::accounts::CreateGhostRootInscription {
                /* the inscription root is set to metaplex
                inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),
                signer: ghost_root_signer.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            }, 
            &[ghost_root_seeds]), 
        libreplex_inscriptions::instructions::CreateGhostRootInscriptionInput {
            authority: Some(payer.key()), // this includes update auth / holder, hence
            signer_type: SignerType::FairLaunchGhostRootSigner,
            validation_hash: None,
            root: asset_id,
        })?;
    
    let data_bytes = deployment.mint_template.clone().into_bytes();


    libreplex_inscriptions::cpi::resize_inscription_v3(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            ResizeInscriptionV3 {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                authority: payer.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
            },
        ),
        libreplex_inscriptions::instructions::ResizeInscriptionInput {
            change: data_bytes.len() as i32 - 8,
            expected_start_size: 8,
            target_size: data_bytes.len() as u32,
        },
    )?;

    libreplex_inscriptions::cpi::write_to_inscription_v3(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            WriteToInscriptionV3 {
                authority: payer.to_account_info(),
                payer: payer.to_account_info(),
                inscription_v3: inscription_v3.to_account_info(),
                system_program: system_program.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
        ),
        WriteToInscriptionInput {
            data: data_bytes,
            start_pos: 0,
            media_type: Some("text/plain".to_owned()),
            encoding_type: Some("ascii".to_owned()),
        },
    )?;

    libreplex_inscriptions::cpi::make_inscription_immutable_v3(CpiContext::new(
        inscriptions_program.to_account_info(),
        MakeInscriptionImmutableV3 {
            payer: payer.to_account_info(),
            authority: payer.to_account_info(),
            inscription_summary: inscription_summary.to_account_info(),
            inscription_v3: inscription_v3.to_account_info(),
            system_program: system_program.to_account_info(),
        },
    ))?;
    
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
            &asset_id, 
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
        mint: asset_id,
        ticker: deployment.ticker.clone(),
        tokens_minted: deployment.number_of_tokens_issued,
        max_number_of_tokens: deployment.max_number_of_tokens,
    });

    Ok(())
}