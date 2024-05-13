use anchor_lang::{prelude::*, system_program};
use libreplex_editions::{
    cpi::accounts::InitialiseCtx, program::LibreplexEditions, group_extension_program, InitialiseInput
};

use crate::EditionsControls;


#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseControlInput {
    pub max_mints_per_wallet: u64,
    pub treasury: Pubkey,

    // for core editions

    pub max_number_of_tokens: u64, // this is the max *number* of tokens
    pub symbol: String,
     // add curlies if you want this to be created dynamically. For example
    // hippo #{} -> turns into hippo #0, hippo #1, etc
    // without curlies the url is the same for all mints 
    pub name: String,
    // add curlies if you want this to be created dynamically. For example
    // ipfs://pippo/{} -> turns into ipfs://pippo/1, ipfs://pippo/2, etc
    // without curlies the url is the same for all mints 
    pub offchain_url: String,
    pub cosigner_program_id: Option<Pubkey>,
}


#[derive(Accounts)]
#[instruction(_initialise_controls_input: InitialiseControlInput)]
pub struct InitialiseEditionControlsCtx<'info> {
   
   
    #[account(init,
        space = EditionsControls::INITIAL_SIZE,
        payer = payer,
        seeds = [
            b"editions_controls", editions_deployment.key().as_ref()
            ],
        bump
    )]
    pub editions_controls: Account<'info, EditionsControls>,

    /// CHECK: CPI: Passed into libreplex_editions program for initialisation. Checking seed here for early warning
    #[account(mut)]
    pub editions_deployment: UncheckedAccount<'info>,

    /// CHECK: Checked in via CPI
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: can be different from payer for PDA integration
    #[account()]
    pub creator: UncheckedAccount<'info>,

    /// CHECK: created
    #[account(mut)]
    pub group_mint: Signer<'info>,

    /// CHECK: created
    #[account(mut)]
    pub group: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,

    /// CHECK: address checked
    #[account(address = spl_token_2022::ID)]
    pub token_program: AccountInfo<'info>,

     /// CHECK: address checked
     #[account(address = group_extension_program::ID)]
     pub group_extension_program: AccountInfo<'info>,

    pub libreplex_editions_program: Program<'info, LibreplexEditions>
}

pub fn initialise_editions_controls(ctx: Context<InitialiseEditionControlsCtx>, input: InitialiseControlInput) -> Result<()> {
    
    let libreplex_editions_program = &ctx.accounts.libreplex_editions_program;
    let editions_controls = &mut ctx.accounts.editions_controls;
    
    let editions_deployment = &ctx.accounts.editions_deployment;
    let hashlist = &ctx.accounts.hashlist;
    let payer = &ctx.accounts.payer;
    let creator = &ctx.accounts.creator;

    let group = &ctx.accounts.group;
    let group_mint = &ctx.accounts.group_mint;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let group_extension_program = &ctx.accounts.group_extension_program;
    

    

    let core_input = InitialiseInput {
        max_number_of_tokens: input.max_number_of_tokens,
        symbol: input.symbol,
        name: input.name,
        offchain_url: input.offchain_url,
        creator_cosign_program_id: Some(crate::ID),
    };


    libreplex_editions::cpi::initialise(
    
        CpiContext::new(
            libreplex_editions_program.to_account_info(),
            InitialiseCtx {
                editions_deployment: editions_deployment.to_account_info(),
                hashlist: hashlist.to_account_info(),
                payer: payer.to_account_info(),
                creator: editions_controls.to_account_info(),
                group: group.to_account_info(),
                group_mint: group_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                group_extension_program: group_extension_program.to_account_info(),
                
            },
        ),
        core_input,
    )?;

    editions_controls.set_inner(EditionsControls { 
        editions_deployment: editions_deployment.key(), 
        creator: creator.key(), 
        max_mints_per_wallet: input.max_mints_per_wallet,
        padding: [0; 200], 
        cosigner_program_id: match input.cosigner_program_id {
            Some(x)=>x,
            None => system_program::ID
        },
        phases: vec![],
        treasury: input.treasury, 
    });



    Ok(())
}
