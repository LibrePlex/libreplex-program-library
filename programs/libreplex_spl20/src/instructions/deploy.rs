use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV2,
    cpi::accounts::MakeInscriptionImmutable,
    cpi::accounts::ResizeInscription,
    cpi::accounts::WriteToInscription,
    instructions::{SignerType, WriteToInscriptionInput}, InscriptionSummary, Inscription, InscriptionV3,
};

use crate::{TokenDeployment, TICKER_LIMIT, errors::Spl20Error, ROOT_TYPE_LIMIT};


pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}   


#[derive(Accounts)]
#[instruction(input: DeployInput)]
pub struct DeployCtx<'info>  {
    #[account(init, payer = payer, space = 8 + TokenDeployment::INIT_SPACE, 
        seeds = ["spl20".as_ref(), input.ticker.as_ref()], bump)]
    pub token_deployment: Account<'info, TokenDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = input.decimals,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = escrow,
    )]
    pub token_account_escrow: Account<'info, TokenAccount>,


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

    /// CHECK: Passed in via CPI
    #[account(mut)]
    pub inscription_ranks_current_page: UncheckedAccount<'info>,

    /// CHECK: Passed in via CPI
    #[account(mut)]
    pub inscription_ranks_next_page: UncheckedAccount<'info>,

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
    sysvar_instructions: UncheckedAccount<'info>

}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DeployInput {
    pub creator: Pubkey,
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub collection_mint: Pubkey,
    pub ticker: String,
    pub image_: String,
    pub root_type: String,
    pub decimals: u8,
    pub deployment_template: String,
    pub mint_template: String,
}

pub fn deploy(ctx: Context<DeployCtx>, input: DeployInput) -> Result<()> {
    let deployment = &mut ctx.accounts.token_deployment;

    if input.ticker.len() > TICKER_LIMIT {
        return Err(Spl20Error::TickerTooLong.into());
    }

    if input.root_type.len() > ROOT_TYPE_LIMIT {
        return Err(Spl20Error::RootTypeTooLong.into())
    }
    
    // create 
    deployment.creator = input.creator;
    deployment.limit_per_mint = input.limit_per_mint;
    deployment.max_number_of_tokens = input.max_number_of_tokens;
    deployment.collection_mint = input.collection_mint;
    deployment.ticker = input.ticker;
    deployment.root_type = input.root_type;
    deployment.number_of_tokens_issued = 0;
    deployment.deployment_template = input.deployment_template;
    deployment.mint_template = input.mint_template;

    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &ctx.accounts.inscription_summary;
    let mint = &ctx.accounts.mint;
    let inscription = &ctx.accounts.inscription;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    
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
    
    let data_bytes = deployment.deployment_template.clone().into_bytes();

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
    


    Ok(())
}