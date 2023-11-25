use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};

use libreplex_inscriptions::{
    cpi::accounts::CreateInscriptionV2,
    cpi::accounts::MakeInscriptionImmutable,
    cpi::accounts::ResizeInscription,
    cpi::accounts::WriteToInscription,
    instructions::{SignerType, WriteToInscriptionInput}, InscriptionSummary,
};

use crate::{TokenDeployment, TICKER_LIMIT, errors::Src20Error};


pub mod sysvar_instructions_program {
    use anchor_lang::declare_id;
    declare_id!("Sysvar1nstructions1111111111111111111111111");
}   


#[derive(Accounts)]
#[instruction(input: DeployInput)]
pub struct DeployCtx<'info>  {
    #[account(init, payer = payer, space = 8 + TokenDeployment::INIT_SPACE, 
        seeds = ["deployment".as_ref(), input.ticker.as_ref()], bump)]
    pub deployment: Account<'info, TokenDeployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = input.decimals,
        mint::authority = deployment, // this is important
    )]
    pub fungible_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
    )]
    pub non_fungible_mint: Account<'info, Mint>,


    #[account(
        init,
        payer = payer,
        token::mint = fungible_mint,
        token::authority = deployment,
    )]
    pub fungible_escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub inscription_summary: Account<'info, InscriptionSummary>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_v3: UncheckedAccount<'info>,
    

      /// CHECK: passed in via CPI to libreplex_inscriptions program
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    #[account()]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: ID checked via constraint
    #[account(
        constraint = inscriptions_program.key() == libreplex_inscriptions::ID
    )]
    pub inscriptions_program: UncheckedAccount<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: ID checked via constraint
    #[account(
        constraint = sysvar_instructions.key() == sysvar_instructions_program::ID
    )]
    sysvar_instructions: UncheckedAccount<'info>

}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct DeployInput {
    pub limit_per_mint: u64, // this number of SPL tokens are issued into the escrow when an op: 'mint' comes in 
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub ticker: String,
    pub image_: String,
    pub decimals: u8,
    pub deployment_template: String,
    pub mint_template: String,
    pub offchain_url: String
}

pub fn deploy(ctx: Context<DeployCtx>, input: DeployInput) -> Result<()> {
    let deployment = &mut ctx.accounts.deployment;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &ctx.accounts.inscription_summary;
    let mint = &ctx.accounts.fungible_mint;
    let inscription = &ctx.accounts.inscription;
    let inscription_v3 = &ctx.accounts.inscription_v3;
    let inscription_data = &ctx.accounts.inscription_data;
    

    if input.ticker.len() > TICKER_LIMIT {
        return Err(Src20Error::TickerTooLong.into());
    }

    
    // create 
    deployment.creator = payer.key();
    deployment.limit_per_mint = input.limit_per_mint;
    deployment.max_number_of_tokens = input.max_number_of_tokens;
    deployment.ticker = input.ticker;
    deployment.number_of_tokens_issued = 0;
    deployment.deployment_template = input.deployment_template;
    deployment.mint_template = input.mint_template;


    
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
            // the authority here doesn't matter because we will make this immutable at the
            // end of the transaction
            authority: Some(payer.key()),
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

    // set update auth to 1111111111111111
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