use crate::state::{Metadata};
use crate::{ CreateMetadataInput, PermissionType, MetadataEvent, MetadataEventType, Asset};
use anchor_lang::{prelude::*, system_program};
use libreplex_inscriptions::instructions::CreateInscriptionInput;
use libreplex_inscriptions::program::Inscriptions;


use libreplex_inscriptions::cpi::accounts::{CreateInscription};

/* 
    we need a separate method since we want to 
    1) create ordinal and the metadata together (this requires metadata to sign)
    2) have metadata as the ordinal target
    3) have metadata asset type = Ordinal with account_id pointing to the ordinal

    (two-way link ensures that the mapping is 1-1)
*/ 

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateOrdinalMetadataInput {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub inscription_input: CreateInscriptionInput,
    pub update_authority: Pubkey,
}

impl CreateOrdinalMetadataInput {
    pub fn get_size(&self) -> usize {
        let size = 
            4 + self.name.len()
            + 4
            + self.symbol.len()
            + 4
            + self.inscription_input.get_size() as usize
            + 2 + 32 // for ordinal asset type
            + match &self.description {
                None =>0,
                Some(x) => 4 + x.len()
            };

        return size;
    }
}


#[derive(Accounts)]
#[instruction(metadata_input: CreateOrdinalMetadataInput)]
pub struct CreateOrdinalMetadata<'info> {
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
    pub mint: Signer<'info>,

    // ordinal must sign otherwise 
    pub ordinal: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub inscriptions_program: Program<'info, Inscriptions>

}

pub fn handler(ctx: Context<CreateOrdinalMetadata>, metadata_input: CreateOrdinalMetadataInput) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let ordinal = &mut ctx.accounts.ordinal;
    let authority = &ctx.accounts.signer;

    let inscriptions_program = &ctx.accounts.inscriptions_program;
    let system_program = &ctx.accounts.system_program;
    let signer = &ctx.accounts.signer;

    let mint_key = &ctx.accounts.mint.key();

    let metadata_seeds: &[&[u8]] = &[
        b"metadata",
        mint_key.as_ref(),
        &[ctx.bumps["metadata"]],
    ];
    
    libreplex_inscriptions::cpi::create_inscription(
        CpiContext::new_with_signer(
            inscriptions_program.to_account_info(),
            CreateInscription {
                // raffle is the owner of the pod
                root: metadata.to_account_info(),
                ordinal: ordinal.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: signer.to_account_info()
            },
            &[&metadata_seeds]
        ),
        metadata_input.inscription_input
    )?;

    // Update the metadata state account
    metadata.mint = ctx.accounts.mint.key();
    metadata.is_mutable = true;
    metadata.symbol = metadata_input.symbol.clone();
    metadata.name = metadata_input.name.clone();
    metadata.update_authority = metadata_input.update_authority;
    metadata.description = metadata_input.description;
    metadata.asset = Asset::Inscription {
            account_id: ctx.accounts.ordinal.key()
    };

   
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