use anchor_lang::{prelude::*, system_program};
use libreplex_shared::{create_token_2022_and_metadata, MintAccounts2022, TokenGroupInput};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_metadata_interface::state::TokenMetadata;

use crate::{Editions, NAME_LIMIT, OFFCHAIN_URL_LIMIT, SYMBOL_LIMIT};



#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub symbol: String,
    pub name: String,
    pub offchain_url: String, // used both for the fungible and the non-fungible
    pub creator_cosign_program_id: Option<Pubkey>,
}


#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct InitialiseCtx<'info>  {
    #[account(init, payer = payer, space = 8 + Editions::INIT_SPACE, 
        seeds = ["editions_deployment".as_ref(), input.symbol.as_ref()], bump)]
    pub editions_deployment: Account<'info, Editions>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // can be different from payer for PDA integration
    #[account(mut)]
    pub creator: Signer<'info>,


    #[account(mut)]
    pub group_mint: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: address checked
    #[account(address = spl_token_2022::ID)]
    pub token_program: AccountInfo<'info>,
}


pub fn initialise(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
   
    if input.offchain_url.len() > OFFCHAIN_URL_LIMIT {
        panic!("Offchain url too long");
    }
    if input.symbol.len() > SYMBOL_LIMIT {
        panic!("Symbol too long");
    }
    if input.name.len() > NAME_LIMIT {
        panic!("Name too long");
    }

    let group_mint = &ctx.accounts.group_mint;

    ctx.accounts.editions_deployment.set_inner(Editions {
        creator: ctx.accounts.creator.key(),
        max_number_of_tokens: input.max_number_of_tokens,
        number_of_tokens_issued: 0,
        group_mint: group_mint.key(),
        cosigner_program_id: match input.creator_cosign_program_id {
            Some(x) => x,
            _ => system_program::ID
        },
        symbol: input.symbol,
        name: input.name,
        offchain_url: input.offchain_url,
        padding: [0; 100],
    });


    let editions_deployment = &ctx.accounts.editions_deployment;
    let payer = &ctx.accounts.payer;
    let group_mint = &ctx.accounts.group_mint;
    let token_program = &ctx.accounts.token_program;


    let update_authority =
        OptionalNonZeroPubkey::try_from(Some(editions_deployment.key())).expect("Bad update auth");

        let deployment_seeds: &[&[u8]] = &[
            "editions_deployment".as_bytes(),
            editions_deployment.symbol.as_ref(),
            &[ctx.bumps.editions_deployment],
        ];


    // msg!("Create token 2022 w/ metadata");
    create_token_2022_and_metadata(
        MintAccounts2022 {
            authority: editions_deployment.to_account_info(),
            payer: payer.to_account_info(),
            nft_owner: editions_deployment.to_account_info(),
            nft_mint: group_mint.to_account_info(),
            spl_token_program: token_program.to_account_info(),
        },
        0,
        Some(TokenMetadata {
            name: editions_deployment.name.clone(),
            symbol: editions_deployment.symbol.clone(),
            uri: editions_deployment.offchain_url.clone(),
            update_authority,
            mint: group_mint.key(),
            additional_metadata: vec![],
        }),
        Some(TokenGroupInput {
            max_size: match editions_deployment.max_number_of_tokens  {
                0 => u32::MAX,
                _ => editions_deployment.max_number_of_tokens as u32
            },
        }),
        None,
        Some(deployment_seeds),
        0
    )?;
    

    Ok(())

}
