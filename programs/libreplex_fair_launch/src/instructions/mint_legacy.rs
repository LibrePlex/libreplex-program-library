use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use libreplex_inscriptions::InscriptionSummary;
// use libreplex_shared::sysvar_instructions_program;

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV2,
    cpi::accounts::MakeInscriptionImmutable,
    cpi::accounts::ResizeInscription,
    cpi::accounts::WriteToInscription,
    instructions::{SignerType, WriteToInscriptionInput},
};
use libreplex_shared::create_metadata_and_masteredition;
use mpl_token_metadata::types::Creator;

use crate::{
    errors::FairLaunchError, swap_to_fungible::sysvar_instructions_program, Deployment, Hashlist, MintAndOrder,
};

#[derive(Accounts)]
pub struct MintLegacyCtx<'info> {
    #[account(mut,
        seeds = ["deployment".as_ref(), deployment.ticker.as_ref()], bump)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut, 
        realloc = (8 + 4 + (deployment.number_of_tokens_issued + 1)*36 )as usize,
        realloc::payer = payer,
        realloc::zero = false,
        seeds = ["hashlist".as_bytes(), 
        deployment.key().as_ref()],
        bump,)]
    pub hashlist: Account<'info, Hashlist>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: It's a fair launch. Anybody can sign, anybody can receive the inscription
    #[account(mut)]
    pub inscriber: UncheckedAccount<'info>,

    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    // when we create an "op" operation, we mint the corresponding amount of
    // spl tokens into this token account
    #[account(
        mut,
        constraint = fungible_token_account_escrow.owner == deployment.key(),
        constraint = fungible_token_account_escrow.mint == fungible_mint.key()
    )]
    pub fungible_token_account_escrow: Account<'info, TokenAccount>,

    // legacy - TokenKeg
    // libre - Token22
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer, // will be a non
    )]
    pub non_fungible_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = inscriber,
    )]
    pub non_fungible_token_account: Account<'info, TokenAccount>,

    
    
    /// CHECK: passed in via CPI to mpl_token_metadata program
    pub non_fungible_metadata: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to mpl_token_metadata program
    pub non_fungible_masteredition: UncheckedAccount<'info>,


    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,

    /// CHECK: sent via CPI to libreplex_inscriptions_program
    #[account()]
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

}

pub fn mint_legacy(ctx: Context<MintLegacyCtx>) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;

    // to be discussed w/ everybody and feedback. Not strictly in line with BRC 20 thinking
    // but seems pointless to issue tokens if they can never be valid
    if deployment.number_of_tokens_issued >= deployment.max_number_of_tokens || deployment.minted_out {
        return Err(FairLaunchError::MintedOut.into());
    }

    let hashlist = &mut ctx.accounts.hashlist;

    let inscription_summary = &ctx.accounts.inscription_summary;

    let payer = &ctx.accounts.payer;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let non_fungible_metadata =  &ctx.accounts.non_fungible_metadata;
    let non_fungible_masteredition = &ctx.accounts.non_fungible_masteredition;
    

    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let inscription = &ctx.accounts.inscription;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let token_account_escrow = &ctx.accounts.fungible_token_account_escrow;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let metadata_program = &ctx.accounts.metadata_program;
   

    deployment.number_of_tokens_issued += 1;
    if deployment.number_of_tokens_issued == deployment.max_number_of_tokens {
        deployment.minted_out = true;
    }

    // mint X number of tokens into escrow token account

    // issue the NFT + inscription to the signer

    // STEP 1 - create inscription
    libreplex_inscriptions::cpi::create_inscription_v2(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            CreateInscriptionV2 {
                /* the inscription root is set to metaplex
                    inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),

                root: non_fungible_mint.to_account_info(),
                /// since root in this case can sign (we are creating a brand new mint),
                /// it will sign
                signer: non_fungible_mint.to_account_info(),
                inscription: inscription.to_account_info(),
                inscription2: inscription_v3.to_account_info(),

                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
            },
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInput {
            authority: Some(payer.key()), // this includes update auth / holder, hence
            current_rank_page: 0,
            signer_type: SignerType::Root,
            validation_hash: None,
        },
    )?;

    let data_bytes = deployment.mint_template.clone().into_bytes();

    libreplex_inscriptions::cpi::resize_inscription(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            ResizeInscription {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                authority: payer.to_account_info(),

                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                inscription2: Some(inscription_v3.to_account_info()),
            },
        ),
        libreplex_inscriptions::instructions::ResizeInscriptionInput {
            change: data_bytes.len() as i32 - 8,
            expected_start_size: 8,
            target_size: data_bytes.len() as u32,
        },
    )?;

    libreplex_inscriptions::cpi::write_to_inscription(
        CpiContext::new(
            inscriptions_program.to_account_info(),
            WriteToInscription {
                authority: payer.to_account_info(),
                payer: payer.to_account_info(),
                inscription: inscription.to_account_info(),
                inscription2: Some(inscription_v3.to_account_info()),
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

    libreplex_inscriptions::cpi::make_inscription_immutable(CpiContext::new(
        inscriptions_program.to_account_info(),
        MakeInscriptionImmutable {
            payer: payer.to_account_info(),
            authority: payer.to_account_info(),
            inscription_summary: inscription_summary.to_account_info(),
            inscription: inscription.to_account_info(),
            inscription2: Some(inscription_v3.to_account_info()),
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

    let deployment_seeds: &[&[u8]] = &[
        "deployment".as_bytes(),
        deployment.ticker.as_ref(),
        &[ctx.bumps.deployment],
    ];


    // mint fungible only
    // minting 
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: fungible_mint.to_account_info(),
                // always mint spl tokens to the program escrow
                to: token_account_escrow.to_account_info(),
                authority: deployment.to_account_info(),
            },
            &[deployment_seeds],
        ),
        deployment.limit_per_mint,
    )?;

    let deployment_seeds = &["deployment".as_bytes(), deployment.ticker.as_ref(), &[ctx.bumps.deployment]];

    // create non-fungible metadata for the "MINT" instruction
    create_metadata_and_masteredition(
        &payer.to_account_info(),
        &deployment.to_account_info(),
        &non_fungible_mint.to_account_info(),
        &non_fungible_metadata.to_account_info(),
        Some(non_fungible_masteredition),
        &token_program.to_account_info(),
        &metadata_program.to_account_info(),
        &system_program.to_account_info(),
        None,
        // rent.to_account_into(),
        deployment.ticker.clone(),
        deployment.ticker.clone(),
        deployment.offchain_url.clone(),
        0,
        Some(
            [Creator {
                address: deployment.key(),
                verified: true,
                share: 100,
            }]
            .to_vec(),
        ),
        None, // this is the supply of the editions. always 0
        Some(deployment_seeds),
        false,
    )?;



    // update hashlist

    hashlist.issues.push(MintAndOrder {
        mint: fungible_mint.key(),
        order: inscription_summary.inscription_count_total
    });

    Ok(())
}
