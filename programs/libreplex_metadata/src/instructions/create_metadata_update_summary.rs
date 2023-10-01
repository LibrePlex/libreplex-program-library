use crate::state::Metadata;
use crate::MetadataSummary;
use crate::CreateMetadataInput;
use anchor_lang::prelude::*;
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;

use super::handle_create_metadata;

// whitelisted signer programs

pub mod migrator_lite {
    use super::*;
    declare_id!("migr1m1An7f3X75nKuuUn9mm3844miK62ZohpqRfQHp");
}

#[derive(Accounts)]
#[instruction(metadata_input: CreateMetadataInput)]
pub struct CreateMetadataUpdateSummary<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, seeds = [b"metadata", mint.key().as_ref()],
              bump, payer = payer, space = Metadata::BASE_SIZE + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(init_if_needed, seeds = [b"metadata_summary"],
              bump, payer = payer, space = MetadataSummary::BASE_SIZE)]
    pub metadata_summary: Box<Account<'info, MetadataSummary>>,

    /// CHECK: Checked against ID constraint
    #[account(
        constraint = mint.owner.eq(&TOKEN_2022_PROGRAM_ID)
    )]
    pub mint: UncheckedAccount<'info>,

    /*
        Authority needs to be a mint or a PDA generated by a whitelisted migrator program
    */
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    /*
     only to be supplied if the migration is invoked by a whitelisted
     migrator program.

     if a migrator program is invoked, then the signer account must be
     a PDA derived by the migrator program from seed [mint].
    */
    pub invoked_migrator_program: Option<UncheckedAccount<'info>>,
}

pub fn handler(
    ctx: Context<CreateMetadataUpdateSummary>,
    metadata_input: CreateMetadataInput,
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let mint_info = &mut ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let invoked_migrator_program = &ctx.accounts.invoked_migrator_program;
    let payer = &mut ctx.accounts.payer;

    let metadata_summary = &mut ctx.accounts.metadata_summary;

    let clock = Clock::get()?;
    metadata_summary.mint_count_total += 1;
    metadata_summary.last_mint = mint_info.key();
    metadata_summary.last_minter = payer.key();
    metadata_summary.last_mint_time = clock.unix_timestamp;

    handle_create_metadata(
        mint_info,
        authority,
        invoked_migrator_program,
        metadata,
        metadata_input,
    )
}
