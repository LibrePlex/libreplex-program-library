use anchor_lang::{prelude::Result, Key}; // , solana_program::{program::invoke, system_instruction}
use anchor_lang::{prelude::Context, prelude::Pubkey}; // , solana_program::{program::invoke, system_instruction}
use anchor_spl::token::{TokenAccount, Mint, Token}; // Transfer


use mpl_token_metadata::{
    instruction::{
        create_master_edition_v3,
    },
    state:: {
        Metadata, TokenMetadataAccount, Creator
    },
    state::{MAX_NAME_LENGTH, MAX_URI_LENGTH},
};


use super::validators::{
    // MintFromMultiMint,
    // ActivateMultiMint,
    // AddMintComponent,
    // FinalizeMultiMint,
    // CreateMultiMint
};

// use super::atomic::{mint_to_wallet};

// use super::errors::{ClaimError};

// pub fn infinity_multimint_mint_logic(ctx: Context<MintFromMultiMint>, 
//     name: String,
//     multi_mint_bump: u8,
//     _multi_mint_component_bump: u8
// ) -> Result<()> {



//     let multimint_expected_bump = *ctx.bumps.get("multi_mint").ok_or(
//         ClaimError::MissingMultiMintBump
//     )?;

//     if multimint_expected_bump != multi_mint_bump {
//         return Err(ClaimError::InvalidMultiMintBump.into());
//     }



//     let multi_mint = &mut ctx.accounts.multi_mint;
//     let mint_component = &mut ctx.accounts.mint_component;

//     if multi_mint.state != 1 {
//         return Err(ClaimError::ClaimStateIsNotActive.into());
//     }


//     // check that there is enough left to claim

//     // 1 ) in the multi mint itself
//     if multi_mint.max_mints_available != 0 && multi_mint.minted + 1 > multi_mint.max_mints_available {
//         return Err(ClaimError::InsufficientAmountLeft.into());
//     }




//     // 2) in the individual mint
//     if mint_component.max_mints_available != 0  && mint_component.minted + 1 > mint_component.max_mints_available {
//         return Err(ClaimError::InsufficientAmountLeftInComponent.into());
//     }



//     let mut creators: Vec<Creator> = vec![];

//     if multi_mint.creator_1_address.is_some() && multi_mint.creator_1_share.is_some() {
//         creators.push(Creator {
//             address: multi_mint.creator_1_address.unwrap(),
//             share: multi_mint.creator_1_share.unwrap(),
//             verified: false
//         })
//     }

//     if multi_mint.creator_2_address.is_some() && multi_mint.creator_2_share.is_some() {
//         creators.push(Creator {
//             address: multi_mint.creator_2_address.unwrap(),
//             share: multi_mint.creator_2_share.unwrap(),
//             verified: false
//         })
//     }

//     if multi_mint.creator_3_address.is_some() && multi_mint.creator_3_share.is_some() {
//         creators.push(Creator {
//             address: multi_mint.creator_3_address.unwrap(),
//             share: multi_mint.creator_3_share.unwrap(),
//             verified: false
//         })
//     }

//     if multi_mint.creator_4_address.is_some() && multi_mint.creator_4_share.is_some() {
//         creators.push(Creator {
//             address: multi_mint.creator_4_address.unwrap(),
//             share: multi_mint.creator_4_share.unwrap(),
//             verified: false
//         })
//     }




//     // mint_to_wallet(name,
//     //     multi_mint_bump,
//     //     _multi_mint_component_bump,
//     //     ctx.accounts.authority.key,
//     //     ctx.accounts.owner.key,
//     //     ctx.accounts.metadata.key,
//     //     &ctx.accounts.mint.as_key(),
//     //     ctx.accounts.mint_authority.key,
//     //     ctx.accounts.payer.key,
//     //     ctx.accounts.token_metadata_program.key,
//     //     ctx.accounts.token_program.key,
//     //     ctx.accounts.system_program.key,
//     //     ctx.accounts.rent.key,
//     //     ctx.accounts.multi_mint_creator.key,
//     //     creators,
//     //     ctx.accounts.multi_mint_creator.key,
//     //     ctx.accounts.mint_component.to_account_info().name,
//     //     ctx.accounts.mint_component.to_account_info().symbol,
//     //     ctx.accounts.mint_component.to_account_info().off_chain_url,
//     //     ctx.accounts.mint_component.to_account_info().seller_fee_basis_points,
//     // );


//     multi_mint.minted += 1; // initial_claim_amount; // we start off with amount_claimed as 1. when amount reaches zero, the claim has been exhausted
//     mint_component.minted += 1;

//     Ok(())
// }








// pub fn create_infinity_multimint_logic(ctx: Context<CreateMultiMint>, 
//     name: String,
//     max_mints_available: u64,
//     multimint_bump: u8) -> Result<()>{

//     let multimint_expected_bump = *ctx.bumps.get("multi_mint").ok_or(
//         ClaimError::MissingMultiMintBump
//     )?;

//     if multimint_expected_bump != multimint_bump {
//         return Err(ClaimError::InvalidMultiMintBump.into());
//     }
//     let multi_mint = &mut ctx.accounts.multi_mint;
//     multi_mint.owner = *ctx.accounts.owner.key;
//     multi_mint.state = 0;  // starts in active state
//     multi_mint.authority = *ctx.accounts.authority.key;
//     multi_mint.name = name;
//     multi_mint.bump = multimint_bump;
//     multi_mint.max_mints_available = max_mints_available;
//     multi_mint.minted = 0;
//     Ok(())
// }






// pub fn infinity_multimint_finalize_logic(ctx: Context<FinalizeMultiMint>, 
//     _name: String
// )  -> Result<()> {

//     let multi_mint = &mut ctx.accounts.multi_mint;

//     if multi_mint.state > 1 {
//         return Err(ClaimError::ClaimStateIsNotActive.into());
//     }

//     multi_mint.state = 2; 
//     Ok(())
// }






