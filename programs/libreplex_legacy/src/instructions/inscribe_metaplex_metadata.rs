use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use libreplex_inscriptions::{
    cpi::accounts::CreateInscription, program::LibreplexInscriptions, instructions::SignerType,
};
use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};

use crate::{legacy_inscription::LegacyInscription, LegacyInscriptionErrorCode};


/* 
    This signer is needed when holder inscribes their mints.
    In that case, we need a second signer to validate the
    inscription content.

    For update authority we don't care so much. It's their 
    collection and they can put whatever junk they want in the 
    inscription.

    However we do want to prevent scenarios where the holder
    buys a turbo-rug for 0.01 SOL and then uploads a mad lad 
    skull as an inscription in the hopes of flogging it off
    to the highest bidder.
*/
pub mod content_validator_signer {
    use anchor_lang::declare_id;
    declare_id!("S1GNkYN3NZxyKVZfaTecXbrb8tA8yDMEUYFxd9yuW22");
}


#[derive(Clone, AnchorDeserialize, AnchorSerialize, PartialEq, Copy)]
pub enum AuthorityType {
    /*
        Holder-created inscription. Update authority holder
        can not touch the inscription. However they can 
        remove the mint from their collection and airdrop
        a new mint with inscription to the holder in case 
        they want to have a collection-wide inscription
        owned by the update authority.

        For mutable inscriptions, holder can resize / update 
        if the underlying offchain image changes. holder can 
        also close the inscription and reclaim the rent.

        For immutable inscriptions, nothing can be done to it. 
        Rent from ommutable inscriptions CANNOT BE RECLAIMED.
     */
    Holder,

    /*
        Update-authority created inscription. If it is immutable,
        it is forever. If it is mutable, the update authority can
        resize / update the inscription.

        Holder cannot create a new inscription of a mint that
        already has an update authority inscription on it.

        For immutable inscriptions, nothing can be done to it. 
        Rent from ommutable inscriptions CANNOT BE RECLAIMED.
     */
    UpdateAuthority,
}

// Adds a metadata to a group
#[derive(Accounts)]
#[instruction(authority_type: AuthorityType, hash: String)]
pub struct InscribeLegacyMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Can be any wallet
    pub owner: UncheckedAccount<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_data: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_summary: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_ranks_current_page: UncheckedAccount<'info>,

    /// CHECK: Checked via a CPI call
    #[account(mut)]
    pub inscription_ranks_next_page: UncheckedAccount<'info>,

    #[account(init,
        payer = authority,
        space = LegacyInscription::SIZE,
        seeds=[
            &(authority_type as u32).to_le_bytes(),
            "legacy_inscription".as_bytes(),
            mint.key().as_ref()
        ], bump)]
    pub legacy_inscription: Account<'info, LegacyInscription>,

    /// CHECK: Checked in logic
    #[account()]
    pub legacy_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in logic
    #[account()]
    pub legacy_metadata: UncheckedAccount<'info>,

    /// CHECK: The token program
    #[account(
        address = anchor_spl::token::ID
    )]
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(ctx: Context<InscribeLegacyMetadata>, authority_type: AuthorityType) -> Result<()> {
    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let inscription_summary = &mut ctx.accounts.inscription_summary;

    let inscription = &mut ctx.accounts.inscription;
    let inscription_data = &mut ctx.accounts.inscription_data;
    let system_program = &ctx.accounts.system_program;
    let authority = &ctx.accounts.authority;
    let mint = &ctx.accounts.mint;
    let legacy_inscription = &ctx.accounts.legacy_inscription;
    let legacy_mint = &ctx.accounts.legacy_mint;

    let inscription_ranks_current_page = &ctx.accounts.inscription_ranks_current_page;
    let inscription_ranks_next_page = &ctx.accounts.inscription_ranks_next_page;
    let metaplex_metadata = &ctx.accounts.legacy_metadata;

    let auth_key = ctx.accounts.authority.key();
    // make sure we are dealing with the correct metadata object.
    // this is to ensure that the mint in question is in fact a legacy
    // metadata object

    let expected_bump = ctx.bumps["legacy_inscription"];
    let mint_key = mint.key();
    check_permissions(metaplex_metadata, mint, authority_type, auth_key)?;
    let inscription_auth_seeds: &[&[u8]] = &[
        &(authority_type as u32).to_le_bytes(),
        mint_key.as_ref(),
        &[expected_bump],
    ];
    libreplex_inscriptions::cpi::create_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscription {
                /* the inscription root is set to metaplex
                 inscription object.
                */
                inscription_summary: inscription_summary.to_account_info(),

                root: mint.to_account_info(),
                /// since root in this case can not sign,
                /// this legacy inscription must be the signer
                /// this is ok as the inscriptions guarantee uniqueness
                /// per mint.
                signer: legacy_mint.to_account_info(),
                inscription: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: authority.to_account_info(),
                inscription_data: inscription_data.to_account_info(),
                inscription_ranks_current_page: inscription_ranks_current_page.to_account_info(),
                inscription_ranks_next_page: inscription_ranks_next_page.to_account_info(),
            },
            &[inscription_auth_seeds],
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInput {
            /*  set authority equal to the metaplex_inscription
            so that this program can later update it and
             make it immutable as long as the current holder
             of the metadata authorises it

             This delegation is needed to ensure that
             any authority associated with the inscription
             travels with the mint.

             the authority can (as of today) do any of the following
             1) resize the inscription
             2) upload data to the inscription (co-authorised by global signer
                    to ensure integrity of what's written )
             3) make the inscription immutable (and get a rank)
            */
            authority: Some(legacy_inscription.key()), // this includes update auth / holder, hence
            max_data_length: 0,
            current_rank_page: 0,
            signer_type: SignerType::LegacyMetadataSigner,
        },
    )?;

    Ok(())
}

fn check_permissions(metaplex_metadata: &UncheckedAccount<'_>, mint: &Account<Mint>, authority_type: AuthorityType, auth_key: Pubkey) 
    -> Result<()> {
    let mai = metaplex_metadata.to_account_info().clone();
    let data: &[u8] = &mai.try_borrow_data()?[..];
    let metadata_obj = Metadata::deserialize(&mut data.clone())?;
    if metadata_obj.mint != mint.key() {
        return Err(LegacyInscriptionErrorCode::BadMint.into());
    }
    if (authority_type == AuthorityType::UpdateAuthority
        && metadata_obj.update_authority != auth_key)
        || 
        // if authority == Holder, anybody can sign this. Because of the second signature
        // i.e. the content validator, we know that the request is bona fide and a hash 
        // will be set. only the holder can upload / resize at this point, but the upload
        // has to conform to the hash.
        (authority_type == AuthorityType::Holder
            && auth_key != content_validator_signer::id())
    {
        return Err(LegacyInscriptionErrorCode::BadAuthority.into());
    }
    match metadata_obj.token_standard {
        Some(x) => match &x {
            TokenStandard::Fungible => {
                return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
            }
            TokenStandard::FungibleAsset => {
                return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
            }
            _ => {}
        },
        None => {
            return Err(LegacyInscriptionErrorCode::CannotInscribeFungible.into());
        }
    }
   
    Ok(())
}
