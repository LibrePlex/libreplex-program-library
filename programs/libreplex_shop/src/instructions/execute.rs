use crate::{
    constants::{LISTING},
    state::{Listing, Price},
};
use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token},
};
use libreplex_metadata::{Group, Metadata, RoyaltyShare};
use libreplex_shared::{transfer_tokens, SharedError};



use spl_token_2022::{ID as TOKEN_2022_PROGRAM_ID, instruction::close_account};

struct RoyaltyAmount {
    amount: u64,
    recipient: Pubkey,
}

#[event]
pub struct ExecuteEvent {
    pub id: Pubkey,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    /// CHECK: checked against listing.lister in macro
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

   /// CHECK: Checked against ID constraint
   #[account(
        constraint = mint.owner.eq(&TOKEN_2022_PROGRAM_ID)
    )]
    pub mint: UncheckedAccount<'info>,


    #[account()]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account()]
    pub group: Option<Box<Account<'info, Group>>>,

    #[account(mut,
        close=seller,
        constraint = listing.lister == seller.key()
        )]
    pub listing: Box<Account<'info, Listing>>,

    pub buyer: Signer<'info>,

    /// CHECK: Checked in logic
    #[account(mut)]
    pub escrow_token_account: UncheckedAccount<'info>,

    /// CHECK: Is allowed to be empty in which case we create it
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    #[account(mut)]
    pub lister_payment_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    #[account(mut)]
    pub buyer_payment_token_account: UncheckedAccount<'info>,

    /// CHECK: Ignored for native price
    pub payment_mint: Option<UncheckedAccount<'info>>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,
    
    /// CHECK: Checked against ID constraint
    #[account(
        constraint = token_program_2022.key.eq(&TOKEN_2022_PROGRAM_ID)
    )]
    pub token_program_2022: UncheckedAccount<'info>,
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
    let token_program_2022 = &ctx.accounts.token_program_2022;
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

    // let's handle both 2022 and trad here just in case.
    let mint_token_program = match escrow_token_account.owner {
        &TOKEN_2022_PROGRAM_ID => token_program_2022.to_account_info(),
        _ => token_program.to_account_info()
    };
    msg!("amount: {}", listing.amount);
    // transfer listing to the buyer
    transfer_tokens(
        &mint_token_program,
        &escrow_token_account.to_account_info(),
        &recipient_token_account.to_account_info(),
        &listing.to_account_info(),
        &mint.to_account_info(),
        &buyer_account_info,
        &associated_token_program.to_account_info(),
        &system_program.to_account_info(),
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

            match payment_mint {
                None => {
                    return Err(SharedError::BadMint.into());
                }
                Some(x) => {
                    if mint != x.key() {
                        return Err(SharedError::BadMint.into());
                    }

                    let total_royalty_amount = (amount as u128)
                        .checked_mul(royalty_bps as u128)
                        .unwrap()
                        .checked_div(10000)
                        .unwrap() as u64;

                    let royalty_amounts =
                        calculate_royalty_amounts(total_royalty_amount, amount, shares)?;

                    // like above, let's handle both trad and 2022 for royalties as well
                    let payment_mint_token_program = match buyer_payment_token_account.owner {
                        &TOKEN_2022_PROGRAM_ID => token_program_2022.to_account_info(),
                        _ => token_program.to_account_info()
                    };


                    for royalty_amount in royalty_amounts {
                        let royalty_wallet = remaining_accounts.pop().unwrap();
                        let royalty_token_account = remaining_accounts.pop().unwrap();

                        transfer_tokens(
                            &payment_mint_token_program,
                            &buyer_payment_token_account.to_account_info(),
                            &royalty_token_account.to_account_info(),
                            &buyer_account_info.to_account_info(),
                            &x.to_account_info(),
                            &royalty_wallet.to_account_info(),
                            associated_token_program,
                            system_program,
                            None,
                            &buyer_account_info,
                            royalty_amount.amount,
                        )?;
                    }

                    transfer_tokens(
                        &payment_mint_token_program,
                        &buyer_payment_token_account.to_account_info(),
                        &lister_payment_token_account.to_account_info(),
                        &buyer_account_info.to_account_info(),
                        &x.to_account_info(),
                        &seller.to_account_info(),
                        associated_token_program,
                        system_program,
                        None,
                        &buyer_account_info,
                        amount - total_royalty_amount,
                    )?;
                }
            }
        }
    }

    solana_program::program::invoke_signed(
        &close_account(&token_program_2022.key(), 
            &escrow_token_account.key(), 
            &listing.lister, 
            &listing.key(), 
            &[])?
            ,
        &[
            escrow_token_account.to_account_info().clone(),
            seller.to_account_info().clone(),
            listing.to_account_info().clone(),
            token_program_2022.to_account_info().clone(),
        ],
        &[auth_seeds],
    )?;

    
    emit!(ExecuteEvent {
        id: listing.key(),
    });


    Ok(())
}

fn calculate_royalty_amounts(
    total_royalty_amount: u64,
    _lamports: u64,
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
