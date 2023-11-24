
use anchor_lang::{Accounts, AccountSerialize, prelude::*};
use anchor_spl::token::{Mint, Token};

use super::errors::*;

use super::state::{MultiMintComponent, InfinityMint};










// // validation struct for lock claim
// #[derive(Accounts)]
// #[instruction(
//     name: String
// )]
// pub struct FinalizeMultiMint<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,

//     /// CHECK: checked against destination_token_account PDA
//     pub authority: UncheckedAccount<'info>,

//     #[account(
//         mut,
//         has_one = owner,
//         seeds =[
//             b"multi_mint",
//             authority.key().as_ref(),
//             owner.key().as_ref(),
//             // &amount.to_le_bytes(),
//             name.as_bytes()
//         ], bump)]
//     pub multi_mint: Account<'info, InfinityMultiMint>,

//     pub mint: Account<'info, Mint>,

//     pub system_program: Program<'info, System>,
// }




