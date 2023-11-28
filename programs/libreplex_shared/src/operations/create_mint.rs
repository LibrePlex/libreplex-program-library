use anchor_lang::{prelude::*, solana_program};


use solana_program::program::invoke_signed;

/// Accounts to mint.
pub struct CreateMintAccounts<'info> {
    pub authority_pda: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub nft_owner: AccountInfo<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub nft_mint_authority: AccountInfo<'info>,
    pub nft_metadata: AccountInfo<'info>,
    pub nft_master_edition: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub spl_token_program: AccountInfo<'info>,
    pub spl_ata_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub sysvar_instructions: Option<AccountInfo<'info>>,
    pub rule_set: Option<AccountInfo<'info>>,
}

/// Creates the metadata accounts and mint a new token.
pub fn create_mint(
    accounts: CreateMintAccounts,
    authority_seeds: &[&[u8]],
    max_supply: u64,
    is_mutable: bool
) -> Result<()> {
    // create metadata accounts

    let sysvar_instructions_info = accounts
        .sysvar_instructions
        .as_ref().unwrap();
        
    let token_info = accounts
        .token_account
        .as_ref();
    
    let spl_ata_program_info = accounts.spl_ata_program.as_ref();



    // let mint_ix = mint_builder.amount(1).instruction();

    // invoke_signed(&mint_ix, &mint_infos, &[&authority_seeds])?;

    Ok(())
}
