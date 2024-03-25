use anchor_lang::{context::CpiContext, AccountDeserialize, Key, ToAccountInfo};
use anchor_spl::token::TokenAccount;
use solana_program::account_info::AccountInfo;



pub fn create_and_verify_ata<'a>(
    payer: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    ata: &AccountInfo<'a>,
    associated_token_program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>
) -> TokenAccount{
    let expected_non_fungible_token_account =
        anchor_spl::associated_token::get_associated_token_address(&owner.key(), &mint.key());
    if expected_non_fungible_token_account != ata.key() {
        panic!("Invalid token account");
    }
    if ata.to_account_info().data_is_empty() {
        // msg!("{}",payer.key() );
        anchor_spl::associated_token::create(CpiContext::new(
            associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: payer.to_account_info(),
                associated_token: ata.to_account_info(),
                authority: owner.to_account_info(),
                mint: mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        )).unwrap();
    }

    // deserialize
    let mut ata_data: &[u8] = &ata.try_borrow_data().unwrap();

    anchor_spl::token::TokenAccount::try_deserialize(&mut ata_data).unwrap()
}
