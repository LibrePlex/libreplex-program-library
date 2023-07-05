use crate::{
    constants::{ESCROW_WALLET, LISTING},
    state::{Listing, Price},
};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use libreplex_metadata::{Group, Metadata, RoyaltyShare};
use libreplex_shared::{transfer_tokens, SharedError};

struct RoyaltyAmount {
    amount: u64,
    recipient: Pubkey,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    /// CHECK: checked against listing.lister in macro
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account()]
    pub metadata: Account<'info, Metadata>,

    #[account()]
    pub group: Option<Account<'info, Group>>,

    #[account(mut,
        close=seller,
        constraint = listing.lister == seller.key()
        )]
    pub listing: Account<'info, Listing>,

    pub buyer: Signer<'info>,

    #[account(mut,
        close=seller,
        constraint = escrow_token_account.mint == listing.mint,
        constraint = escrow_token_account.owner == listing.key(),
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    /// CHECK: Is allowed to be empty in which case we create it
    pub buyer_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    pub lister_payment_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    pub buyer_payment_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    pub payment_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Execute<'info>>) -> Result<()> {
    let seller = &ctx.accounts.seller;
    let escrow_token_account = &ctx.accounts.escrow_token_account;
    let recipient_token_account = &ctx.accounts.buyer_token_account;
    let mint = &ctx.accounts.mint;
    let listing = &ctx.accounts.listing;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let group = &ctx.accounts.group;
    let payment_mint = &ctx.accounts.payment_mint;
    let metadata = &ctx.accounts.metadata;
    let buyer_account_info = &ctx.accounts.buyer.to_account_info().clone();
    let seller_account_info = &ctx.accounts.seller.to_account_info().clone();

    let mut remaining_accounts = ctx.remaining_accounts.to_vec();

    let mint_key = &mint.key();
    let auth_seeds = &[
        LISTING.as_bytes(),
        &mint_key.as_ref(),
        &[listing.listing_bump],
    ];

    match &metadata.group {
        Some(g) => match group {
            Some(g2) => {
                if !g.eq(&g2.key()) {
                    return Err(SharedError::GroupMismatch.into());
                }
            }
            None => {
                return Err(SharedError::GroupAccountMissing.into());
            }
        },
        None => {
            // metadata does not belong to a group. No need to do anything else
        }
    }

    // transfer listing to the buyer
    transfer_tokens(
        &token_program.to_account_info(),
        &escrow_token_account.to_account_info(),
        &recipient_token_account.to_account_info(),
        &listing.to_account_info(),
        &mint.to_account_info(),
        &buyer_account_info,
        &associated_token_program.to_account_info(),
        &associated_token_program.to_account_info(),
        Some(&[auth_seeds]),
        &buyer_account_info,
        listing.amount,
    )?;

    let mut shares: Vec<RoyaltyShare> = vec![];

    let royalty_bps = match group {
        Some(x) => match &x.royalties {
            Some(royalties) => {
                for share in &royalties.shares {
                    shares.push(share.clone());
                }

                royalties.bps as u16
            }
            None => 0 as u16,
        },
        None => 0 as u16,
    };

    let buyer_account_info = &buyer_account_info.clone();

    match listing.price {
        Price::Native { lamports } => {
            let total_royalty_amount = (lamports as u128)
                .checked_mul(royalty_bps as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap() as u64;

            let royalty_amounts =
                calculate_royalty_amounts(total_royalty_amount, lamports, shares)?;

            for royalty_amount in royalty_amounts {
                anchor_lang::solana_program::program::invoke(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        &buyer_account_info.key(),
                        &royalty_amount.recipient.key(),
                        royalty_amount.amount,
                    ),
                    &[
                        buyer_account_info.clone(),
                        remaining_accounts.pop().unwrap().to_account_info(),
                    ],
                )?;
            }
            // main payment net of royalties
            anchor_lang::solana_program::program::invoke(
                &anchor_lang::solana_program::system_instruction::transfer(
                    &buyer_account_info.key(),
                    &seller.key(),
                    lamports - total_royalty_amount as u64,
                ),
                &[buyer_account_info.clone(), seller_account_info.clone()],
            )?;
        }
        Price::Spl { mint, amount } => {
            let buyer_payment_token_account = &ctx.accounts.buyer_payment_token_account;
            let lister_payment_token_account = &ctx.accounts.lister_payment_token_account;

            if mint != payment_mint.key() {
                return Err(SharedError::BadMint.into());
            }

            let total_royalty_amount = (amount as u128)
                .checked_mul(royalty_bps as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap() as u64;

            let royalty_amounts =
                calculate_royalty_amounts(total_royalty_amount, amount, shares)?;

            for royalty_amount in royalty_amounts {
             
                let royalty_wallet = remaining_accounts.pop().unwrap();
                let royalty_token_account = remaining_accounts.pop().unwrap();

                transfer_tokens(
                    &token_program.to_account_info(),
                    &buyer_payment_token_account.to_account_info(),
                    &royalty_token_account.to_account_info(),
                    &buyer_account_info.to_account_info(),
                    &payment_mint.to_account_info(),
                    &royalty_wallet.to_account_info(),
                    associated_token_program,
                    system_program,
                    None,
                    token_program,
                    royalty_amount.amount,
                )?;
            }

            transfer_tokens(
                &token_program.to_account_info(),
                &buyer_payment_token_account.to_account_info(),
                &lister_payment_token_account.to_account_info(),
                &buyer_account_info.to_account_info(),
                &payment_mint.to_account_info(),
                &seller.to_account_info(),
                associated_token_program,
                system_program,
                None,
                token_program,
                amount - total_royalty_amount,
            )?;
        }
    }

    Ok(())
}

fn calculate_royalty_amounts(
    total_royalty_amount: u64,
    lamports: u64,
    shares: Vec<RoyaltyShare>,
) -> Result<Vec<RoyaltyAmount>> {
    let mut royalty_amounts: Vec<RoyaltyAmount> = Vec::new();

    if total_royalty_amount > 0 {
        for share in shares {
            let current_royalty_amount = total_royalty_amount
                .checked_mul(share.share as u64)
                .unwrap()
                .checked_div(10000 as u64)
                .unwrap() as u64;

            royalty_amounts.push(RoyaltyAmount {
                amount: current_royalty_amount,
                recipient: share.recipient.key(),
            });
        }
    }

    Ok(royalty_amounts)
}
