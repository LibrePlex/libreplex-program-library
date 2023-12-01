use crate::{Inscription, InscriptionV3, Migrator};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MigrateToV3<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in logic
    pub root: UncheckedAccount<'info>,

    #[account(init,
    payer=payer,
    space=72,
    seeds=[
        "migrator".as_bytes(),
        root.key().as_ref()
    ],
    bump)]
    pub migrator: Account<'info, Migrator>,

    /// CHECK: validated in logic
    #[account(seeds=[
            "inscription".as_bytes(),
            root.key().as_ref()
        ],
        bump)]
    pub inscription: Account<'info, Inscription>,

    #[account(init,
        // keeping things safe for now
        space = InscriptionV3::get_new_size_for_init(&inscription),
        seeds=[
            "inscription_v3".as_bytes(),
            root.key().as_ref()
        ],
        bump,
        payer = payer)]
    pub inscription2: Account<'info, InscriptionV3>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MigrateToV3>) -> Result<()> {
    let inscription = &ctx.accounts.inscription;
    let inscription_v2 = &mut ctx.accounts.inscription2;

    let payer = &ctx.accounts.payer;
    let migrator = &mut ctx.accounts.migrator;
    migrator.migrator = payer.key();
    migrator.root = inscription.root.key();

    // inscription v2
    inscription_v2.authority = inscription.authority;
    inscription_v2.root = inscription.root;
    inscription_v2.inscription_data = inscription.inscription_data;

    inscription_v2.order = inscription.order;
    inscription_v2.size = inscription.size;

    inscription_v2.content_type = inscription.media_type.convert_to_string();
    inscription_v2.encoding = inscription.encoding_type.convert_to_string();

    inscription_v2.validation_hash = inscription.validation_hash.clone();

    Ok(())
}
