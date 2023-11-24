use anchor_lang::prelude::*;
use anchor_spl::{token::{mint_to, Mint, TokenAccount, Token, MintTo}, associated_token::AssociatedToken};
use libreplex_inscriptions::{InscriptionSummary, Inscription, InscriptionV3};
use libreplex_shared::sysvar_instructions_program;

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV2,
    cpi::accounts::MakeInscriptionImmutable,
    cpi::accounts::ResizeInscription,
    cpi::accounts::WriteToInscription,
    instructions::{SignerType, WriteToInscriptionInput}
};




use crate::{TokenDeployment, errors::Spl20Error};

#[derive(Accounts)]
pub struct MintCtx<'info> {
    #[account(mut)]
    pub token_deployment: Account<'info, TokenDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub inscriber: UncheckedAccount<'info>,

    #[account(mut)]
    pub fungible_mint: Account<'info, Mint>,

    // legacy - TokenKeg
    // libre - Token22
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer, // will be a non
    )]
    pub non_fungible_mint: Account<'info, Mint>,

    // when we create an "op" operation, we mint the corresponding amount of
    // spl tokens into this token account
    #[account(
        mut,
        constraint = token_account_escrow.owner == escrow.key(),
        constraint = token_account_escrow.mint == fungible_mint.key()
    )]
    pub token_account_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = non_fungible_mint,
        token::authority = inscriber,
    )]
    pub inscriber_token_account: Account<'info, TokenAccount>,

    #[account(mut,
        seeds = ["escrow".as_ref(), token_deployment.key().as_ref()], bump)]
    pub escrow: UncheckedAccount<'info>,

    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    // CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: Account<'info, Inscription>,

    #[account(mut)]
    pub inscription_v3: Account<'info, InscriptionV3>,

    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// Check: sent via CPI
    #[account()]
    pub inscription_data: UncheckedAccount<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Passed in via CPI
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>,
}

pub fn mint(ctx: Context<MintCtx>) -> Result<()> {
    let token_deployment = &mut ctx.accounts.token_deployment;

    // to be discussed w/ everybody and feedback. Not strictly in line with BRC 20 thinking
    // but seems pointless to issue tokens if they can never be valid
    if token_deployment.number_of_tokens_issued >= token_deployment.max_number_of_tokens {
        return Err(Spl20Error::MaxNumberOfTokenExceeded.into());
    }

    let inscription_summary = &ctx.accounts.inscription_summary;

    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let mint = &ctx.accounts.non_fungible_mint;
    let inscription = &ctx.accounts.inscription;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    let token_account_escrow = &ctx.accounts.token_account_escrow;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let escrow = &ctx.accounts.escrow;

    token_deployment.number_of_tokens_issued += 1;

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

                root: mint.to_account_info(),
                /// since root in this case can sign (we are creating a brand new mint),
                /// it will sign
                signer: mint.to_account_info(),
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

    let data_bytes = token_deployment.mint_template.clone().into_bytes();

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
        this the solana way where we meet brc 20 type thinking:

        As we create a 'mint' op inscription, we also mint a corresponding amount of
        spl tokens into an escrow account held by a PDA.

        This ensures that any SPL-20 account is always convertible into a corresponding
        amount of traditional SPL token AND vice versa.
    */

    mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            MintTo {
            mint: fungible_mint.to_account_info(),
            to: token_account_escrow.to_account_info(),
            authority: escrow.to_account_info(),
        }),
        token_deployment.limit_per_mint,
    )?;

    Ok(())
}
