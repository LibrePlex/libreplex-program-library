use crate::state::Metadata;
use crate::{Asset, MetadataEvent, MetadataEventType, MetadataExtension};
use anchor_lang::prelude::*;

use libreplex_inscriptions::cpi::accounts::CreateInscription;

use libreplex_inscriptions::program::LibreplexInscriptions;

/*
    we need a separate method since we want to
    1) create ordinal and the metadata together (this requires metadata to sign)
    2) have metadata as the ordinal target
    3) have metadata asset type = Ordinal with account_id pointing to the ordinal

    (two-way link ensures that the mapping is 1-1)
*/

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateMetadataInscriptionInput {
    pub name: String,
    pub symbol: String,
    pub update_authority: Pubkey,
    pub extension: MetadataExtension,
    pub description: Option<String>,
    pub data_type: String,
}

impl CreateMetadataInscriptionInput {
    pub fn get_size(&self) -> usize {
        let size =
            4 + self.name.len() +
            4 + self.symbol.len() +
            // inscription asset
            Asset::BASE_SIZE + 
            32 +
            4 + self.data_type.len() +
            1 + match &self.description {
                Some(x) => 4 + x.len(),
                None => 0,
            } + self.extension.get_size();

        return size;
    }
}

#[derive(Accounts)]
#[instruction(metadata_input: CreateMetadataInscriptionInput)]
pub struct CreateInscriptionMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"metadata", mint.key().as_ref()],
              bump, payer = signer, space = Metadata::BASE_SIZE + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    /*
        Signer constraint to be relaxed later
        to allow for migration signatures etc.

        Currently this signer does not need to be a mint,
        but you can tag metadata onto anything.A
    */
    #[account(mut)]
    pub mint: Signer<'info>,

    // ordinal must sign otherwise
    #[account(mut)]
    pub inscription: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, LibreplexInscriptions>,
}

pub fn handler(
    ctx: Context<CreateInscriptionMetadata>,
    metadata_input: CreateMetadataInscriptionInput
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let inscription = &mut ctx.accounts.inscription;

    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let system_program = &ctx.accounts.system_program;
    let signer = &ctx.accounts.signer;

    let mint_key = &ctx.accounts.mint.key();

    let metadata_seeds: &[&[u8]] = &[b"metadata", mint_key.as_ref(), &[ctx.bumps["metadata"]]];

    libreplex_inscriptions::cpi::create_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscription {
                // raffle is the owner of the pod
                root: metadata.to_account_info(),
                ordinal: inscription.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: signer.to_account_info(),
            },
            &[&metadata_seeds],
        ),
        libreplex_inscriptions::instructions::CreateInscriptionInput {
            authority: Some(signer.key()),
            max_data_length: 0,
        },
    )?;

    // Update the metadata state account
    metadata.mint = ctx.accounts.mint.key();
    metadata.is_mutable = true;
    metadata.symbol = metadata_input.symbol.clone();
    metadata.name = metadata_input.name.clone();
    metadata.update_authority = metadata_input.update_authority;
    metadata.asset = Asset::Inscription {
        account_id: ctx.accounts.inscription.key(),
        data_type: metadata_input.data_type,
        description: metadata_input.description,
    };
    metadata.creator = signer.key();
    metadata.extension = metadata_input.extension;

    msg!(
        "metadata created for mint with pubkey {}",
        ctx.accounts.mint.key()
    );

    emit!(MetadataEvent {
        id: metadata.key(),
        mint: ctx.accounts.mint.key(),
        event_type: MetadataEventType::Create
    });

    Ok(())
}
